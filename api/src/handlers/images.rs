use crate::{
    app_state::AppState, models::response::ApiResponse, services::auth_token_service::find_by_auth,
};
use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};

/// Handler for GET /api/v1/{content_type}/{category}
/// Returns a random image from the specified category
pub async fn get_random_image(
    Path((content_type, category)): Path<(String, String)>,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let auth_token = headers
        .get("Authorization")
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
            return (StatusCode::BAD_REQUEST, Json(response)).into_response();
        }
    };

    let user = match find_by_auth(state.pool.clone(), token.clone()).await {
        Ok(Json(user)) => user,
        Err((status, res)) => return (status, res).into_response(),
    };

    let extend = user.gold.unwrap_or(0) != 0;
    let rate_status = state.rate_limiter.check(token, extend);

    let mut resp_headers = HeaderMap::new();
    resp_headers.insert("X-RateLimit-Limit", HeaderValue::from(rate_status.limit));
    resp_headers.insert(
        "X-RateLimit-Remaining",
        HeaderValue::from(rate_status.remaining),
    );
    resp_headers.insert(
        "X-RateLimit-Reset",
        HeaderValue::from(rate_status.reset_after),
    );

    if !rate_status.is_allowed {
        if let Some(retry_after) = rate_status.retry_after {
            resp_headers.insert("Retry-After", HeaderValue::from(retry_after));
        }

        let response = ApiResponse {
            id: None,
            message: Some(format!(
                "Rate limit exceeded. Try again in {} seconds.",
                rate_status.retry_after.unwrap_or(1)
            )),
            success: false,
            status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
            url: None,
        };

        return (StatusCode::TOO_MANY_REQUESTS, resp_headers, Json(response)).into_response();
    }

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
            (StatusCode::OK, resp_headers, Json(response)).into_response()
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
            (StatusCode::BAD_REQUEST, resp_headers, Json(response)).into_response()
        }
    }
}
