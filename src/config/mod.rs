use serde::{Deserialize, Serialize};
use std::path::{Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub author: String, 
    pub auth_token: String,
    pub email: String,
    pub username: String,
    pub tree: String,
}

pub fn get_config(path: &Path) -> Config {

    let data = std::fs::read_to_string(path).expect("Something happened with the file"); 
    let config: Config = serde_json::from_str(&data).unwrap();
 
    Config{
        author: config.author,
        auth_token: config.auth_token,
        email: config.email,
        username: config.username,
        tree: config.tree
    } 
}