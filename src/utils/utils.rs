use std::env;

pub fn get_env(key: &str) -> String {
    env::var(key).expect(&format!("{} not set", key))
}
