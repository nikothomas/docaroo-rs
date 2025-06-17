# docaroo-rs

A Rust SDK for the Docaroo Care Navigation Data API, providing healthcare provider pricing discovery and procedure likelihood analysis.

## Features

- **In-network Pricing Lookup**: Get contracted rates for healthcare providers (NPIs) for specific billing codes and insurance plans
- **Procedure Likelihood Scoring**: Evaluate the likelihood that providers perform specific medical procedures (0.0 to 1.0 confidence scores)
- **Bulk Operations**: Support for up to 10 NPIs per pricing request
- **Multiple Code Types**: Support for CPT, NDC, HCPCS, ICD, DRG variants, and more
- **Async/Await**: Built with Tokio for efficient async operations
- **Type-Safe**: Strongly typed request/response models with builder patterns
- **Comprehensive Error Handling**: Detailed error types with retry support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
docaroo-rs = "0.0.1"
```

## Quick Start

```rust
use docaroo_rs::{DocarooClient, models::PricingRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with your API key
    let client = DocarooClient::new("your-api-key");
    
    // Look up pricing for a provider
    let request = PricingRequest::builder()
        .npis(vec!["1043566623"])
        .condition_code("99214")
        .build();
    
    let response = client.pricing().get_in_network_rates(request).await?;
    
    // Process the response
    for (npi, rates) in response.data {
        println!("NPI {}: {} rates found", npi, rates.len());
        for rate in rates {
            println!("  Average rate: ${:.2}", rate.avg_rate);
        }
    }
    
    Ok(())
}
```

## API Overview

### Pricing API

Look up in-network contracted rates for healthcare providers:

```rust
use docaroo_rs::models::{PricingRequest, CodeType};

let request = PricingRequest::builder()
    .npis(vec!["1043566623", "1972767655"])  // Can add multiple NPIs (up to 10)
    .condition_code("99214")
    .plan_id("942404110")  // Optional, defaults to "942404110"
    .code_type(CodeType::Cpt)  // Optional, defaults to CPT
    .build();

let response = client.pricing().get_in_network_rates(request).await?;
```

### Procedure Likelihood API

Evaluate the likelihood that providers perform specific procedures:

```rust
use docaroo_rs::models::LikelihoodRequest;

let request = LikelihoodRequest::builder()
    .npis(vec!["1487648176"])
    .condition_code("99214")
    .code_type("CPT")
    .build();

let response = client.procedures().get_likelihood(request).await?;

// Or use the convenience method
let response = client.procedures()
    .check_providers(&["1487648176", "1234567890"], "99214", "CPT")
    .await?;
```

## Configuration

### Custom Configuration

```rust
use docaroo_rs::client::DocarooConfig;

let config = DocarooConfig::builder()
    .api_key("your-api-key")
    .base_url("https://custom-api-url.com")  // Optional custom URL
    .http_client(custom_client)  // Optional custom reqwest client
    .build();

let client = DocarooClient::with_config(config);
```

### Environment Variables

The examples use environment variables for API keys:

```bash
export DOCAROO_API_KEY="your-api-key"
cargo run --example pricing
```

## Error Handling

The SDK provides comprehensive error handling:

```rust
use docaroo_rs::DocarooError;

match client.pricing().get_in_network_rates(request).await {
    Ok(response) => {
        // Handle success
    }
    Err(e) => {
        match e {
            DocarooError::RateLimitExceeded { retry_after } => {
                println!("Rate limited. Retry after {} seconds", retry_after);
            }
            DocarooError::AuthenticationFailed(msg) => {
                println!("Auth failed: {}", msg);
            }
            DocarooError::InvalidRequest(msg) => {
                println!("Invalid request: {}", msg);
            }
            _ => {
                println!("Error: {}", e);
            }
        }
        
        // Check if error is retryable
        if e.is_retryable() {
            // Implement retry logic
        }
        
        // Get request ID for support
        if let Some(request_id) = e.request_id() {
            println!("Request ID: {}", request_id);
        }
    }
}
```

## Medical Code Types

The SDK supports all medical billing code standards used by the API:

- **CPT**: Current Procedural Terminology
- **NDC**: National Drug Code
- **HCPCS**: Healthcare Common Procedure Coding System
- **ICD**: International Classification of Diseases
- **DRG variants**: MS-DRG, R-DRG, S-DRG, APS-DRG, AP-DRG, APR-DRG
- **APC**: Ambulatory Payment Classification
- **CDT**: Current Dental Terminology
- And more...

## Examples

See the [examples](examples/) directory for detailed usage examples:

- [pricing.rs](examples/pricing.rs) - Pricing API usage
- [likelihood.rs](examples/likelihood.rs) - Procedure likelihood API usage
- [error_handling.rs](examples/error_handling.rs) - Error handling and retry logic

## API Documentation

For detailed SDK documentation, run:

```bash
cargo doc --open
```

For the Docaroo API documentation and details about the API endpoints, visit the [official Docaroo documentation](https://docs.docaroo.com/).

## Author

This Rust client was developed by Nikolas Yanek-Chrones.

- GitHub: [@nikothomas](https://github.com/nikothomas)
- Email: nik@sunnyhealth.ai

## Support

- For API access and questions: support@docaroo.com
- For SDK issues: [GitHub Issues](https://github.com/nikothomas/docaroo-rs/issues)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.