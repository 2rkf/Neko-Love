use serde::{Deserialize, Serialize};

/// A wrapper around JWT claims for authentication purposes.
/// This newtype pattern helps distinguish raw claims from verified claims in the type system.
pub struct AuthClaims(pub Claims);

/// JWT claims structure containing standard fields for token validation.
#[derive(Deserialize, Serialize)]
pub struct Claims {
    /// Expiration timestamp (as UNIX timestamp).
    pub exp: usize,
    /// Subject identifier (the username).
    pub sub: String,
}

/// Request payload for user login endpoint.
#[derive(Deserialize)]
pub struct LoginRequest {
    /// User's password.
    pub password: String,
    /// User's username.
    pub username: String,
}

/// Request payload for user registration endpoint.
#[derive(Deserialize)]
pub struct RegisterRequest {
    /// User's email address.
    pub email: String,
    /// User's chosen password.
    pub password: String,
    /// User's chosen unique username.
    pub username: String,
}
