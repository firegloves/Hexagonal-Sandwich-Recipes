use actix_web::web;
use rand::Rng;

use crate::domain::sandwich::Sandwich;
use crate::driven::repository::{RepoCreateError, RepoFindAllError};
use crate::helpers::empty_find_sandwich;
use crate::Repository;

#[derive(Debug)]
pub enum SyncDbsError {
    Unknown(String),
}

// this is my port / use case
pub async fn sync_databases<'a, T: Repository<Sandwich>, U: Repository<Sandwich>>(mongo_repository: web::Data<T>, sql_repository: web::Data<U>) -> Result<(), SyncDbsError> {

    // delete all from sql
    let sql_sands = sql_repository.find_all(empty_find_sandwich()).await
        .map_err(|_| SyncDbsError::Unknown("Error deleting all records from Sql DB".to_string()))?;
    for s in sql_sands {
        sql_repository.delete(s.id().value().as_ref().unwrap()).await
            .map_err(|_| SyncDbsError::Unknown("Error deleting all records from Sql DB".to_string()))?;
    }

    // fetch all from mongo
    let sandwiches = mongo_repository.find_all(empty_find_sandwich()).await
        .map_err(|e| return match e {
            RepoFindAllError::Unknown(e) => SyncDbsError::Unknown(e)
        })?;

    // insert all into sql
    for s in sandwiches {

        // emulate stars by randomization
        let mut rng = rand::thread_rng();
        let cloned = Sandwich::new(
            "".to_string(),
            s.name().value().clone(),
            s.ingredients().value().clone(),
            s.sandwich_type().clone(),
            rng.gen_range(0..250))
            .unwrap();

        sql_repository.create(cloned).await
            .map_err(|e| return match e {
                RepoCreateError::Unknown(e) | RepoCreateError::InvalidData(e) => SyncDbsError::Unknown(e)
            })?;
    }

    Ok(())
}