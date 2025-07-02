use serde::Serialize;

/// The image response
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    pub status: u16,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
