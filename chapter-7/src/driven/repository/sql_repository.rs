use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{Error, FromRow, MySql, Pool, query_as};
use sqlx::mysql::MySqlPoolOptions;

use crate::config::MariaDBConfig;
use crate::domain::sandwich::{Sandwich, SandwichType};
use crate::driven::repository::{FindSandwich, RepoCreateError, RepoDeleteError, RepoFindAllError, RepoSelectError, Repository, RepoUpdateError};

const SANDWICH_TABLE: &str = "sandwich";
const SANDWICH_ID_FIELD: &str = "id";
const SANDWICH_NAME_FIELD: &str = "name";
const SANDWICH_INGREDIENTS_FIELD: &str = "ingredients";
const SANDWICH_STARS_FIELD: &str = "stars";

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SandwichSql {
    id: i64,
    name: String,
    ingredients: String,
    stars: i32,
}

impl From<Sandwich> for SandwichSql {
    fn from(sandwich: Sandwich) -> Self {
        let id = match sandwich.id().value() {
            Some(id) => id,
            None => ""
        };

        let ingredients_json = serde_json::to_string(sandwich.ingredients().value()).unwrap();

        let sand_sql = SandwichSql {
            id: id.parse::<i64>().unwrap(),
            name: sandwich.name().value().to_string(),
            ingredients: ingredients_json,
            stars: 0,
        };

        sand_sql
    }
}

impl TryInto<Sandwich> for SandwichSql {
    type Error = String;

    fn try_into(self) -> Result<Sandwich, Self::Error> {
        Sandwich::new(self.id.to_string(),
                      self.name,
                      serde_json::from_str(&self.ingredients).unwrap(),
                      SandwichType::Undefined,
                      self.stars)
    }
}


#[derive(Clone)]
pub struct SandwichSqlRepository {
    database: String,
    conn_uri: String,
}

impl SandwichSqlRepository {
    /// new constructor function
    pub fn new(config: &MariaDBConfig) -> Result<Self, String> where Self: Sized {
        config.validate()?;
        let config = config.clone();
        let conn_uri = create_connection_uri(&config);

        Ok(SandwichSqlRepository {
            database: config.database,
            conn_uri,
        })
    }

    async fn open_connection(&self) -> Result<Pool<MySql>, Error> {
        MySqlPoolOptions::new()
            .max_connections(5)     // FIXME
            .connect(self.conn_uri.as_str())
            .await
    }
}

#[async_trait]
impl Repository<Sandwich> for SandwichSqlRepository {

    async fn create(&self, sandwich: Sandwich) -> Result<Sandwich, RepoCreateError> {

        let poll = self.open_connection().await
            .map_err(|e| RepoCreateError::Unknown(e.to_string()))?;

        let query = format!("INSERT INTO {} ({}, {}, {}) VALUES (?, ?, ?)",
                            SANDWICH_TABLE, SANDWICH_NAME_FIELD, SANDWICH_INGREDIENTS_FIELD, SANDWICH_STARS_FIELD);

        let ingredients_json = to_string(&*sandwich.ingredients().value())
            .map_err(|e| RepoCreateError::Unknown(e.to_string()))?;

        let result = sqlx::query(&query)
            .bind(sandwich.name().value())
            .bind(ingredients_json)
            .bind(sandwich.stars().value())
            .execute(&poll)
            .await;

        match result {
            Ok(_) => Ok(sandwich),
            Err(e) => Err(RepoCreateError::Unknown(e.to_string())),
        }
    }

    async fn find_one(&self, _: FindSandwich) -> Result<Sandwich, RepoSelectError> {
        Err(RepoSelectError::Unknown("Not implemented".to_string()))
    }

    async fn find_all(&self, sandwich: FindSandwich) -> Result<Vec<Sandwich>, RepoFindAllError> {
        let pool = self.open_connection().await
            .map_err(|e| RepoFindAllError::Unknown(e.to_string()))?;

        let query = format!("SELECT * FROM {}", SANDWICH_TABLE);

        let result: Result<Vec<SandwichSql>, sqlx::Error> = query_as::<MySql, SandwichSql>(&query)
            .fetch_all(&pool)
            .await;

        match result {
            Ok(sandwich_sql_vec) => {
                let sandwiches: Vec<Sandwich> = sandwich_sql_vec
                    .into_iter()
                    .map(|sandwich_sql| sandwich_sql.try_into())
                    .filter_map(Result::ok)
                    .collect();

                Ok(sandwiches)
            }
            Err(e) => Err(RepoFindAllError::Unknown(e.to_string())),
        }
    }

    async fn update(&self, _: Sandwich) -> Result<Sandwich, RepoUpdateError> {
        Err(RepoUpdateError::Unknown("Not implemented".to_string()))
    }

    async fn delete(&self, id: &str) -> Result<(), RepoDeleteError> {
        let pool = self.open_connection().await
            .map_err(|e| RepoDeleteError::Unknown(e.to_string()))?;

        let query = format!("DELETE FROM {} WHERE {} = ?", SANDWICH_TABLE, SANDWICH_ID_FIELD);

        let result = sqlx::query(&query)
            .bind(id)
            .execute(&pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoDeleteError::Unknown(e.to_string())),
        }
    }
}


fn create_connection_uri(config: &MariaDBConfig) -> String {
    format!("mysql://{}:{}@{}/{}",
            config.user,
            config.password,
            match config.port {
                None => config.host.to_string(),
                Some(port) => config.host.clone()
                    + ":"
                    + port.to_string().as_str(),
            },
            config.database)
}