mod app;
mod config;
mod requests;

use std::path::PathBuf;
//use home::home_dir;

fn main() { 
    
    let home_dir = home::home_dir().unwrap().as_path().display().to_string();
    let path_joined = [home_dir, ".anyfile/config.json".to_string()].join("/");
    let path = PathBuf::from(path_joined); 

    let config = config::get_config(&path); 
    //***************************************************** */
    // UI
    app::controller::run(config)
}
