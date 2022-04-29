//https://api.github.com/repos/mjehrhart/minty/branches

use crate::config::Config;
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoBranches {
    pub name: String,
}

pub fn get_repo_branches(
    config: &crate::config::Config,
    repo: String,
) -> Result<Vec<RepoBranches>, Box<dyn std::error::Error>> {
    // Step 1
    let request_url = format!(
        "https://api.github.com/repos/{user}/{repo}/branches",
        user = config.username,
        repo = repo,
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

    match res.json::<Vec<RepoBranches>>() {
        Ok(response) => Ok(response),
        Err(e) => Err(Box::new(e)),
    }
}

// Form Bearer Auth Token
pub fn format_auth_token(config: &Config) -> String {
    let token = format!("Bearer {auth_token}", auth_token = config.auth_token);
    token
}
