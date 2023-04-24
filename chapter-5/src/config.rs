use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

const PERSISTENCE_HOST: &str = "PERSISTENCE_HOST";
const PERSISTENCE_PORT: &str = "PERSISTENCE_PORT";
const PERSISTENCE_USER: &str = "PERSISTENCE_USER";
const PERSISTENCE_PWD: &str ="PERSISTENCE_PWD";
const PERSISTENCE_DB: &str = "PERSISTENCE_DB";
const PERSISTENCE_SCHEMA_COLLECTION: &str = "PERSISTENCE_SCHEMA";
const AUTH_DB: &str = "AUTH_DB";

#[derive(Deserialize)]
pub struct Config {
    pub persistence: PersistenceConfig
}

#[derive(Deserialize, Clone)]
pub struct PersistenceConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub password: String,
    pub database: String,
    pub schema_collection: String,
    pub auth_db: String
}

impl PersistenceConfig {
    pub fn validate(&self) -> Result<(), String> {
        if self.user.is_empty() {
            return Err("Empty username".to_string());
        }

        if self.password.is_empty() {
            return Err("Empty password".to_string());
        }

        if self.host.is_empty() {
            return Err("Empty hostname".to_string());
        }

        if self.database.is_empty() {
            return Err("Empty database".to_string());
        }

        if self.schema_collection.is_empty() {
            return Err("Empty collection".to_string());
        }

        if self.auth_db.is_empty() {
            return Err("Empty auth database".to_string());
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

    let pers = config.persistence;

    Config {
        persistence: PersistenceConfig {
            host: env::var(PERSISTENCE_HOST).unwrap_or(pers.host),
            port: env::var(PERSISTENCE_PORT).map(|p| p.parse::<u16>().expect("Cannot parse the received persistence port")).ok().or(pers.port),
            user: env::var(PERSISTENCE_USER).unwrap_or(pers.user),
            password: env::var(PERSISTENCE_PWD).unwrap_or(pers.password),
            database: env::var(PERSISTENCE_DB).unwrap_or(pers.database),
            schema_collection: env::var(PERSISTENCE_SCHEMA_COLLECTION).unwrap_or(pers.schema_collection),
            auth_db: env::var(AUTH_DB).unwrap_or(pers.auth_db),
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
        let pers = config.persistence;

        assert_eq!("localhost", pers.host);
        assert_eq!(27017, pers.port.unwrap());
        assert_eq!("root", pers.user);
        assert_eq!("s4ndw1chr3c1p3RUS7", pers.password);
        assert_eq!("sandwich-recipes", pers.database);
        assert_eq!("test_recipes", pers.schema_collection);
        assert_eq!("admin", pers.auth_db);
    }

    #[test]
    #[serial]
    fn should_override_a_parsed_config_with_env_vars() {

        env::set_var(PERSISTENCE_HOST, "my_host");
        env::set_var(PERSISTENCE_PORT, "1111");
        env::set_var(PERSISTENCE_USER, "just_me");
        env::set_var(PERSISTENCE_PWD, "what_a_pwd");
        env::set_var(PERSISTENCE_DB, "my_db");
        env::set_var(PERSISTENCE_SCHEMA_COLLECTION, "simple_schema");
        env::set_var(AUTH_DB, "auth_admin");

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/tests/test_config.toml");
        let config = parse_config(d);
        let pers = config.persistence;

        assert_eq!("my_host", pers.host);
        assert_eq!(1111, pers.port.unwrap());
        assert_eq!("just_me", pers.user);
        assert_eq!("what_a_pwd", pers.password);
        assert_eq!("my_db", pers.database);
        assert_eq!("simple_schema", pers.schema_collection);
        assert_eq!("auth_admin", pers.auth_db);

        // reset env vars
        env::remove_var(PERSISTENCE_HOST);
        env::remove_var(PERSISTENCE_PORT);
        env::remove_var(PERSISTENCE_USER);
        env::remove_var(PERSISTENCE_PWD);
        env::remove_var(PERSISTENCE_DB);
        env::remove_var(PERSISTENCE_SCHEMA_COLLECTION);
        env::remove_var(AUTH_DB);
    }
}
