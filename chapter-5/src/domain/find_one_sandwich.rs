use actix_web::web;

use crate::domain::sandwich::Sandwich;
use crate::driven::repository::{FindSandwich, RepoSelectError};
use crate::Repository;

#[derive(Debug)]
pub enum FindOneError {
    Unknown(String),
    NotFound,
}

// this is my port / use case
pub async fn find_one_sandwich<'a, T: Repository<Sandwich>>(repository: web::Data<T>, id: &'a str, name: &'a str, ingredients: &'a Vec<&str>) -> Result<Sandwich, FindOneError> {

    let ingredients = ingredients.iter().map(|item| item.to_string()).collect::<Vec<String>>();

    let s = FindSandwich {
        id: Some(String::from(id)),
        name: String::from(name),
        ingredients
    };

    repository.find_one(s).await
        .map_err(|e| return match e {
            RepoSelectError::Unknown(e) => FindOneError::Unknown(format!("Unknown error: {}", e)),
            RepoSelectError::NotFound => FindOneError::NotFound
        })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;
    use crate::tests::test_utils::shared::{assert_on_sandwich, get_testing_persistence_config, SANDWICH_NAME, stub_sandwich};

    use super::*;

    #[actix_rt::test]
    async fn should_find_the_expected_sandwich() {

        let repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();

        match find_one_sandwich(Data::new(repo), "", SANDWICH_NAME, &vec![]).await {
            Ok(s) => assert_on_sandwich(s, &stub_sandwich(false), false),
            Err(_) => unreachable!()
        }
    }

    #[actix_rt::test]
    async fn should_not_find_the_expected_sandwich() {

        let mut repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();
        repo.set_error(true);

        match find_one_sandwich(Data::new(repo), "", SANDWICH_NAME, &vec![]).await {
            Err(_) => {},
            Ok(_) => unreachable!()
        }
    }
}
