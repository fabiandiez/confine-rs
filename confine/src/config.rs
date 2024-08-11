use proc_macro2::TokenStream;
use quote::quote;
use syn::Type;
use toml::{Table, Value};

pub struct Config {
    base_table: Option<Table>,
    env_table: Option<Table>,
}

pub enum ConfigValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("The configuration file was not found.")]
    NotFound,

    #[error("The configuration file was not valid TOML.")]
    InvalidToml,

    #[error("No value for key '{0}' was found.")]
    NoValueForKey(String),

    #[error("The value for key '{0}' is not a supported type ('{1}').")]
    IncorrectType(String, String),

    #[error("Not a single valid configuration file was found.")]
    NoValidConfig,
}

impl Config {
    pub fn new(
        prefix: String,
        path: String,
        environment: Option<String>,
    ) -> Result<Self, ConfigError> {
        let base_path = format!("{}/{}.toml", path, prefix);
        let base_table = Config::load_file(&base_path);

        if environment.is_some() {
            let env = environment.unwrap();
            let env_path = format!("{}/{}-{}.toml", path, prefix, env);
            let env_table = Config::load_file(&env_path);

            if base_table.is_err() && env_table.is_err() {
                return Err(ConfigError::NoValidConfig);
            }

            Ok(Config {
                base_table: base_table.ok(),
                env_table: env_table.ok(),
            })
        } else {
            if base_table.is_err() {
                return Err(ConfigError::NoValidConfig);
            }

            Ok(Config {
                base_table: base_table.ok(),
                env_table: None,
            })
        }
    }

    pub fn get(&self, key: &str, expected_type: &str) -> Result<ConfigValue, ConfigError> {
        let base_table = self.base_table.as_ref().unwrap();
        let mut value = base_table.get(key);

        if self.env_table.is_some() {
            let env_table = self.env_table.as_ref().unwrap();
            let env_value = env_table.get(key);

            if let Some(v) = env_value {
                value = Some(v);
            }
        }

        match value {
            None => Err(ConfigError::NoValueForKey(key.to_string())),
            Some(v) => {
                match (v, expected_type) {
                    (Value::String(v), "String") => { Ok(ConfigValue::String(v.to_string())) }
                    (Value::Integer(v), "i64") => Ok(ConfigValue::Int(*v)),
                    (Value::Float(v), "f64") => Ok(ConfigValue::Float(*v)),
                    (Value::Boolean(v), "bool") => Ok(ConfigValue::Bool(*v)),
                    _ => Err(ConfigError::IncorrectType(key.to_string(), expected_type.to_string())),
                }
            }
        }
    }

    fn load_file(config_path: &str) -> Result<Table, ConfigError> {
        let config = match std::fs::read_to_string(config_path) {
            Ok(v) => v,
            Err(_) => {
                return Err(ConfigError::NotFound);
            }
        };

        let config = match config.parse::<Table>() {
            Ok(v) => v,
            Err(_) => {
                return Err(ConfigError::InvalidToml);
            }
        };

        Ok(config)
    }
}
