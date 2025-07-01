use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, http::StatusCode,
};
use crate::models::response::ApiResponse;
use std::sync::Arc;
use crate::services::image_service::ImageService;

/// Handler for GET /api/v4/{content_type}/{category}
/// Returns a random image from the specified category
pub async fn get_random_image(
    Path((content_type, category)): Path<(String, String)>,
    State(image_service): State<Arc<ImageService>>,
) -> impl IntoResponse {
    match image_service.get_random_image(&content_type, &category) {
        Ok((id, filename)) => {
            let response = ApiResponse {
                id: Some(id.clone()),
                message: "".into(),
                success: true,
                status: StatusCode::OK.as_u16(),
                url: Some(image_service.build_image_url(&filename)),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Error getting random image: {}", e);
            let response = ApiResponse {
                id: None,
                message: "Unknown image category.".into(),
                success: false,
                status: StatusCode::BAD_REQUEST.as_u16(),
                url: None,
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}