use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};

const APP_AGENT: &str = "github.com/alfreddobradi/gh-bell:v0.1.0";

const GITHUB_API_URL: &str = "https://api.github.com/notifications";
const GITHUB_API_ACCEPT: &str = "application/vnd.github+json";
const GITHUB_API_VERSION_KEY: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";

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
    pub subject: Subject,
    pub reason: String,
}

pub fn get_notifications(ghp: String) -> anyhow::Result<Vec<Notification>> {    
    let client = reqwest::blocking::Client::new();

    let token = HeaderValue::from_str(format!("Bearer {}", ghp).as_str())?;

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static(GITHUB_API_ACCEPT));
    headers.insert(AUTHORIZATION, token);
    headers.insert(USER_AGENT, HeaderValue::from_static(APP_AGENT));
    headers.insert(GITHUB_API_VERSION_KEY, HeaderValue::from_static(GITHUB_API_VERSION));

    let res = client.get(GITHUB_API_URL).headers(headers).send()?.error_for_status()?;

    Ok(res.json::<Vec::<Notification>>()?)
}
