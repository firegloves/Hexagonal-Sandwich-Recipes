use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::Entity;
use crate::domain::sandwich::Sandwich;

pub mod mongo_repository;
pub mod sql_repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindSandwich {
    pub id: Option<String>,
    pub name: String,
    pub ingredients: Vec<String>,
}

#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String)
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String)
}

#[async_trait]
pub trait Repository<T> where T: Entity {

    /// Insert the received entity in the persistence system
    async fn create(&self, sandwich: T) -> Result<T, RepoCreateError>;

    /// Find and return one single record from the persistence system
    async fn find_one(&self, sandwich: FindSandwich) -> Result<T, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<T>, RepoFindAllError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, sandwich: T) -> Result<T, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError>;
}

// TODO check if this refactor can be omitted to avoid reporting it
pub trait SandwichRepository: Repository<Sandwich> + Send + Sync {}

impl<T: Repository<Sandwich> + Send + Sync> SandwichRepository for T {}

