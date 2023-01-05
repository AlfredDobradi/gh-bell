use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

#[derive(Deserialize)]
pub struct Repository {
    pub id: i32,
    pub full_name: String,
    pub html_url: String,
    pub releases_url: String,
}

#[derive(Deserialize)]
pub struct Subject {
    pub title: String,
    pub url: String,
    #[serde(alias="type")]
    pub kind: String,
}

#[derive(Deserialize)]
pub struct Notification {
    pub id: String,
    pub repository: Repository,
    pub subject: Subject,
    pub reason: String,
    pub unread: bool,
    pub url: String,
}

pub fn get_notifications(ghp: String) -> anyhow::Result<Vec<Notification>> {    
    let url = String::from("https://api.github.com/notifications");
    let client = reqwest::blocking::Client::new();

    let token = HeaderValue::from_str(format!("Bearer {}", ghp).as_str())?;

    // Accept
    // Authorization
    // X-Github-Api-Version
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github+json"));
    headers.insert(AUTHORIZATION, token);
    headers.insert(USER_AGENT, HeaderValue::from_static("github.com/alfreddobradi/gh-bell:v0.1.0"));
    headers.insert("X-GitHub-Api-Version", HeaderValue::from_static("2022-11-28"));

    println!("{:?}", headers);

    let res = client.get(url).headers(headers).send()?.error_for_status()?;

    Ok(res.json::<Vec::<Notification>>()?)
}
