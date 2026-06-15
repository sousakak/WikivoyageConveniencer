use serde::{Deserialize, Serialize};

use crate::wiki::client::WikiClient;

#[derive(Debug, Serialize)]
pub struct UserData {
    pub name: String,
    pub registration: Option<String>,
    pub edit_count: u64,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    query: Query,
    #[serde(rename = "continue")]
    cont: Option<Continue>,
}

#[derive(Debug, Deserialize)]
struct Query {
    allusers: Vec<ApiUser>,
}

#[derive(Debug, Deserialize)]
struct ApiUser {
    name: String,
    registration: Option<String>,
    #[serde(rename = "editcount")]
    edit_count: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct Continue {
    #[serde(rename = "aufrom")]
    aufrom: String,
}

#[derive(Debug, Serialize)]
pub struct UserPage {
    pub users: Vec<UserData>,
    pub next_from: Option<String>,
}

pub async fn get_users(aufrom: Option<String>) -> Result<UserPage, String> {
    let client = WikiClient::ja_voyage();

    let mut url = format!(
        "{}?action=query&list=allusers&auprop=editcount|registration&aulimit=50&format=json",
        client.api_url
    );

    if let Some(from) = aufrom {
        url.push_str(&format!("&aufrom={}", from));
    }

    let http_client = reqwest::Client::builder()
        .user_agent("WikivoyageConveniencer/0.1")
        .build()
        .map_err(|e| e.to_string())?;

    let response = http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: ApiResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let next = data.cont.map(|c| c.aufrom);

    let users = data.query.allusers
        .into_iter()
        .map(|u| UserData {
            name: u.name,
            registration: u.registration,
            edit_count: u.edit_count.unwrap_or(0),
        })
        .collect();

    Ok(UserPage {
        users,
        next_from: next,
    })
}