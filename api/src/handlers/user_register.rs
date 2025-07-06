use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::env;

use crate::{
    ApiResponse,
    app_state::AppState,
    models::{auth::RegisterRequest, user::User},
};

pub async fn register_user(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<RegisterRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let api_key = env::var("API_KEY").expect("API_KEY must be set");

    if auth.token() != api_key {
        return (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                id: None,
                message: Some("Unauthorised.".into()),
                status: StatusCode::UNAUTHORIZED.as_u16(),
                success: false,
                url: None,
            }),
        );
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hashed_password = match argon2.hash_password(payload.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
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

    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username LIKE ? OR email LIKE ?",
        payload.username,
        payload.email
    )
    .fetch_one(&state.pool)
    .await;

    match user {
        Ok(_) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse {
                    id: None,
                    message: Some("This user already exists.".into()),
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        }
        Err(_) => {
            let result = sqlx::query(
                "INSERT INTO users (email, password, username, nickname) VALUES (?, ?, ?, ?)",
            )
            .bind(&payload.email)
            .bind(&hashed_password)
            .bind(&payload.username)
            .bind(&payload.username)
            .execute(&state.pool)
            .await;

            match result {
                Ok(_) => {
                    return (
                        StatusCode::CREATED,
                        Json(ApiResponse {
                            id: None,
                            message: Some("User successfully registered.".into()),
                            status: StatusCode::CREATED.as_u16(),
                            success: true,
                            url: None,
                        }),
                    );
                }
                Err(_) => {
                    return (
                        StatusCode::CONFLICT,
                        Json(ApiResponse {
                            id: None,
                            message: Some("User already exists or database error.".into()),
                            status: StatusCode::CONFLICT.as_u16(),
                            success: false,
                            url: None,
                        }),
                    );
                }
            }
        }
    }
}
