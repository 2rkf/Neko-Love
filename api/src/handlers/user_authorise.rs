use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::{
    TypedHeader,
    headers::authorization::{Authorization, Bearer},
};
use jsonwebtoken::{Header, encode};
use std::env;

use crate::{
    ApiResponse, KEYS,
    app_state::AppState,
    models::auth::{Claims, LoginRequest},
};

/// Authorises the user.
pub async fn authorise_user(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(payload): Json<LoginRequest>,
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

    let user = sqlx::query!(
        "SELECT password FROM users WHERE username = ?",
        payload.username
    )
    .fetch_optional(&state.pool)
    .await;

    match user {
        Ok(Some(record)) => {
            let argon2 = Argon2::default();
            let parsed_hash = PasswordHash::new(&record.password);

            match parsed_hash {
                Ok(hash)
                    if argon2
                        .verify_password(payload.password.as_bytes(), &hash)
                        .is_ok() =>
                {
                    let expiration = chrono::Utc::now().timestamp() as usize + 2629744;
                    let claims = Claims {
                        exp: expiration,
                        sub: payload.username.clone(),
                    };
                    let token = encode(&Header::default(), &claims, &KEYS.encoding);

                    match token {
                        Ok(t) => {
                            return (
                                StatusCode::OK,
                                Json(ApiResponse {
                                    id: None,
                                    message: Some(t),
                                    status: StatusCode::OK.as_u16(),
                                    success: true,
                                    url: None,
                                }),
                            );
                        }
                        Err(_) => {
                            return (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ApiResponse {
                                    id: None,
                                    message: Some("Failed to generate token.".into()),
                                    status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                                    success: false,
                                    url: None,
                                }),
                            );
                        }
                    }
                }
                _ => {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse {
                            id: None,
                            message: Some("Invalid username or password.".into()),
                            status: StatusCode::UNAUTHORIZED.as_u16(),
                            success: false,
                            url: None,
                        }),
                    );
                }
            }
        }
        _ => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    id: None,
                    message: Some("Invalid username or password.".into()),
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        }
    }
}
