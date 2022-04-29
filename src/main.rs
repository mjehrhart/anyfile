mod app;
mod config;
mod requests;

use std::path::PathBuf;

fn main() {
    // Config file format in json 
    /* { 
        "author": "",           // Any name
        "email": "",            // Github email (maybe it can be any email address)
        "auth_token": "",       // Get this from github --> settings --> developer settings -> personal auth token
        "username": ""          // Github login name
    } */
    let path = PathBuf::from("./config.json");
    let config = config::get_config(&path);
 
    //***************************************************** */
    // UI  
    app::controller::run(config);
}
