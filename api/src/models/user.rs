use chrono::NaiveDateTime;
use serde::Serialize;

/// Represents a user entity in the system with authentication, profile, and account status fields.
#[derive(Debug, Serialize)]
pub struct User {
    /// API key for Nekoi.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// Blacklist status (0 = normal, 1 = temporary ban, 2 = permanent ban).
    pub blacklisted: Option<i8>,

    /// Timestamp of user registration in UTC.
    pub created_at: NaiveDateTime,

    /// User's contact email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Premium status indicator (0 = standard, 1 = gold tier).
    pub gold: Option<i8>,

    /// Unique User ID.
    pub id: i32,

    /// Public display name.
    pub nickname: String,

    /// Hashed password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Unique username.
    pub username: String,
}
