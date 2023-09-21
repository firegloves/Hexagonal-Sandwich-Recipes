#[cfg(test)]
pub mod shared {
    use std::path::PathBuf;

    use actix_web::web::Data;

    use crate::{parse_local_config, Repository, SandwichMongoRepository};
    use crate::config::{Config, parse_config, PersistenceConfig};
    use crate::domain::sandwich::{Sandwich, SandwichType};
    use crate::driven::repository::FindSandwich;
    use crate::driving::rest_handler::sandwiches::SandwichResponse;
    use crate::tests::sandwich_repo_double::repo_doble::SandwichRepoDouble;

    pub const SANDWICH_ID: &str = "sand-id";
    pub const SANDWICH_NAME: &str = "Hot dog";
    pub const SANDWICH_TYPE: SandwichType = SandwichType::Meat;
    pub const CHEESEBURGER_NAME: &str = "Cheeseburger";

    //
    // ASSERTION HELPERS
    //

    pub fn assert_on_sandwich(expected: Sandwich, actual: &Sandwich, assert_on_id: bool) {
        if assert_on_id {
            assert_eq!(actual.id().value().as_ref().unwrap(), expected.id().value().as_ref().unwrap());
        }
        assert_eq!(actual.name().value(), expected.name().value());
        assert_on_ingredients(expected.ingredients().value(), actual.ingredients().value());
    }

    pub fn assert_on_ingredients(expected_ingredients: &Vec<String>, actual_ingredients: &Vec<String>) {
        assert_eq!(expected_ingredients.len(), actual_ingredients.len());

        for (i, exp_ingr) in expected_ingredients.iter().enumerate() {
            assert_eq!(exp_ingr, &actual_ingredients[i]);
        }
    }


    //
    // STUBBING HELPERS
    //

    pub fn stub_sandwich(with_id: bool) -> Sandwich {
        let sandwich_id = if with_id { SANDWICH_ID } else { "" };
        let sandwich_name = SANDWICH_NAME;

        let hot_dog = Sandwich::new(sandwich_id.to_string(),
                                    sandwich_name.to_string(),
                                    stub_ingredients(),
                                    SANDWICH_TYPE)
            .unwrap();

        hot_dog
    }

    pub fn stub_ingredients() -> Vec<String> {
        vec!["Wurst".to_string(), "Ketchup".to_string()]
    }


    pub fn stub_cheeseburger_ingredients() -> Vec<String> {
        vec!(String::from("ground meat"),
             String::from("cheese"),
             String::from("ketchup"),
             String::from("mayo"))
    }

    pub fn stub_cheeseburger() -> Sandwich {
        let ingredients = stub_cheeseburger_ingredients();

        Sandwich::new(String::from(""),
                      String::from(CHEESEBURGER_NAME),
                      ingredients.clone(),
                      SANDWICH_TYPE)
            .unwrap()
    }

    pub async fn create_repo_and_default_sandwich() -> Sandwich {
        let repo: SandwichMongoRepository = create_sandwich_repo();
        create_default_sandwich(&repo).await
    }

    pub async fn create_default_sandwich<'a, T: Repository<Sandwich>>(repo: &T) -> Sandwich {
        create_sandwich(repo, stub_sandwich(false)).await
    }

    pub async fn create_sandwich<'a, T: Repository<Sandwich>>(repo: &T, sandwich: Sandwich) -> Sandwich {
        repo.create(sandwich).await.unwrap()
    }

    pub async fn delete_sandwiches_from_list_response<'a, T: Repository<Sandwich>>(repo: &T, resp_vec: Vec<SandwichResponse>) {

        for sand_resp in resp_vec.iter() {
            delete_sandwich_from_sandwich_response(repo, &sand_resp).await;
        }
    }

    pub async fn delete_sandwich_from_sandwich_response<'a, T: Repository<Sandwich>>(repo: &T, resp: &SandwichResponse) {
        repo.delete(&resp.id).await.unwrap();
    }

    pub fn create_sandwich_repo() -> SandwichMongoRepository {
        let config = parse_local_config();
        <SandwichMongoRepository as Repository<Sandwich>>::new(&config.persistence).unwrap()
    }

    pub fn empty_find_sandwich() -> FindSandwich {
        FindSandwich {
            id: None,
            name: "".to_string(),
            ingredients: vec![],
        }
    }

    pub fn get_testing_persistence_config() -> PersistenceConfig {
        get_testing_config().persistence
    }

    pub fn get_testing_config() -> Config {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/config.toml");
        parse_config(d)
    }


    pub fn match_and_assert_on_sandwich<T>(res: Result<Sandwich, T>, expected: Sandwich) {
        match res {
            Ok(sandwich) => {
                assert!(sandwich.id().value().is_some());
                assert_on_sandwich(expected, &sandwich, false);
            },
            _ => unreachable!()
        };
    }

    pub fn double_repo_data() -> Data<SandwichRepoDouble> {
        let repo = SandwichRepoDouble::new(&get_testing_persistence_config()).unwrap();
        Data::new(repo)
    }
}
