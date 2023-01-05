use std::{env, iter::Map};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize)]
struct Repository {
    id: i32,
    full_name: String,
    html_url: String,
    releases_url: String,
}

#[derive(Deserialize)]
struct Subject {
    title: String,
    url: String,
    #[serde(alias="type")]
    kind: String,
}

#[derive(Deserialize)]
struct Notification {
    id: String,
    repository: Repository,
    subject: Subject,
    reason: String,
    unread: bool,
    url: String,
}

fn main() -> anyhow::Result<()> {
    let ghp = env::var("GH_RELEASE")?;

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

    let payload = res.json::<Vec::<Notification>>()?;

    payload.iter()
        .for_each(|item| {
            println!("#{} - [{}] {} ({})", item.id, item.reason, item.subject.title, item.subject.url);
        });

    Ok(())
}
