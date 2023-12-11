use actix_web::web;

use crate::domain::does_sandwich_exist_by_name;
use crate::domain::sandwich::{Sandwich, SandwichType};
use crate::driven::repository::RepoCreateError;
use crate::Repository;

#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

// this is my port / use case
pub async fn create_sandwich<'a, T: Repository<Sandwich>>(repository: web::Data<T>, name: &'a str, ingredients: &'a Vec<&str>, sandwich_type: &SandwichType) -> Result<Sandwich, CreateError> {

    let ingredients = ingredients.iter().map(|item| item.to_string()).collect::<Vec<String>>();
    let sandwich = Sandwich::new(String::from(""), name.to_string(), ingredients, sandwich_type.clone(), 0)
        .map_err(|e| CreateError::InvalidData(e))?;

    if does_sandwich_exist_by_name(&repository, name).await {
        return Err(CreateError::Conflict(String::from("A sandwich with this name is already present")))
    }

    repository.create(sandwich).await
        .map_err(|e| return match e {
            RepoCreateError::InvalidData(e) => CreateError::InvalidData(format!("Invalid data: {}", e)),
            RepoCreateError::Unknown(e) => CreateError::Unknown(format!("Unknown error: {}", e)),
        })
}


#[cfg(test)]
mod tests {

    use actix_web::web::Data;
    use crate::helpers::string_vec_to_vec_str;
    use crate::tests::test_utils::shared::{stub_cheeseburger, get_testing_mongodb_config, match_and_assert_on_sandwich, SANDWICH_NAME, SANDWICH_STARS, SANDWICH_TYPE, stub_sandwich, stub_ingredients, assert_on_sandwich, SANDWICH_ID};
    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;

    use super::*;

    #[actix_rt::test]
    async fn should_create_a_sandwich() {

        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);

        let mut repo = SandwichRepoDouble::new(&get_testing_mongodb_config()).unwrap();
        repo.set_error(true);
        let s = create_sandwich(Data::new(repo), SANDWICH_NAME, &ingredients, &SANDWICH_TYPE).await.unwrap();

        assert_eq!(s.id().value().is_some(), true);
        assert_on_sandwich(stub_sandwich(false),&s, false);
    }

    #[actix_rt::test]
    async fn should_not_create_a_sandwich_if_conflicting() {

        let ingredients = stub_ingredients();
        let ingredients = string_vec_to_vec_str(&ingredients);

        let mut repo = SandwichRepoDouble::new(&get_testing_mongodb_config()).unwrap();
        repo.set_error(true);
        let s = create_sandwich(Data::new(repo), SANDWICH_NAME, &ingredients, &SANDWICH_TYPE).await.unwrap();

        assert_eq!(s.id().value().is_some(), true);
        assert_on_sandwich(stub_sandwich(false),&s, false);
    }
}
