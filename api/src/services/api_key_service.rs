use axum::{Json, http::StatusCode};
use sqlx::MySqlPool;

use crate::{ApiResponse, models::user::User};

pub async fn find_by_key(
    pool: MySqlPool,
    token: String,
) -> Result<Json<User>, (StatusCode, Json<ApiResponse>)> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE api_key = ?", token)
        .fetch_one(&pool)
        .await
        .map_err(|_| {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse {
                    id: None,
                    message: Some("Unauthorised.".into()),
                    status: StatusCode::UNAUTHORIZED.as_u16(),
                    success: false,
                    url: None,
                }),
            );
        })?;
    Ok(Json(user))
}
