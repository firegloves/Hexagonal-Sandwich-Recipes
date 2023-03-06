use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::middleware::Logger;

use crate::driving::rest_handler;

mod domain;
mod helpers;
mod tests;
mod driving;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    create_server().await.unwrap().await.expect("An error occurred while starting the web application");
}

async fn create_server() -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
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
                                .route(web::get().to(rest_handler::sandwiches::find_sandwiches))
                                .route(web::post().to(rest_handler::sandwiches::create_sandwich))
                                .route(web::put().to(rest_handler::sandwiches::update_sandwich))
                        ).service(
                        web::resource("sandwiches/{id}")
                            .route(web::get().to(rest_handler::sandwiches::get_by_id))
                            .route(web::delete().to(rest_handler::sandwiches::delete_one_sandwich))
                    )
                )
        );
}
