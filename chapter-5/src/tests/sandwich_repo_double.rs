#[cfg(test)]
pub mod repo_doble {
    use std::cell::RefCell;

    use async_trait::async_trait;

    use crate::config::PersistenceConfig;
    use crate::domain::sandwich::Sandwich;
    use crate::driven::repository::{FindSandwich, RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, Repository, RepoUpdateError};
    use crate::tests::test_utils::shared::{SANDWICH_ID, stub_cheeseburger, stub_sandwich};

    struct Wrap(RefCell<bool>);

    unsafe impl Sync for Wrap {}

    pub struct SandwichRepoDouble {
        has_error: Wrap,
    }

    impl SandwichRepoDouble {
        pub fn set_error(&mut self, value: bool) {
            *self.has_error.0.borrow_mut() = value;
        }
    }

    #[async_trait]
    impl Repository<Sandwich> for SandwichRepoDouble {
        fn new(_config: &PersistenceConfig) -> Result<Self, String> where Self: Sized {
            Ok(SandwichRepoDouble {
                has_error: Wrap(RefCell::from(false)),
            })
        }

        async fn create(&self, sandwich: Sandwich) -> Result<Sandwich, RepoCreateError> {
            if self.has_error.0.take() {
                return Err(RepoCreateError::Unknown(String::from("Error occurred")));
            }

            let s = Sandwich::new(String::from(SANDWICH_ID),
                                  sandwich.name().value().clone(),
                                  sandwich.ingredients().value().clone(),
                                  sandwich.sandwich_type().clone())
                .unwrap();

            Ok(s)
        }

        async fn find_one(&self, sandwich: FindSandwich) -> Result<Sandwich, RepoSelectError> {
            if self.has_error.0.take() {
                return Err(RepoSelectError::Unknown(String::from("Error occurred")));
            }

            Ok(stub_sandwich(false))
        }

        async fn find_all(&self, _sandwich: FindSandwich) -> Result<Vec<Sandwich>, RepoFindAllError> {
            if self.has_error.0.take() {
                return Err(RepoFindAllError::Unknown(String::from("Error occurred")));
            }

            Ok(vec![stub_sandwich(true), stub_cheeseburger()])
        }

        async fn update(&self, sandwich: Sandwich) -> Result<Sandwich, RepoUpdateError> {
            if self.has_error.0.take() {
                return Err(RepoUpdateError::Unknown(String::from("Error occurred")));
            }

            Ok(sandwich.clone())
        }

        async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
            if self.has_error.0.take() {
                return Err(RepoDeleteError::Unknown(String::from("Error occurred")));
            }

            Ok(())
        }
    }
}
