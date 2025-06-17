//! Procedures API operations for likelihood scoring

use crate::{
    client::DocarooClient,
    error::Result,
    models::{LikelihoodRequest, LikelihoodResponse},
};

/// Client for procedure likelihood operations
#[derive(Debug, Clone)]
pub struct ProceduresClient {
    client: DocarooClient,
}

impl ProceduresClient {
    /// Create a new procedures client
    pub(crate) fn new(client: DocarooClient) -> Self {
        Self { client }
    }

    /// Get procedure likelihood scores for healthcare providers
    ///
    /// Evaluate the likelihood that healthcare providers (NPIs) perform specific medical
    /// procedures or services. The API analyzes historical claims data and provider
    /// specialties to generate confidence scores from 0.0 (unlikely) to 1.0 (highly likely).
    ///
    /// # Arguments
    ///
    /// * `request` - The likelihood request containing NPIs, billing code, and code type
    ///
    /// # Returns
    ///
    /// A `LikelihoodResponse` containing likelihood scores organized by NPI and metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The request contains invalid parameters
    /// - Authentication fails (invalid API key)
    /// - Rate limits are exceeded
    /// - The API returns an error response
    ///
    /// # Example
    ///
    /// ```no_run
    /// use docaroo_rs::{DocarooClient, models::LikelihoodRequest};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DocarooClient::new("your-api-key");
    /// 
    /// let request = LikelihoodRequest::builder()
    ///     .npis(vec!["1487648176".to_string()])
    ///     .condition_code("99214")
    ///     .code_type("CPT")
    ///     .build();
    ///
    /// let response = client.procedures().get_likelihood(request).await?;
    ///
    /// // Process the response
    /// for (npi, data) in response.data {
    ///     println!("NPI {}: Likelihood score = {:.2}", npi, data.likelihood);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_likelihood(&self, request: LikelihoodRequest) -> Result<LikelihoodResponse> {
        // Validate request
        self.validate_likelihood_request(&request)?;

        // Build URL
        let url = self.client.build_url("/procedures/likelihood")?;

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

    /// Validate a likelihood request before sending
    fn validate_likelihood_request(&self, request: &LikelihoodRequest) -> Result<()> {
        use crate::error::DocarooError;

        // Validate NPIs
        if request.npis.is_empty() {
            return Err(DocarooError::InvalidRequest(
                "At least one NPI must be provided".to_string(),
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

        // Validate code type is not empty
        if request.code_type.trim().is_empty() {
            return Err(DocarooError::InvalidRequest(
                "Code type cannot be empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Check multiple providers for a procedure at once
    ///
    /// This is a convenience method that allows checking multiple providers
    /// for the same procedure in a single request.
    ///
    /// # Arguments
    ///
    /// * `npis` - List of National Provider Identifiers
    /// * `condition_code` - Medical billing code
    /// * `code_type` - Medical billing code standard (e.g., "CPT")
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use docaroo_rs::DocarooClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = DocarooClient::new("your-api-key");
    /// 
    /// let npis = vec!["1487648176", "1234567890"];
    /// let response = client.procedures()
    ///     .check_providers(&npis, "99214", "CPT")
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_providers(
        &self,
        npis: &[&str],
        condition_code: impl Into<String>,
        code_type: impl Into<String>,
    ) -> Result<LikelihoodResponse> {
        let request = LikelihoodRequest::builder()
            .npis(npis.iter().map(|&s| s.to_string()).collect::<Vec<_>>())
            .condition_code(condition_code)
            .code_type(code_type)
            .build();

        self.get_likelihood(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_likelihood_request_valid() {
        let client = DocarooClient::new("test-key");
        let procedures_client = ProceduresClient::new(client);

        let request = LikelihoodRequest::builder()
            .npis(vec![String::from("1234567890")])
            .condition_code("99214")
            .code_type("CPT")
            .build();

        assert!(procedures_client.validate_likelihood_request(&request).is_ok());
    }

    #[test]
    fn test_validate_likelihood_request_empty_npis() {
        let client = DocarooClient::new("test-key");
        let procedures_client = ProceduresClient::new(client);

        let request = LikelihoodRequest {
            npis: vec![],
            condition_code: "99214".to_string(),
            code_type: "CPT".to_string(),
        };

        let result = procedures_client.validate_likelihood_request(&request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("At least one NPI must be provided"));
    }

    #[test]
    fn test_validate_likelihood_request_invalid_npi() {
        let client = DocarooClient::new("test-key");
        let procedures_client = ProceduresClient::new(client);

        let request = LikelihoodRequest::builder()
            .npis(vec![String::from("ABC1234567")]) // Contains letters
            .condition_code("99214")
            .code_type("CPT")
            .build();

        let result = procedures_client.validate_likelihood_request(&request);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid NPI format"));
    }

    #[test]
    fn test_validate_likelihood_request_empty_code_type() {
        let client = DocarooClient::new("test-key");
        let procedures_client = ProceduresClient::new(client);

        let request = LikelihoodRequest {
            npis: vec!["1234567890".to_string()],
            condition_code: "99214".to_string(),
            code_type: "".to_string(),
        };

        let result = procedures_client.validate_likelihood_request(&request);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Code type cannot be empty"));
    }
}