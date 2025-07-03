use serde::{Deserialize, Serialize};

pub struct AuthClaims(pub Claims);

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub exp: usize,
    pub sub: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub password: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: String,
}
