use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::headers::HeaderMap;

use crate::{app_state::AppState, models::response::ApiResponse, services::auth_token_service::find_by_auth};

/// Handler for GET /api/v1/{content_type}/{category}
/// Returns a random image from the specified category
pub async fn get_random_image(
    Path((content_type, category)): Path<(String, String)>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_token = headers.get("Authorization")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_string());

    let token = match auth_token {
        Some(token) => token,
        None => {
            let response = ApiResponse {
                id: None,
                message: Some("Missing 'Authorization' header.".into()),
                success: false,
                status: StatusCode::BAD_REQUEST.as_u16(),
                url: None,
            };
            return (StatusCode::BAD_REQUEST, Json(response));
        }
    };

    match find_by_auth(state.pool, token).await {
        Ok(Json(user)) => user,
        Err((status, res)) => {
            return (status, res)
        }
    };

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
