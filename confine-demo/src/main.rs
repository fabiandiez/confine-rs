#![allow(unused)]

use confine::confine;
use confine::ConfineConfigBuilder;
use serde::Deserialize;

fn main() {
    // Using the builder
    let config = ConfineConfigBuilder::default()
        .config_path("confine-demo/config".into())
        .env_var("ENVIRONMENT".into())
        .prefix("confine-demo".into())
        .try_load::<MyConfig>()
        .unwrap();
    
    // Using the macro
    let config = MyConfig::try_load().unwrap();

    println!("My int: {}", config.my_int);
    println!("My string: {}", config.my_string);
    println!("My bool: {}", config.my_bool);
}

#[derive(Deserialize)]
#[confine(env_var = "ENVIRONMENT", path = "confine-demo/config")]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
