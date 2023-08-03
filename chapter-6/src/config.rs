use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

const MONGODB_HOST: &str = "MONGODB_HOST";
const MONGODB_PORT: &str = "MONGODB_PORT";
const MONGODB_USER: &str = "MONGODB_USER";
const MONGODB_PWD: &str ="MONGODB_PWD";
const MONGODB_DB: &str = "MONGODB_DB";
const MONGODB_SCHEMA_COLLECTION: &str = "MONGODB_SCHEMA";
const AUTH_DB: &str = "AUTH_DB";

const MARIADB_HOST: &str = "MARIADB_HOST";
const MARIADB_PORT: &str = "MARIADB_PORT";
const MARIADB_USER: &str = "MARIADB_USER";
const MARIADB_PWD: &str ="MARIADB_PWD";
const MARIADB_DB: &str = "MARIADB_DB";

#[derive(Deserialize)]
pub struct Config {
    pub mongo_db: MongoDBConfig,
    pub maria_db: MariaDBConfig
}

#[derive(Deserialize, Clone)]
pub struct MongoDBConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub schema_collection: String,
    pub auth_db: String
}

#[derive(Deserialize, Clone)]
pub struct MariaDBConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl MongoDBConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.user.is_empty() {
            return Err("Empty MongoDB username".to_string());
        }

        if self.password.is_empty() {
            return Err("Empty MongoDB password".to_string());
        }

        if self.host.is_empty() {
            return Err("Empty MongoDB hostname".to_string());
        }

        if self.database.is_empty() {
            return Err("Empty MongoDB database".to_string());
        }

        if self.schema_collection.is_empty() {
            return Err("Empty MongoDB collection".to_string());
        }

        if self.auth_db.is_empty() {
            return Err("Empty MongoDB auth database".to_string());
        }

        Ok(())
    }
}

impl MariaDBConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.user.is_empty() {
            return Err("Empty MariaDB username".to_string());
        }

        if self.password.is_empty() {
            return Err("Empty MariaDB password".to_string());
        }

        if self.host.is_empty() {
            return Err("Empty MariaDB hostname".to_string());
        }

        if self.database.is_empty() {
            return Err("Empty MariaDB database".to_string());
        }

        Ok(())
    }
}

pub fn parse_local_config() -> Config {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("resources/config.toml");
    parse_config(d)
}

pub fn parse_config(path_buf: PathBuf) -> Config {
    let config = parse_config_from_file(path_buf);
    override_config_with_env_vars(config)
}

fn parse_config_from_file(path_buf: PathBuf) -> Config {
    let config_file = path_buf.into_os_string().into_string().unwrap();
    toml::from_str(read_to_string(config_file).unwrap().as_str()).unwrap()
}

fn override_config_with_env_vars(config: Config) -> Config {

    let mongo_db = config.mongo_db;
    let maria_db = config.maria_db;

    Config {
        mongo_db: MongoDBConfig {
            host: env::var(MONGODB_HOST).unwrap_or(mongo_db.host),
            port: env::var(MONGODB_PORT).map(|p| p.parse::<u16>().expect("Cannot parse the received MongoDB port")).ok().or(mongo_db.port),
            user: env::var(MONGODB_USER).unwrap_or(mongo_db.user),
            password: env::var(MONGODB_PWD).unwrap_or(mongo_db.password),
            database: env::var(MONGODB_DB).unwrap_or(mongo_db.database),
            schema_collection: env::var(MONGODB_SCHEMA_COLLECTION).unwrap_or(mongo_db.schema_collection),
            auth_db: env::var(AUTH_DB).unwrap_or(mongo_db.auth_db),
        },
        maria_db: MariaDBConfig {
            host: env::var(MARIADB_HOST).unwrap_or(maria_db.host),
            port: env::var(MARIADB_PORT).map(|p| p.parse::<u16>().expect("Cannot parse the received MariaDB port")).ok().or(maria_db.port),
            user: env::var(MARIADB_USER).unwrap_or(maria_db.user),
            password: env::var(MARIADB_PWD).unwrap_or(maria_db.password),
            database: env::var(MARIADB_DB).unwrap_or(maria_db.database)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::path::PathBuf;

    use serial_test::serial;

    use crate::config::parse_config;

    use super::*;

    #[test]
    #[serial]
    fn should_parse_a_config() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/tests/test_config.toml");
        let config = parse_config(d);
        let mongo_db = config.mongo_db;

        assert_eq!("localhost", mongo_db.host);
        assert_eq!(27017, mongo_db.port.unwrap());
        assert_eq!("root", mongo_db.user);
        assert_eq!("s4ndw1chr3c1p3RUS7", mongo_db.password);
        assert_eq!("sandwich-recipes", mongo_db.database);
        assert_eq!("test_recipes", mongo_db.schema_collection);
        assert_eq!("admin", mongo_db.auth_db);

        let maria_db = config.maria_db;

        assert_eq!("localhost", maria_db.host);
        assert_eq!(3306, maria_db.port.unwrap());
        assert_eq!("root", maria_db.user);
        assert_eq!("m4r14dbs4ndw1ch3s", maria_db.password);
        assert_eq!("sandwich-recipes", maria_db.database);
    }

    #[test]
    #[serial]
    fn should_override_a_parsed_config_with_env_vars() {

        env::set_var(MONGODB_HOST, "my_host");
        env::set_var(MONGODB_PORT, "1111");
        env::set_var(MONGODB_USER, "just_me");
        env::set_var(MONGODB_PWD, "what_a_pwd");
        env::set_var(MONGODB_DB, "my_db");
        env::set_var(MONGODB_SCHEMA_COLLECTION, "simple_schema");
        env::set_var(AUTH_DB, "auth_admin");

        env::set_var(MARIADB_HOST, "maria_host");
        env::set_var(MARIADB_PORT, "2222");
        env::set_var(MARIADB_USER, "just_you");
        env::set_var(MARIADB_PWD, "such_a_pwd");
        env::set_var(MARIADB_DB, "your_db");

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/tests/test_config.toml");
        let config = parse_config(d);
        let mongo_db = config.mongoDb;

        assert_eq!("my_host", mongo_db.host);
        assert_eq!(1111, mongo_db.port.unwrap());
        assert_eq!("just_me", mongo_db.user);
        assert_eq!("what_a_pwd", mongo_db.password);
        assert_eq!("my_db", mongo_db.database);
        assert_eq!("simple_schema", mongo_db.schema_collection);
        assert_eq!("auth_admin", mongo_db.auth_db);

        let maria_db = config.mongoDb;

        assert_eq!("maria_host", maria_db.host);
        assert_eq!(2222, maria_db.port.unwrap());
        assert_eq!("just_you", maria_db.user);
        assert_eq!("such_a_pwd", maria_db.password);
        assert_eq!("your_db", maria_db.database);

        // reset env vars
        env::remove_var(MONGODB_HOST);
        env::remove_var(MONGODB_PORT);
        env::remove_var(MONGODB_USER);
        env::remove_var(MONGODB_PWD);
        env::remove_var(MONGODB_DB);
        env::remove_var(MONGODB_SCHEMA_COLLECTION);
        env::remove_var(AUTH_DB);

        env::remove_var(MARIADB_HOST);
        env::remove_var(MARIADB_PORT);
        env::remove_var(MARIADB_USER);
        env::remove_var(MARIADB_PWD);
        env::remove_var(MARIADB_DB);
    }
}
