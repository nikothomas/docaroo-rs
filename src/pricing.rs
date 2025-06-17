//! Pricing API operations for in-network contracted rates

use crate::{
    client::DocarooClient,
    error::Result,
    models::{PricingRequest, PricingResponse},
};

/// Client for pricing-related operations
#[derive(Debug, Clone)]
pub struct PricingClient {
    client: DocarooClient,
}

impl PricingClient {
    /// Create a new pricing client
    pub(crate) fn new(client: DocarooClient) -> Self {
        Self { client }
    }

    /// Get in-network contracted rates for healthcare providers
    ///
    /// Retrieve contracted rates for healthcare providers (NPIs) for specific billing codes
    /// and insurance plans. This endpoint supports bulk lookups for up to 10 NPIs per request.
    ///
    /// # Arguments
    ///
    /// * `request` - The pricing request containing NPIs, billing code, and optional plan ID
    ///
    /// # Returns
    ///
    /// A `PricingResponse` containing rate data organized by NPI and response metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request contains invalid parameters (e.g., invalid NPI format)
    /// - Authentication fails (invalid API key)
    /// - Rate limits are exceeded
    /// - The API returns an error response
    ///
    /// # Example
    ///
    /// ```no_run
    /// use docaroo_rs::{DocarooClient, models::PricingRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DocarooClient::new("your-api-key");
    /// 
    /// let request = PricingRequest::builder()
    ///     .npis(vec!["1043566623".to_string(), "1972767655".to_string()])
    ///     .condition_code("99214")
    ///     .plan_id("942404110")
    ///     .build();
    ///
    /// let response = client.pricing().get_in_network_rates(request).await?;
    ///
    /// // Process the response
    /// for (npi, rates) in response.data {
    ///     println!("NPI {}: {} rates found", npi, rates.len());
    ///     for rate in rates {
    ///         println!("  - Min: ${:.2}, Max: ${:.2}, Avg: ${:.2}",
    ///             rate.min_rate, rate.max_rate, rate.avg_rate);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_in_network_rates(&self, request: PricingRequest) -> Result<PricingResponse> {
        // Validate request
        self.validate_pricing_request(&request)?;

        // Build URL
        let url = self.client.build_url("/pricing/in-network")?;

        // Send request
        let response = self
            .client
            .http_client()
            .post(url)
            .json(&request)
            .send()
            .await?;

        // Handle response
        DocarooClient::handle_response(response).await
    }

    /// Validate a pricing request before sending
    fn validate_pricing_request(&self, request: &PricingRequest) -> Result<()> {
        use crate::error::DocarooError;

        // Validate NPIs count
        if request.npis.is_empty() {
            return Err(DocarooError::InvalidRequest(
                "At least one NPI must be provided".to_string(),
            ));
        }

        if request.npis.len() > 10 {
            return Err(DocarooError::InvalidRequest(
                "Maximum 10 NPIs allowed per request".to_string(),
            ));
        }

        // Validate NPI format (10 digits)
        for npi in &request.npis {
            if npi.len() != 10 || !npi.chars().all(|c| c.is_ascii_digit()) {
                return Err(DocarooError::InvalidRequest(format!(
                    "Invalid NPI format: '{}'. NPIs must be 10-digit numbers",
                    npi
                )));
            }
        }

        // Validate condition code is not empty
        if request.condition_code.trim().is_empty() {
            return Err(DocarooError::InvalidRequest(
                "Condition code cannot be empty".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_pricing_request_valid() {
        let client = DocarooClient::new("test-key");
        let pricing_client = PricingClient::new(client);

        let request = PricingRequest::builder()
            .npis(vec!["1234567890".to_string()])
            .condition_code("99214")
            .build();

        assert!(pricing_client.validate_pricing_request(&request).is_ok());
    }

    #[test]
    fn test_validate_pricing_request_empty_npis() {
        let client = DocarooClient::new("test-key");
        let pricing_client = PricingClient::new(client);

        let request = PricingRequest {
            npis: vec![],
            condition_code: "99214".to_string(),
            plan_id: None,
            code_type: None,
        };

        let result = pricing_client.validate_pricing_request(&request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("At least one NPI must be provided"));
    }

    #[test]
    fn test_validate_pricing_request_too_many_npis() {
        let client = DocarooClient::new("test-key");
        let pricing_client = PricingClient::new(client);

        let npis: Vec<String> = (0..11).map(|i| format!("{:010}", i)).collect();
        let request = PricingRequest {
            npis,
            condition_code: "99214".to_string(),
            plan_id: None,
            code_type: None,
        };

        let result = pricing_client.validate_pricing_request(&request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Maximum 10 NPIs allowed"));
    }

    #[test]
    fn test_validate_pricing_request_invalid_npi_format() {
        let client = DocarooClient::new("test-key");
        let pricing_client = PricingClient::new(client);

        let request = PricingRequest::builder()
            .npis(vec!["123".to_string()]) // Too short
            .condition_code("99214")
            .build();

        let result = pricing_client.validate_pricing_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid NPI format"));
    }
}