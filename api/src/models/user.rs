use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    pub blacklisted: Option<i8>,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub gold: Option<i8>,
    pub id: i32,
    pub nickname: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub username: String,
}
