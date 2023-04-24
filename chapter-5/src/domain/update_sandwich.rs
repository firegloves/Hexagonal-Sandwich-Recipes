use actix_web::web;

use crate::domain::does_sandwich_exist;
use crate::domain::sandwich::{Sandwich, SandwichType};
use crate::driven::repository::RepoUpdateError;
use crate::Repository;

#[derive(Debug)]
pub enum UpdateError {
    InvalidData(String),
    Unknown(String),
    NotFound,
    Conflict(String),
}

// this is my port / use case
pub async fn update_sandwich<'a, T: Repository<Sandwich>>(repository: web::Data<T>, id: &'a str, name: &'a str, ingredients: &'a Vec<&str>, sandwich_type: &SandwichType) -> Result<Sandwich, UpdateError> {
    if id.is_empty() {
        return Err(UpdateError::InvalidData(String::from("Cannot update without a target id")));
    }

    let ingredients = ingredients.iter().map(|item| item.to_string()).collect::<Vec<String>>();
    let sandwich = Sandwich::new(String::from(id), name.to_string(), ingredients, sandwich_type.clone())
        .map_err(|e| UpdateError::InvalidData(e))?;

    if ! does_sandwich_exist(&repository, id).await {
        return Err(UpdateError::Conflict(String::from("Cannot find the sandwich to update")));
    }

    repository.update(sandwich).await
        .map_err(|e| return match e {
            RepoUpdateError::InvalidData(e) => UpdateError::InvalidData(format!("Invalid data: {}", e)),
            RepoUpdateError::NotFound => UpdateError::NotFound,
            RepoUpdateError::Unknown(e) => UpdateError::Unknown(format!("Unknown error: {}", e)),
        })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use crate::helpers::string_vec_to_vec_str;
    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;
    use crate::tests::test_utils::shared::{assert_on_sandwich, get_testing_persistence_config, SANDWICH_ID, SANDWICH_NAME, SANDWICH_TYPE, stub_ingredients, stub_sandwich};

    use super::*;

    #[actix_rt::test]
    async fn should_update_an_existing_sandwich() {

        let ingrs = stub_ingredients();
        let ingrs = string_vec_to_vec_str(&ingrs);

        let repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let s = update_sandwich(Data::new(repo), SANDWICH_ID, SANDWICH_NAME, &ingrs, &SANDWICH_TYPE).await.unwrap();

        assert_on_sandwich(stub_sandwich(false), &s, false);
    }

    #[actix_rt::test]
    async fn should_not_update_a_non_existing_sandwich() {

        let ingrs = stub_ingredients();
        let ingrs = string_vec_to_vec_str(&ingrs);

        let mut repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();
        repo.set_error(true);

        let res = update_sandwich(Data::new(repo), SANDWICH_ID, SANDWICH_NAME, &ingrs, &SANDWICH_TYPE).await;

        match res {
            Err(_) => {},
            Ok(_) => unreachable!()
        }
    }
}
