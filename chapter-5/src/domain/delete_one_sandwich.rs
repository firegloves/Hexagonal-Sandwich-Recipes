use actix_web::web;

use crate::domain::sandwich::Sandwich;
use crate::driven::repository::RepoDeleteError;
use crate::Repository;

#[derive(Debug)]
pub enum DeleteOneError {
    InvalidData(String),
    Unknown(String),
    NotFound
}

// this is my port / use case
pub async fn delete_one_sandwich<'a, T: Repository<Sandwich>>(repository: web::Data<T>, id: &str) -> Result<(), DeleteOneError> {

    repository.delete(id).await
        .map_err(|e| return match e {
            RepoDeleteError::InvalidData(e) => DeleteOneError::InvalidData(format!("Invalid data: {}", e)),
            RepoDeleteError::Unknown(e) => DeleteOneError::Unknown(format!("Unknown error: {}", e)),
            RepoDeleteError::NotFound => DeleteOneError::NotFound
        })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;
    use crate::tests::test_utils::shared::{get_testing_persistence_config, SANDWICH_ID};

    use super::*;

    #[actix_rt::test]
    async fn should_delete_a_sandwich() {

        // GIVEN a repo
        let repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();

        // WHEN I delete the hot dog
        let deleted = delete_one_sandwich(Data::new(repo), SANDWICH_ID).await;

        // THEN Ok is returned
        assert_eq!(true, deleted.is_ok());
    }

    #[actix_rt::test]
    async fn should_not_delete_a_non_existing_sandwich() {

        // GIVEN a repo
        let mut repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();
        // and the repo returns an error (e.g. non existing entity)
        repo.set_error(true);

        // WHEN I delete the hot dog
        let deleted = delete_one_sandwich(Data::new(repo), SANDWICH_ID).await;

        // THEN Err is returned
        assert_eq!(true, deleted.is_err());
    }
}
