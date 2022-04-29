use std::{path::{Path}, ffi::OsStr};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    pub name: String,
    pub sha: String,
    pub size: u64,
    pub path: String,
    pub download_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub content: Response,
}

// Put a file in repository
pub fn put_file_in_repo(
    config: crate::config::Config,
    repo: String,
    file_base64: String,
    file_name: &Path,
    ext: Option<&OsStr>,
) -> Result<Content, Box<dyn std::error::Error>> {

    // Step 1 : Build request url
    let name = Path::new(file_name).file_stem().unwrap(); 
    let url = format!("https://api.github.com/repos/{user}/{repo}/contents/images/", 
        user = config.username,
        repo = repo
    );

    let request_url = [
        url,
        name.to_str().unwrap().to_string(),
        "_".to_string(),
        get_random_name(),
        ".".to_string(),
        ext.unwrap().to_str().unwrap().to_string(),
    ]
    .join("");

    // Step 2
    let mut json_body: Vec<u8>= format!(
        "{{\"message\": \"upload file for github readme or disussion\", \"committer\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"sha\":\"\", \"content\": \"{}",
        config.author,
        config.email, 
        file_base64
    ).as_bytes().to_vec();
    json_body.extend_from_slice("\"}".as_bytes());

    // Step 3
    let client = reqwest::blocking::Client::new();
    let res = client
        .put(request_url) 
        .header(AUTHORIZATION, format_auth_token(config))
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        .body(json_body)
        .send()?;
 
        //dbg!("{:#?}", &res.text());

    let response: Content = res.json().unwrap();   
    Ok(response)
}

// Form Bearer Auth Token
pub fn format_auth_token(config: Config) -> String{
    let token = format!("Bearer {auth_token}", auth_token = config.auth_token);
    token
}
// Random name for the file
pub fn get_random_name() -> String {
   
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    s
}