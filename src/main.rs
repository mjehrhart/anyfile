mod app;
mod config;
mod requests;

use std::path::PathBuf;

fn main() {
    //
    let path = PathBuf::from("./config.json");
    let config = config::get_config(&path);
 
    //***************************************************** */
    // UI  
    app::controller::run(config);
}
