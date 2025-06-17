//! Example demonstrating error handling and retry logic

use docaroo_rs::{
    DocarooClient, 
    DocarooError,
    models::PricingRequest
};
use std::{env, time::Duration};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("DOCAROO_API_KEY")
        .unwrap_or_else(|_| "demo-key-for-testing".to_string());

    // Create client
    let client = DocarooClient::new(api_key);

    // Example 1: Handle invalid request
    println!("Example 1: Handle invalid request");
    println!("---------------------------------");
    
    let invalid_request = PricingRequest::builder()
        .npis(vec!["123".to_string()]) // Invalid NPI (too short)
        .condition_code("99214")
        .build();

    match client.pricing().get_in_network_rates(invalid_request).await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            println!("Expected error occurred: {}", e);
            
            // Check error type
            match &e {
                DocarooError::InvalidRequest(msg) => {
                    println!("Invalid request details: {}", msg);
                }
                _ => println!("Different error type: {:?}", e),
            }
        }
    }

    // Example 2: Handle authentication error
    println!("\n\nExample 2: Handle authentication error");
    println!("-------------------------------------");
    
    let bad_client = DocarooClient::new("invalid-api-key");
    let request = PricingRequest::builder()
        .npis(vec!["1043566623".to_string()])
        .condition_code("99214")
        .build();

    match bad_client.pricing().get_in_network_rates(request.clone()).await {
        Ok(_) => println!("Unexpected success"),
        Err(e) => {
            match &e {
                DocarooError::AuthenticationFailed(msg) => {
                    println!("Authentication failed: {}", msg);
                    println!("Action: Check your API key");
                }
                DocarooError::ApiError { code, message, request_id } => {
                    println!("API error ({}): {}", code, message);
                    if let Some(id) = request_id {
                        println!("Request ID for support: {}", id);
                    }
                }
                _ => println!("Error: {}", e),
            }
        }
    }

    // Example 3: Retry logic for transient errors
    println!("\n\nExample 3: Retry logic for transient errors");
    println!("------------------------------------------");
    
    let request = PricingRequest::builder()
        .npis(vec!["1043566623".to_string()])
        .condition_code("99214")
        .build();

    async fn call_with_retry(
        client: &DocarooClient,
        request: PricingRequest,
        max_retries: u32,
    ) -> Result<(), DocarooError> {
        let mut retries = 0;
        
        loop {
            match client.pricing().get_in_network_rates(request.clone()).await {
                Ok(response) => {
                    println!("Success! Found {} NPIs with data", response.data.len());
                    return Ok(());
                }
                Err(e) => {
                    if e.is_retryable() && retries < max_retries {
                        retries += 1;
                        
                        let delay = match &e {
                            DocarooError::RateLimitExceeded { retry_after } => {
                                println!("Rate limit hit. Waiting {} seconds...", retry_after);
                                Duration::from_secs(*retry_after)
                            }
                            _ => {
                                let backoff = Duration::from_secs(2u64.pow(retries));
                                println!("Retryable error: {}. Retry {} of {} in {:?}...", 
                                    e, retries, max_retries, backoff);
                                backoff
                            }
                        };
                        
                        sleep(delay).await;
                        continue;
                    } else {
                        println!("Non-retryable error or max retries reached: {}", e);
                        return Err(e);
                    }
                }
            }
        }
    }

    let _ = call_with_retry(&client, request, 3).await;

    // Example 4: Comprehensive error information
    println!("\n\nExample 4: Comprehensive error information");
    println!("-----------------------------------------");
    
    // Try various invalid requests
    let test_cases = vec![
        (
            PricingRequest::builder()
                .npis(vec![]) // Empty NPIs
                .condition_code("99214")
                .build(),
            "Empty NPIs list"
        ),
        (
            PricingRequest::builder()
                .npis((0..15).map(|i| format!("{:010}", i)).collect::<Vec<String>>()) // Too many NPIs
                .condition_code("99214")
                .build(),
            "Too many NPIs"
        ),
        (
            PricingRequest::builder()
                .npis(vec!["ABCDEFGHIJ".to_string()]) // Non-numeric NPI
                .condition_code("99214")
                .build(),
            "Non-numeric NPI"
        ),
    ];

    for (request, description) in test_cases {
        println!("\nTesting: {}", description);
        match client.pricing().get_in_network_rates(request).await {
            Ok(_) => println!("  Unexpected success"),
            Err(e) => {
                println!("  Error: {}", e);
                
                // Additional error details
                if let Some(request_id) = e.request_id() {
                    println!("  Request ID: {}", request_id);
                }
                println!("  Is retryable: {}", e.is_retryable());
            }
        }
    }

    Ok(())
}