//! Error handling for the Docaroo API client

use crate::models::ErrorResponse;
use thiserror::Error;

/// Result type alias for Docaroo operations
pub type Result<T> = std::result::Result<T, DocarooError>;

/// Errors that can occur when interacting with the Docaroo API
#[derive(Error, Debug)]
pub enum DocarooError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// API returned an error response
    #[error("API error: {message} (code: {code})")]
    ApiError {
        /// Error code from the API
        code: String,
        /// Error message from the API
        message: String,
        /// Optional request ID for support
        request_id: Option<String>,
    },

    /// Invalid request parameters
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded. Retry after {retry_after} seconds")]
    RateLimitExceeded {
        /// Number of seconds to wait before retrying
        retry_after: u64,
    },

    /// Authentication failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// Deserialization error
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// URL parsing error
    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),
}

impl DocarooError {
    /// Create an API error from an error response
    pub fn from_error_response(response: ErrorResponse) -> Self {
        match response.error.as_str() {
            "rate_limit_exceeded" => {
                let retry_after = response
                    .details
                    .as_ref()
                    .and_then(|d| d.get("retryAfter"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(60);
                Self::RateLimitExceeded { retry_after }
            }
            "unauthorized" => Self::AuthenticationFailed(response.message),
            _ => Self::ApiError {
                code: response.error,
                message: response.message,
                request_id: response.request_id,
            },
        }
    }

    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RequestFailed(_) | Self::RateLimitExceeded { .. }
        )
    }

    /// Get the request ID if available (for support purposes)
    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::ApiError { request_id, .. } => request_id.as_deref(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_error_from_response() {
        let error_response = ErrorResponse {
            error: "bad_request".to_string(),
            message: "Invalid NPI format".to_string(),
            details: None,
            request_id: Some("req_123".to_string()),
            timestamp: Some(Utc::now()),
        };

        let error = DocarooError::from_error_response(error_response);
        match error {
            DocarooError::ApiError {
                code,
                message,
                request_id,
            } => {
                assert_eq!(code, "bad_request");
                assert_eq!(message, "Invalid NPI format");
                assert_eq!(request_id, Some("req_123".to_string()));
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_rate_limit_error() {
        let error_response = ErrorResponse {
            error: "rate_limit_exceeded".to_string(),
            message: "Too many requests".to_string(),
            details: Some(serde_json::json!({ "retryAfter": 120 })),
            request_id: None,
            timestamp: None,
        };

        let error = DocarooError::from_error_response(error_response);
        match error {
            DocarooError::RateLimitExceeded { retry_after } => {
                assert_eq!(retry_after, 120);
            }
            _ => panic!("Expected RateLimitExceeded"),
        }
    }

    #[test]
    fn test_is_retryable() {
        let rate_limit_error = DocarooError::RateLimitExceeded { retry_after: 60 };
        assert!(rate_limit_error.is_retryable());

        let api_error = DocarooError::ApiError {
            code: "bad_request".to_string(),
            message: "Invalid request".to_string(),
            request_id: None,
        };
        assert!(!api_error.is_retryable());
    }
}