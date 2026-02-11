use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tauri::State;

use crate::database::Database;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResult {
    pub success: bool,
    pub user: Option<UserInfo>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn login(
    user_id: String,
    password: String,
    db: State<'_, Database>,
) -> Result<LoginResult, String> {
    let user = sqlx::query_as::<_, UserInfo>(
        "SELECT id, name, role FROM users WHERE id = $1 AND password = $2",
    )
    .bind(&user_id)
    .bind(&password)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| e.to_string())?;

    match user {
        Some(u) => Ok(LoginResult {
            success: true,
            user: Some(u),
            error: None,
        }),
        None => Ok(LoginResult {
            success: false,
            user: None,
            error: Some("รหัสพนักงานหรือรหัสผ่านไม่ถูกต้อง".to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_users(db: State<'_, Database>) -> Result<Vec<UserInfo>, String> {
    sqlx::query_as::<_, UserInfo>("SELECT id, name, role FROM users ORDER BY role, id")
        .fetch_all(&db.pool)
        .await
        .map_err(|e| e.to_string())
}
