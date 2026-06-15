use crate::wiki::users::UserData;

#[tauri::command]
pub async fn get_users() -> Result<Vec<UserData>, String> {
    println!("get_users called");

    crate::wiki::users::get_users().await
}
