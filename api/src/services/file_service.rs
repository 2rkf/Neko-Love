use std::env;

use anyhow::Context;
use aws_sdk_s3::Client as S3Client;
use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use mime_guess::from_path;

use crate::app_state::AppState;
use crate::models::response::ApiResponse;

/// Serves a file from S3 bucket at /assets/{content_type}/{category}/{filename}
pub async fn serve_file(
    State(state): State<AppState>,
    Path(filename): Path<String>,
) -> Result<Response<Body>, anyhow::Error> {
    let content_types = ["sfw", "nsfw"];
    let categories_str = env::var("CATEGORIES").expect("Missing 'CATEGORIES'");
    let categories: Vec<&str> = categories_str.split(",").collect();

    if let Some(cached) = state.cache.get(&filename).await {
        let mime = from_path(&filename).first_or_octet_stream();
        return Ok(Response::builder()
            .header("Content-Type", mime.as_ref())
            .body(Body::from(cached))
            .unwrap());
    }

    for content_type in content_types.iter() {
        for category in categories.iter() {
            let s3_key = format!("assets/{}/{}/{}", content_type, category, filename);

            match try_fetch_s3_object(&state.s3_client, &state.s3_bucket, &s3_key).await {
                Ok(bytes) => {
                    println!("Found file at S3 key: {}", s3_key);
                    state.cache.insert(filename.clone(), bytes.clone()).await;
                    let mime = from_path(&filename).first_or_octet_stream();
                    return Ok(Response::builder()
                        .header("Content-Type", mime.as_ref())
                        .body(Body::from(bytes))
                        .unwrap());
                }
                Err(_) => {
                    continue;
                }
            }
        }
    }

    Ok(build_not_found_response(&filename))
}

/// Attempts to fetch an object from S3
async fn try_fetch_s3_object(
    s3_client: &S3Client,
    bucket: &str,
    key: &str,
) -> Result<bytes::Bytes, anyhow::Error> {
    let get_obj = s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await
        .context(format!("Failed to fetch object from S3: {}", key))?;

    let bytes = get_obj
        .body
        .collect()
        .await
        .context(format!("Failed to read S3 object body: {}", key))?
        .into_bytes();

    Ok(bytes)
}

/// Builds a 404 response for file not found
fn build_not_found_response(filename: &str) -> Response<Body> {
    let response = ApiResponse {
        id: None,
        message: Some(format!("File '{}' not found.", filename)),
        success: false,
        status: StatusCode::NOT_FOUND.as_u16(),
        url: None,
    };

    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(serde_json::to_string(&response).unwrap()))
        .unwrap()
}
