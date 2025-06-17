//! Main client for interacting with the Docaroo API

use crate::{
    error::{DocarooError, Result},
    models::ErrorResponse,
    pricing::PricingClient,
    procedures::ProceduresClient,
};
use bon::Builder;
use reqwest::{Client, Response, StatusCode};
use std::sync::Arc;
use url::Url;

/// Configuration for the Docaroo client
#[derive(Debug, Clone, Builder)]
pub struct DocarooConfig {
    /// API key for authentication
    #[builder(into)]
    pub api_key: String,
    
    /// Base URL for the API (defaults to production)
    #[builder(into, default = crate::API_BASE_URL.to_string())]
    pub base_url: String,
    
    /// HTTP client to use (defaults to new client)
    pub http_client: Option<Client>,
}

/// Main client for interacting with the Docaroo API
#[derive(Debug, Clone)]
pub struct DocarooClient {
    config: Arc<DocarooConfig>,
    http_client: Client,
}

impl DocarooClient {
    /// Create a new Docaroo client with the given API key
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_config(
            DocarooConfig::builder()
                .api_key(api_key)
                .build()
        )
    }

    /// Create a new Docaroo client with custom configuration
    pub fn with_config(config: DocarooConfig) -> Self {
        let http_client = config.http_client.clone().unwrap_or_else(|| {
            Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("Failed to create HTTP client")
        });

        Self {
            config: Arc::new(config),
            http_client,
        }
    }

    /// Get the API key
    pub fn api_key(&self) -> &str {
        &self.config.api_key
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    /// Get the HTTP client
    pub(crate) fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Build a URL for an API endpoint
    pub(crate) fn build_url(&self, endpoint: &str) -> Result<Url> {
        let base = Url::parse(&self.config.base_url)?;
        let mut url = base.join(endpoint)?;
        
        // Add API key as query parameter
        url.query_pairs_mut()
            .append_pair("key", &self.config.api_key);
        
        Ok(url)
    }

    /// Handle API response and convert errors
    pub(crate) async fn handle_response<T>(response: Response) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let status = response.status();
        
        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| DocarooError::ParseError(e.to_string()))
        } else {
            // Try to parse error response
            let error_response = response
                .json::<ErrorResponse>()
                .await
                .unwrap_or_else(|_| ErrorResponse {
                    error: status.as_str().to_string(),
                    message: format!("HTTP {} error", status.as_u16()),
                    details: None,
                    request_id: None,
                    timestamp: None,
                });

            // Map status codes to specific errors
            match status {
                StatusCode::UNAUTHORIZED => {
                    Err(DocarooError::AuthenticationFailed(error_response.message))
                }
                StatusCode::BAD_REQUEST => {
                    Err(DocarooError::InvalidRequest(error_response.message))
                }
                StatusCode::TOO_MANY_REQUESTS => {
                    Err(DocarooError::from_error_response(error_response))
                }
                _ => Err(DocarooError::from_error_response(error_response)),
            }
        }
    }

    /// Create a pricing client for in-network rates operations
    pub fn pricing(&self) -> PricingClient {
        PricingClient::new(self.clone())
    }

    /// Create a procedures client for likelihood operations
    pub fn procedures(&self) -> ProceduresClient {
        ProceduresClient::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = DocarooClient::new("test-api-key");
        assert_eq!(client.api_key(), "test-api-key");
        assert_eq!(client.base_url(), crate::API_BASE_URL);
    }

    #[test]
    fn test_client_with_config() {
        let config = DocarooConfig::builder()
            .api_key("custom-key")
            .base_url("https://custom.api.com")
            .build();
        
        let client = DocarooClient::with_config(config);
        assert_eq!(client.api_key(), "custom-key");
        assert_eq!(client.base_url(), "https://custom.api.com");
    }

    #[test]
    fn test_build_url() {
        let client = DocarooClient::new("test-key");
        let url = client.build_url("/pricing/in-network").unwrap();
        
        assert_eq!(url.path(), "/pricing/in-network");
        assert_eq!(
            url.query_pairs().find(|(k, _)| k == "key").map(|(_, v)| v.into_owned()),
            Some("test-key".to_string())
        );
    }
}