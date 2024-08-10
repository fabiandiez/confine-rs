#![allow(unused)]

extern crate confine_rs;

use confine_rs::confine;

fn main() {
    let config = MyConfig {
        my_int: 42,
        my_string: "Hello, world!".to_string(),
        my_bool: true,
    };

    println!("My int: {}", config.my_int);
}

#[confine(env_var = "ENVIRONMENT", path = "config", prefix = "application")]
struct MyConfig {
    pub my_int: i32,
    pub my_string: String,
    pub my_bool: bool,
}
