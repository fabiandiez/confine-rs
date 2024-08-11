#![allow(unused)]

use confine::confine;

fn main() {
    let config = MyConfig::load();

    println!("My int: {}", config.my_int);
    println!("My string: {}", config.my_string);
    println!("My bool: {}", config.my_bool);
}

#[confine(env_var = "ENVIRONMENT", path = "confine-demo/config", prefix = "application")]
struct MyConfig {
    pub my_int: i64,
    pub my_string: String,
    pub my_bool: bool,
}
