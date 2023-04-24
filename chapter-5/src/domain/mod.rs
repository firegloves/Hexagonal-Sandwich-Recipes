use actix_web::web;

use crate::{Repository, Sandwich};
use crate::driven::repository::FindSandwich;

pub mod sandwich;
pub mod create_sandwich;
pub mod delete_one_sandwich;
pub mod find_all_sandwiches;
pub mod find_one_sandwich;
pub mod update_sandwich;

pub trait Entity {}

async fn does_sandwich_exist_by_name<'a, T: Repository<Sandwich>>(repository: &web::Data<T>, name: &str) -> bool {

    let s = FindSandwich {
        id: None,
        name: String::from(name),
        ingredients: vec![]
    };

    repository.find_one(s).await.is_ok()
}

async fn does_sandwich_exist<'a, T: Repository<Sandwich>>(repository: &web::Data<T>, id: &str) -> bool {

    let s = FindSandwich {
        id: Some(String::from(id)),
        name: String::from(""),
        ingredients: vec![]
    };

    repository.find_one(s).await.is_ok()
}
