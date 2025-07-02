use axum::{
    Json,
    extract::State,
    http::StatusCode,
};

use crate::{
    app_state::AppState,
    models::{auth::AuthClaims, response::ApiResponse, user::User},
};

pub async fn get_me(
    State(state): State<AppState>,
    AuthClaims(claims): AuthClaims,
) -> Result<Json<User>, (StatusCode, Json<ApiResponse>)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", claims.sub)
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
            )
        })?;

    Ok(Json(user))
}
