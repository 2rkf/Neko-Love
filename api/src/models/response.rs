use serde::Serialize;

/// Standardised API response format.
#[derive(Debug, Serialize)]
pub struct ApiResponse {
    /// Unique identifier for the processed image (when applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Human-readable message, typically used for errors or status updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// HTTP status code representing the operation result.
    pub status: u16,

    /// Boolean indicating whether the operation succeeded.
    pub success: bool,

    /// URL to access the processed image (when applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
