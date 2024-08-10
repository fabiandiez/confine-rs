use toml::{Table, Value};

pub struct Config {
    base_table: Option<Table>,
    env_table: Option<Table>,
}

// pub trait TomlValueTrait {}
//
// pub enum TomlValue {
//     StringValue(StringValue),
//     IntegerValue(IntegerValue),
//     FloatValue(FloatValue),
//     BooleanValue(BooleanValue),
//     ArrayValue(ArrayValue),
//     TableValue(TableValue),
// }
//
// pub struct StringValue(String);
// pub struct IntegerValue(i64);
// pub struct FloatValue(f64);
// pub struct BooleanValue(bool);
// pub struct ArrayValue(Vec<Value>);
// pub struct TableValue(Table);
//
// impl TomlValueTrait for StringValue {}
// impl TomlValueTrait for IntegerValue {}
// impl TomlValueTrait for FloatValue {}
// impl TomlValueTrait for BooleanValue {}
// impl TomlValueTrait for ArrayValue {}
// impl TomlValueTrait for TableValue {}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("The configuration file was not found.")]
    NotFound,

    #[error("The configuration file was not valid TOML.")]
    InvalidToml,

    #[error("No value for key '{0}' was found.")]
    NoValueForKey(String),

    #[error("The value for key '{0}' is not the correct type.")]
    IncorrectTypeForKey(String),

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

    pub fn get(&self, key: &str) -> Result<&Value, ConfigError> {
        if self.env_table.is_some() {
            let env_table = self.env_table.as_ref().unwrap();
            let value = env_table.get(key);

            if let Some(v) = value {
                return Ok(v);
            }
        }

        let base_table = self.base_table.as_ref().unwrap();
        let value = base_table.get(key);

        if let Some(v) = value {
            return Ok(v);
        }

        Err(ConfigError::NoValueForKey(key.to_string()))
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
