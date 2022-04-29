use reqwest::{
    self,
    header::{ACCEPT, CONTENT_TYPE, USER_AGENT, AUTHORIZATION},
};
use serde::{Deserialize, Serialize};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoList {
    pub path: String,
    pub mode: String,
    pub sha: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Content {
    pub tree: Vec<RepoList>,
}

pub fn get_repo_file_list(
    config: &crate::config::Config,
    repo: String,
    branch: String
) -> Result<Content, Box<dyn std::error::Error>> {
    // Step 1
    let request_url = format!(
        "https://api.github.com/repos/{user}/{repo}/git/trees/{branch}?recursive=1",
        user = config.username,
        repo = repo,
        branch = branch,
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

 
    match res.json::<Content>() {
        Ok(response) => { 
            Ok(response)
        }
        Err(e) => Err(Box::new(e))
    } 
}

// Form Bearer Auth Token
pub fn format_auth_token(config: &Config) -> String{
    let token = format!("Bearer {auth_token}", auth_token = config.auth_token);
    token
}