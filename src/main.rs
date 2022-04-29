mod app;
mod config;
mod requests;

use std::path::PathBuf;

fn main() {
    //
    let path = PathBuf::from("./config.json");
    let config = config::get_config(&path);

     
    //let _x = requests::get_repo_list::get_repo_list(&config, "sandbox".to_string());

    let base64 = "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=".to_string();
    //let _x = requests::put_test::put_test(config.clone(), base64);

    //***************************************************** */
    // UI
    //***************************************************** */
    //
    app::controller::run(config);
}
