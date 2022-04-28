mod app;
mod config;
mod requests;

use std::path::PathBuf;

fn main() {
    //
    let path = PathBuf::from("./config.json");
    let config = config::get_config(&path);


    //let _x = requests::get_repo_list::get_repo_list(&config, "sandbox".to_string());
    //***************************************************** */
    // UI
    //***************************************************** */
    //
    app::controller::run(config);
}
