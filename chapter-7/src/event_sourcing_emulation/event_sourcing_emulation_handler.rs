use actix_web::{HttpResponse, web};

use crate::{Repository, Sandwich};
use crate::driving::rest_handler::errors::ApiError;
use crate::event_sourcing_emulation::sync_databases::{sync_databases, SyncDbsError};

/// emulate_event_sourcing
pub async fn emulate_event_sourcing<T: Repository<Sandwich>, U: Repository<Sandwich>>(
    mongo_repository: web::Data<T>,
    sql_repository: web::Data<U>,
) -> Result<HttpResponse, ApiError> {

    let result = sync_databases(mongo_repository, sql_repository).await;

    result
        .map(|_| Ok(HttpResponse::Ok().finish()))
        .map_err(|e| match e {
            SyncDbsError::Unknown(m) => ApiError::Unknown(m),
        })?
}