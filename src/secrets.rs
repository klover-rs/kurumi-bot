
use std::fs;


pub fn get_secret(key: &str) -> String {
    let contents = fs::read_to_string("Secrets.toml").unwrap();

    let data: toml::Value = contents.parse().unwrap();

    let secret = match data.get(key) {
        Some(secret) => match secret.as_str() {
            Some(secret_str) => secret_str,
             None => panic!("{} value is not a string", key),
        }
        None => panic!("{} key not found", key),
    };

    secret.to_string()
}
