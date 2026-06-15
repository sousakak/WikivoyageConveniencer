// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde_json::Value;

mod commands;
mod wiki;

#[tauri::command]
fn greet(name: String) -> String {
    format!("こんにちは、{}さん！", name)
}

#[tauri::command]
async fn recent_changes() -> Result<Value, String> {
    let url = "https://en.wikipedia.org/w/api.php?action=query&list=recentchanges&format=json";

    let client = reqwest::Client::builder()
        .user_agent("WikivoyageConveniencer/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let res = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res
        .text()
        .await
        .map_err(|e| e.to_string())?;

    println!("DEBUG RESPONSE: {}", text);

    let json: Value = serde_json::from_str(&text)
        .map_err(|e| format!("JSON parse error: {}\nBODY: {}", e, text))?;

    Ok(json)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::users::get_users,
            greet,
            recent_changes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
