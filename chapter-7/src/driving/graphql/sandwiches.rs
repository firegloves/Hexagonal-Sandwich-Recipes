use actix_web::{HttpResponse, web};
use actix_web::http::Error;
use juniper::http::GraphQLRequest;

use crate::domain::sandwich::Sandwich;
use crate::driven::repository::Repository;
use crate::driving::graphql::schema::{Context, Schema};

/// find sandwich recipes graphql
pub async fn sandwiches_graph<T: Repository<Sandwich> + Send + Sync + 'static>(
    repository: web::Data<T>,
    schema: web::Data<Schema>,
    graph_request: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let ctx = Context {
        repository: repository.into_inner(),
    };

    let res = graph_request.execute(&schema, &ctx).await;

    Ok(HttpResponse::Ok().json(res))
}