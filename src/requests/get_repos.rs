use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT, AUTHORIZATION},
};
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoList {
    id: u32,
    pub name: String,
    full_name: String,
}

pub fn get_repo_list_by_user(
    config: &crate::config::Config,
) -> Result<Vec<RepoList>, Box<dyn std::error::Error>> {
    // Step 1
    let request_url = format!(
        "https://api.github.com/users/{user}/repos",
        user = config.username
    );

    // Step 2
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(request_url)
        .header(AUTHORIZATION, format_auth_token(config))
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        .send()?;
 
    let response: Vec<RepoList> = res.json().unwrap();

    Ok(response)
}

// Form Bearer Auth Token
pub fn format_auth_token(config: &Config) -> String{
    let token = format!("Bearer {auth_token}", auth_token = config.auth_token);
    token
}
