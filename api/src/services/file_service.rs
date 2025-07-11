use axum::{body::Body, extract::State, http::StatusCode, response::Response};
use mime_guess::from_path;

use crate::app_state::AppState;
use crate::models::response::ApiResponse;

/// Serves a file from either `/assets/sfw` or `/assets/nsfw` subdirectories.
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
    for content_type in content_types.iter() {
        let prefix = format!("assets/{}", content_type);
        let response = state
            .s3_client
            .list_objects_v2()
            .bucket(state.s3_bucket.clone())
            .prefix(&prefix)
            .send()
            .await?;

        for obj in response.contents() {
            if let Some(key) = obj.key() {
                if key.ends_with(&filename) {
                    let get_obj = state
                        .s3_client
                        .get_object()
                        .bucket(state.s3_bucket)
                        .key(key)
                        .send()
                        .await?;

                    let bytes = get_obj.body.collect().await?.into_bytes();

                    let response = Response::builder()
                        .header("Content-Type", mime.as_ref())
                        .body(Body::from(bytes.clone()))
                        .unwrap();

                    state.cache.insert(filename.clone(), bytes).await;
                    return Ok(response);
                }
            }
        }
    }

    let response = ApiResponse {
        id: None,
        message: Some("File not found.".into()),
        success: false,
        status: StatusCode::NOT_FOUND.as_u16(),
        url: None,
    };

    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(serde_json::to_string(&response)?))
        .unwrap())
}
