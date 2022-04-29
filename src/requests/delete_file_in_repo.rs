// use std::{path::{PathBuf, Path}, ffi::OsStr};
// use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub content: Option<String>,                                        // Value may return as Null
}
 
// Put a file in repository
// DELETE /repos/{owner}/{repo}/contents/{path} 
// https://docs.github.com/en/rest/repos/contents#delete-a-file
pub fn delete_file_in_repo(
    config: crate::config::Config,
    repo: String, 
    sha: String,
    file_name: String,
) -> Result<(reqwest::StatusCode, Response), Box<dyn std::error::Error>> {
   
    // Step 1 : Build request url
    let request_url = format!("https://api.github.com/repos/{user}/{repo}/contents/{file_name}", 
        user = config.username,
        repo = repo,
        file_name = file_name,
    );
 
    // Step 2 : Build the body
    let mut json_body: Vec<u8>= format!(
        "{{\"message\": \"remove file from repo\", \"committer\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"sha\":\"{}\"",
        config.author,
        config.email, 
        sha
    ).as_bytes().to_vec();
    json_body.extend_from_slice("\"}".as_bytes());

    // Step 3 : Exexute call
    let client = reqwest::blocking::Client::new();
    let res = client
        .delete(request_url) 
        .header(AUTHORIZATION, format_auth_token(config))
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        .body(json_body)
        .send()?;

 
    // Step 4 : Group the return statement
    let status_code = res.status().to_owned();
    let response: Response = res.json().unwrap();   

    Ok((status_code, response)) 
}

// Form Bearer Auth Token
pub fn format_auth_token(config: Config) -> String{
    let token = format!("Bearer {auth_token}", auth_token = config.auth_token);
    token
}
