use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{ApiResponse, app_state::AppState, models::user::User};

pub async fn fetch_user(
    State(state): State<AppState>,
    Path(username): Path<String>,
) -> Result<Json<User>, (StatusCode, Json<ApiResponse>)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", username)
        .fetch_one(&state.pool)
        .await
        .map_err(|_| {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse {
                    id: None,
                    message: Some("User is not found.".into()),
                    status: StatusCode::NOT_FOUND.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        })?;
    Ok(Json(User {
        api_key: None,
        blacklisted: user.blacklisted,
        created_at: user.created_at,
        email: None,
        gold: user.gold,
        id: user.id,
        nickname: user.nickname,
        password: None,
        username: user.username,
    }))
}
