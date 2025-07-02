use crate::{app_state::AppState, models::response::ApiResponse};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

/// Handler for GET /api/v1/{content_type}/{category}
/// Returns a random image from the specified category
pub async fn get_random_image(
    Path((content_type, category)): Path<(String, String)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state
        .image_service
        .get_random_image(&content_type, &category)
    {
        Ok((id, filename)) => {
            let response = ApiResponse {
                id: Some(id.clone()),
                message: None,
                success: true,
                status: StatusCode::OK.as_u16(),
                url: Some(state.image_service.build_image_url(&filename)),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Error getting random image: {}", e);
            let response = ApiResponse {
                id: None,
                message: Some("Unknown image category.".into()),
                success: false,
                status: StatusCode::BAD_REQUEST.as_u16(),
                url: None,
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}
