#![allow(unused)]

use confine_macro::{confine};
use serde::Deserialize;

fn main() {
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
