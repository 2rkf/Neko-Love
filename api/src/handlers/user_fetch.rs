use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    models::{response::ApiResponse, user::User},
};

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
        auth_token: None,
        created_at: user.created_at,
        email: None,
        id: user.id,
        nickname: user.nickname,
        password: None,
        username: user.username,
    }))
}
