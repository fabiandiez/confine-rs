use config::Config;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ConfineBuilderError {
    #[error("Error with the config file.")]
    ConfigError(#[from] config::ConfigError),

    #[error("Environment variable not set correctly.")]
    EnvVarError(#[from] std::env::VarError),
}

pub struct ConfineConfigBuilder {
    config_path: PathBuf,
    env_var: String,
    prefix: String,
}

impl Default for ConfineConfigBuilder {
    fn default() -> Self {
        Self {
            config_path: "config".into(),
            env_var: "CONFINE_ENV".into(),
            prefix: "application".into(),
        }
    }
}

impl ConfineConfigBuilder {
    pub fn config_path(mut self, path: PathBuf) -> Self {
        self.config_path = path;
        self
    }

    pub fn env_var(mut self, env_var: String) -> Self {
        self.env_var = env_var;
        self
    }

    pub fn prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn try_load<'de, T>(self) -> Result<T, ConfineBuilderError>
    where
        T: Deserialize<'de>,
    {
        let local_config = self.config_path.join(format!("{}.toml", self.prefix));
        let env = std::env::var(&self.env_var)?;
        let env_config = self
            .config_path
            .join(format!("{}-{}.toml", self.prefix, env));

        let builder = Config::builder()
            .add_source(config::File::from(local_config))
            .add_source(config::File::from(env_config).required(false))
            .add_source(config::Environment::default())
            .build()?;
        Ok(builder.try_deserialize()?)
    }
}
