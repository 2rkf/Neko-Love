use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::Value;

use crate::{ApiResponse, AuthClaims, app_state::AppState};

/// Updates the user information.
pub async fn update_user(
    State(state): State<AppState>,
    AuthClaims(_): AuthClaims,
    Path(username): Path<String>,
    Json(mut body): Json<Value>,
) -> (StatusCode, Json<ApiResponse>) {
    if let Some(password) = body.get("password").and_then(|v| v.as_str()) {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => {
                body["password"] = Value::String(hash.to_string());
            }
            Err(_) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse {
                        id: None,
                        message: Some("Failed to hash password.".into()),
                        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        success: false,
                        url: None,
                    }),
                );
            }
        };
    }

    let update_fields: Vec<String> = body
        .as_object()
        .map(|obj| obj.iter().map(|(key, _)| format!("{} = ?", key)).collect())
        .unwrap_or_default();

    if update_fields.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                id: None,
                message: Some("No fields to update.".into()),
                status: StatusCode::BAD_REQUEST.as_u16(),
                success: false,
                url: None,
            }),
        );
    }

    let query = format!(
        "UPDATE users SET {} WHERE username = ?",
        update_fields.join(", ")
    );

    let mut query_builder = sqlx::query(&query);

    for (key, value) in body.as_object().unwrap().iter() {
        query_builder = match value {
            Value::String(s) => query_builder.bind(s),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    query_builder.bind(i)
                } else if let Some(f) = n.as_f64() {
                    query_builder.bind(f)
                } else {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiResponse {
                            id: None,
                            message: Some(
                                format!("Invalid number format for field '{}'", key).into(),
                            ),
                            status: StatusCode::UNAUTHORIZED.as_u16(),
                            success: false,
                            url: None,
                        }),
                    );
                }
            }
            Value::Bool(b) => query_builder.bind(b),
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse {
                        id: None,
                        message: Some(format!("Unsupported data type for field '{}'", key).into()),
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        success: false,
                        url: None,
                    }),
                );
            }
        };
    }

    query_builder = query_builder.bind(&username);

    match query_builder.execute(&state.pool).await {
        Ok(result) if result.rows_affected() > 0 => {
            return (
                StatusCode::OK,
                Json(ApiResponse {
                    id: None,
                    message: Some("User successfully updated.".into()),
                    status: StatusCode::OK.as_u16(),
                    success: true,
                    url: None,
                }),
            );
        }
        Ok(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    id: None,
                    message: Some("User is not found.".into()),
                    status: StatusCode::NOT_FOUND.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse {
                    id: None,
                    message: Some("Database error.".into()),
                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        }
    }
}
