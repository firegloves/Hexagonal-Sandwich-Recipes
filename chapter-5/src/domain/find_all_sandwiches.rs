use actix_web::web;

use crate::domain::sandwich::Sandwich;
use crate::driven::repository::{FindSandwich, RepoFindAllError};
use crate::Repository;

#[derive(Debug)]
pub enum FindAllError {
    Unknown(String)
}

// this is my port / use case
pub async fn find_all_sandwiches<'a, T: Repository<Sandwich>>(repository: web::Data<T>, name: &'a str, ingredients: &'a Vec<&str>) -> Result<Vec<Sandwich>, FindAllError> {

    let ingredients = ingredients.iter().map(|item| item.to_string()).collect::<Vec<String>>();

    let s = FindSandwich {
        id: None,
        name: String::from(name),
        ingredients
    };

    repository.find_all(s).await
        .map_err(|e| return match e {
            RepoFindAllError::Unknown(s) => FindAllError::Unknown(s)
        })
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;

    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;
    use crate::tests::test_utils::shared::{assert_on_sandwich, get_testing_persistence_config, stub_cheeseburger, stub_sandwich};

    use super::*;

    #[actix_rt::test]
    async fn should_find_all_sandwiches() {

        let repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();

        let sand_list = find_all_sandwiches(Data::new(repo), "", &vec![]).await.unwrap();

        assert_eq!(sand_list.len(), 2);
        assert_on_sandwich(stub_sandwich(false),&sand_list[0], false);
        assert_on_sandwich(stub_cheeseburger(),&sand_list[1], false);
    }

    #[actix_rt::test]
    async fn should_not_find_all_sandwiches_while_the_repo_returns_error() {

        // GIVEN a repo
        let mut repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();
        // and the repo returns an error
        repo.set_error(true);

        // WHEN I fetch the sandwiches
        match find_all_sandwiches(Data::new(repo), "", &vec![]).await {
            // THEN Err is returned
            Err(_) => {},
            Ok(_) => unreachable!()
        }
    }
}
