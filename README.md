# confine-rs
[![Crates.io](https://img.shields.io/crates/v/confine.svg)](https://crates.io/crates/confine)

This crate provides both a macro and a builder API for loading configuration files.
I would recommend using the macro for most use cases, as it provides a more concise way of loading configuration files.
Using either one, you will be provided with a `try_load` function for your configuration struct.
This will instantiate your struct with the configuration values from the following sources in order of precedence:

1. `config/application.toml`
2. `config/application-{environment}.toml`
3. Environment variables

The environment is determined by the `CONFINE_ENV` environment variable, which _must_ be set in order to load the environment-specific configuration file.

You can configure most of these settings, but the general structure is always enforced.
Things you can configure include:
- The path to the configuration files (default: `config`)
- The prefix of the configuration files (default: `application`)
- The environment variable that determines the environment (default: `CONFINE_ENV`)
## Usage

### Default configuration
#### Macro
```rust
use confine::confine;

fn main() {
    let config = MyConfig::try_load().unwrap();
}

#[derive(Deserialize)]
#[confine]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
```

#### Builder
```rust
use confine::ConfineConfigBuilder;
use serde::Deserialize;

fn main() {
    let config = ConfineConfigBuilder::default()
        .try_load::<MyConfig>()
        .unwrap();
}

#[derive(Deserialize)]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
```

### Custom configuration
#### Macro
```rust
use confine::confine;

fn main() {
    let config = MyConfig::try_load().unwrap();
}

#[derive(Deserialize)]
#[confine(
    path = "my_config",
    prefix = "my_prefix",
    env_var = "MY_ENV"
)]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
```

#### Builder
```rust
use confine::ConfineConfigBuilder;
use serde::Deserialize;

fn main() {
    let config = ConfineConfigBuilder::default()
        .path("my_config")
        .prefix("my_prefix")
        .env_var("MY_ENV")
        .try_load::<MyConfig>()
        .unwrap();
}

#[derive(Deserialize)]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
```

## Motivation
I found myself writing the same boilerplate code for loading configuration files in Rust over and over again.
Also, while `config-rs` is a great library, it allows for a lot of flexibility in how _exactly_ you handle configuration.
Especially when working on multiple projects, I prefer to have a consistent way of storing and loading configuration files.
This crate aims to provide an easy-to-use macro for loading configuration files with sensible defaults.
