use serde::Serialize;

/// The image response
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub message: String,
    pub success: bool,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
