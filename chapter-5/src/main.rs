extern crate core;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;

use crate::config::parse_local_config;
use crate::domain::sandwich::Sandwich;
use crate::driven::repository::mongo_repository::SandwichMongoRepository;
use crate::driven::repository::Repository;
use crate::driving::rest_handler;

mod domain;
mod driven;
mod config;
mod driving;
mod helpers;
mod tests;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let config = parse_local_config();
    let repo = SandwichMongoRepository::new(&config.persistence).unwrap();

    create_server(repo).await.unwrap().await.expect("An error occurred while starting the web application");
}

async fn create_server<T: Repository<Sandwich> + Send + Sync + 'static + Clone>(
repo: T,
) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(repo.clone()))
            .configure(routes)
    }).bind(("127.0.0.1", 8080))?
        .run();
    Ok(server)
}


fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/recipes")
                .service(
                    web::scope("/api/v1")
                        .service(
                            web::resource("sandwiches")
                                .route(web::get().to(rest_handler::sandwiches::find_sandwiches::<SandwichMongoRepository>))
                                .route(web::post().to(rest_handler::sandwiches::create_sandwich::<SandwichMongoRepository>))
                                .route(web::put().to(rest_handler::sandwiches::update_sandwich::<SandwichMongoRepository>))
                        ).service(
                        web::resource("sandwiches/{id}")
                            .route(web::get().to(rest_handler::sandwiches::get_by_id::<SandwichMongoRepository>))
                            .route(web::delete().to(rest_handler::sandwiches::delete_one_sandwich::<SandwichMongoRepository>))
                    )
                )
        );
}
