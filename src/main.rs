mod app;

use rand::{distributions::Alphanumeric, Rng};
use reqwest::{
    self,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
    sha: String,
    size: u64,
    download_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Content {
    content: Response,
}

fn main() {
    //***************************************************** */
    // UI
    //***************************************************** */
    app::controller::run();
}
#[tokio::main]
async fn main3() -> Result<(), Box<dyn std::error::Error>> {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();
    println!("{}", s);

    let request_url = [
        "https://api.github.com/repos/mjehrhart/sandbox/contents/".to_string(),
        s,
        ".png".to_string(),
    ]
    .join("");
    println!("{}", request_url);

    let image = include_bytes!("../resources/checked.png");
    let base64_image = base64::encode(&image);

    // Sha dummy value is required for PUT
    let sha_value = "34b8582186de0e1a8a9d60953026ae668fc8f3dc";

    let mut json_body: Vec<u8>= format!(
        "{{\"message\": \"upload file\", \"commiter\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"sha\":\"{}\", \"content\": \"{}",
        "matthew j.",
        "mjeoffline@gmail.com",
        sha_value,
        base64_image
    ).as_bytes().to_vec();
    json_body.extend_from_slice("\"}".as_bytes());

    let client = reqwest::Client::builder().build()?;
    let res = client
        .put(request_url)
        .header(
            AUTHORIZATION,
            "Bearer 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",
        )
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        .body(json_body)
        .send()
        .await?;

    let response: Content = res.json().await?;
    println!("{:#?}", response);

    //***************************************************** */
    // UI
    //***************************************************** */
    //app::controller::run();

    Ok(())
}

fn static_upload() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1
    let image = include_bytes!("../resources/checked.png");
    let base64_image = base64::encode(&image);

    // Sha dummy value is required for PUT
    let sha_value = "34b8582186de0e1a8a9d60953026ae668fc8f3dc";

    let mut json_body: Vec<u8>= format!(
        "{{\"message\": \"upload file\", \"commiter\": {{\"name\": \"{}\", \"email\":\"{}\"}}, \"sha\":\"{}\", \"content\": \"{}",
        "matthew j.",
        "mjeoffline@gmail.com",
        sha_value,
        base64_image
    ).as_bytes().to_vec();
    json_body.extend_from_slice("\"}".as_bytes());

    // Step 2
    let client = reqwest::blocking::Client::new();
    let res = client
        .put("https://api.github.com/repos/mjehrhart/sandbox/contents/test.png")
        .header(
            AUTHORIZATION,
            "Bearer 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",
        )
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        .body(json_body)
        .send()?;

    let response: Content = res.json().unwrap();
    println!("{:#?}", response);

    //println!("{:?}", res.);
    Ok(())
}

async fn main_works() -> Result<(), Box<dyn std::error::Error>> {
    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/stargazers",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);
    // let response = reqwest::get(&request_url).await?;

    // let users: Vec<User> = response.json().await?;
    // println!("{:?}", users);

    let client = reqwest::Client::builder().build()?;
    let res = client
        .get(request_url)
        .header(
            AUTHORIZATION,
            "Bearer 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",
        )
        .header(USER_AGENT, "rustlang")
        .header(CONTENT_TYPE, "application/vnd.github.v3+json")
        .header(ACCEPT, "application/json")
        //.body(json_body)
        .send()
        .await?;

    //let t = res.text().await?;

    //println!("{:#?}", t);
    let users: Vec<User> = res.json().await?;
    println!("{:#?}", users);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct User {
    login: String,
}
