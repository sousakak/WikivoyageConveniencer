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

pub async fn get_users() -> Result<Vec<UserData>, String> {
    let client = WikiClient::ja_voyage();

    let url = format!(
        "{}?action=query\
        &list=allusers\
        &auprop=editcount|registration\
        &aulimit=50\
        &format=json",
        client.api_url
    );

    let http_client = reqwest::Client::builder()
        .user_agent(
            "WikivoyageConveniencer/0.1 (https://github.com/your-name/WikivoyageConveniencer)"
        )
        .build()
        .map_err(|e| e.to_string())?;

    let response = http_client
        .get(&url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = response.status();

    if !response.status().is_success() {
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "<unable to read body>".to_string());

        return Err(format!(
            "HTTP {}: {}",
            status,
            body
        ));
    }

    let data: ApiResponse = response
        .json()
        .await
        .map_err(|e| e.to_string())?;

    Ok(
        data.query
            .allusers
            .into_iter()
            .map(|u| UserData {
                name: u.name,
                registration: u.registration,
                edit_count: u.edit_count.unwrap_or(0),
            })
            .collect(),
    )
}