use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use bytes::Bytes;
use mime_guess::from_path;
use std::{fs, path::PathBuf};

use crate::app_state::AppState;
use crate::models::response::ApiResponse;

/// Serves a file from either `/assets/sfw` or `/assets/nsfw` subdirectories
pub async fn serve_file(
    State(state): State<AppState>,
    filename: String,
) -> Result<Response<Body>, anyhow::Error> {
    let mime = from_path(&filename).first_or_octet_stream();

    if let Some(cached) = state.cache.get(&filename).await {
        return Ok(Response::builder()
            .header("Content-Type", mime.as_ref())
            .body(Body::from(cached))
            .unwrap());
    }

    let content_types = ["sfw", "nsfw"];
    let assets_base = PathBuf::from("./assets");

    for content_type in content_types.iter() {
        let content_path = assets_base.join(content_type);

        if let Ok(entries) = fs::read_dir(&content_path) {
            for entry in entries {
                let category_path = entry?.path();
                if category_path.is_dir() {
                    let potential_path = category_path.join(&filename);
                    if potential_path.exists() {
                        let bytes = tokio::fs::read(&potential_path).await?;
                        let response = Response::builder()
                            .header("Content-Type", mime.as_ref())
                            .body(Body::from(bytes.clone()))
                            .unwrap();

                        state
                            .cache
                            .insert(filename.clone(), Bytes::from(bytes))
                            .await;

                        return Ok(response);
                    }
                }
            }
        }
    }

    let response = ApiResponse {
        id: None,
        message: "File not found.".into(),
        success: false,
        status: StatusCode::NOT_FOUND.as_u16(),
        url: None,
    };

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(serde_json::to_string(&response)?))
        .unwrap())
}
