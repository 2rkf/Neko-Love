mod app_state;
mod handlers;
mod logger;
mod models;
mod services;

use axum::extract::State;
use axum::response::IntoResponse;
use axum::{extract::Path, http::StatusCode, routing::get, Router};
use axum::{middleware, routing, Json};
use dotenv::dotenv;
use std::env;
use std::path::PathBuf;

use crate::app_state::create_state;
use crate::handlers::images::get_random_image;
use crate::logger::log_requests;
use crate::models::response::ApiResponse;
use crate::services::file_service::serve_file;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let server_addr = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:3030".into());
    let assets_path = PathBuf::from("./assets");
    let base_url = env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:3030".into());
    let state = create_state(assets_path, base_url).unwrap();
    let image_service = state.image_service.clone();

    let app = Router::new()
        .route("/api/v1/{content_type}/{category}", get(get_random_image))
        .route(
            "/api/v1/{content_type}/{category}",
            routing::any(|| async {
                let response = ApiResponse {
                    id: None,
                    message: "Method not allowed.".into(),
                    success: false,
                    status: StatusCode::METHOD_NOT_ALLOWED.as_u16(),
                    url: None,
                };

                (StatusCode::METHOD_NOT_ALLOWED, Json(response))
            }),
        )
        .route(
            "/img/{filename}",
            get(|Path(filename): Path<String>| async move {
                match serve_file(State(state), filename).await {
                    Ok(res) => res,
                    Err(_) => {
                        (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
                    }
                }
            }),
        )
        .layer(middleware::from_fn(log_requests))
        .with_state(image_service);

    let listener = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    println!(
        "Server running on {}",
        listener.local_addr().unwrap().to_string()
    );

    axum::serve(listener, app).await.unwrap();
}
