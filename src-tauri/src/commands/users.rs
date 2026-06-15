use crate::wiki::users;

#[tauri::command]
pub async fn get_users(aufrom: Option<String>) -> Result<users::UserPage, String> {
    users::get_users(aufrom).await
}
