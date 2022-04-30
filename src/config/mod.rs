use serde::{Deserialize, Serialize}; 
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub author: String,
    pub auth_token: String,
    pub email: String,
    pub username: String,
}

pub fn get_config(path: &Path) -> Config {
    if let Ok(data_file) = std::fs::read_to_string(path) {
        let config: Config = serde_json::from_str(&data_file).unwrap();

        Config {
            author: config.author,
            auth_token: config.auth_token,
            email: config.email,
            username: config.username,
        }
    } else {
        let empty_config = Config {
            author: "".to_string(),
            auth_token: "".to_string(),
            email: "".to_string(),
            username: "".to_string(),
        };

        //Safely create directory path and config.json
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap(); 
        std::fs::File::create(&path);

        let x = std::fs::write( path, serde_json::to_string_pretty(&empty_config).unwrap(), );
        match x { 
            Ok(c) => {
                std::fs::write( path, serde_json::to_string_pretty(&empty_config).unwrap(), );

                return empty_config;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        empty_config
    }
}
