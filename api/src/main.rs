mod app_state;
mod handlers;
mod middlewares;
mod models;
mod services;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{Json, middleware, routing};
use axum::{
    Router,
    extract::{FromRequestParts, Path},
    http::{StatusCode, request::Parts},
    routing::get,
};
use dotenv::dotenv;
use jsonwebtoken::{Validation, decode};
use sqlx::MySqlPool;
use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::create_state;
use crate::handlers::images::get_random_image;
use crate::handlers::{me::get_me, user_fetch::fetch_user, user_register::register_user};
use crate::middlewares::logging::log_requests;
use crate::models::auth::{AuthClaims, Claims};
use crate::models::keys::Keys;
use crate::models::response::ApiResponse;
use crate::services::file_service::serve_file;

pub static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "jwt_secret".into());
    Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root@localhost/neko-love".into());
    let pool = MySqlPool::connect(&db_url).await.unwrap();
    let server_addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3030".into());
    let assets_path = PathBuf::from("./assets");
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3030".into());
    let state = create_state(pool, assets_path, base_url).unwrap();
    let state_img = state.clone();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/v1/{content_type}/{category}", get(get_random_image))
        .route(
            "/api/v1/{content_type}/{category}",
            routing::any(|| async {
                let response = ApiResponse {
                    id: None,
                    message: Some("Method not allowed.".into()),
                    status: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                    success: false,
                    url: None,
                };

                (StatusCode::METHOD_NOT_ALLOWED, Json(response))
            }),
        )
        .route("/api/users", post(register_user))
        .route("/api/users/{username}", get(fetch_user))
        .route("/api/me", get(get_me))
        .route(
            "/img/{filename}",
            get(|Path(filename): Path<String>| async move {
                match serve_file(State(state_img), filename).await {
                    Ok(res) => res,
                    Err(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                    }
                }
            }),
        )
        .fallback(|| async {
            let response = ApiResponse {
                id: None,
                message: Some("Route not found.".into()),
                status: StatusCode::NOT_FOUND.as_u16(),
                success: false,
                url: None,
            };

            (StatusCode::NOT_FOUND, Json(response))
        })
        .layer(middleware::from_fn(log_requests))
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    println!(
        "Server running on {}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app).await.unwrap();
}

impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<ApiResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(token) = auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
            let token_data = decode::<Claims>(token, &KEYS.decoding, &Validation::default());

            match token_data {
                Ok(data) => Ok(AuthClaims(data.claims)),
                Err(_) => Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ApiResponse {
                        id: None,
                        message: Some("Invalid token.".into()),
                        status: StatusCode::UNAUTHORIZED.as_u16(),
                        success: false,
                        url: None,
                    }),
                )),
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    id: None,
                    message: Some("Missing 'Authorization' header.".into()),
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    success: false,
                    url: None,
                }),
            ))
        }
    }
}
