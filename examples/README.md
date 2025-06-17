# Docaroo SDK Examples

This directory contains examples demonstrating how to use the Docaroo Rust SDK.

## Running the Examples

First, set your API key as an environment variable:

```bash
export DOCAROO_API_KEY="your-api-key-here"
```

Then run any example:

```bash
# Pricing API example
cargo run --example pricing

# Procedure likelihood API example
cargo run --example likelihood

# Error handling example
cargo run --example error_handling
```

## Examples Overview

### pricing.rs
Demonstrates how to use the pricing API to look up in-network contracted rates:
- Basic single NPI lookup
- Multiple NPIs with custom plan
- Different medical code types (CPT, HCPCS, Revenue Codes)

### likelihood.rs
Shows how to use the procedure likelihood API:
- Basic likelihood check for a provider
- Checking multiple providers for the same procedure
- Checking different procedures for a single provider
- Interpreting likelihood scores

### error_handling.rs
Illustrates proper error handling and retry logic:
- Handling invalid request errors
- Authentication errors
- Rate limiting with automatic retry
- Comprehensive error information extraction

## Getting an API Key

To use these examples with real data, you'll need an API key from Docaroo. Contact support@docaroo.com to request access.

## Common Patterns

### Using the Builder Pattern

The SDK uses the `bon` crate for ergonomic builder patterns:

```rust
let request = PricingRequest::builder()
    .npi("1043566623")
    .npi("1972767655")  // Can add multiple NPIs
    .condition_code("99214")
    .plan_id("942404110")
    .code_type(CodeType::Cpt)
    .build();
```

### Error Handling

Always handle errors appropriately:

```rust
match client.pricing().get_in_network_rates(request).await {
    Ok(response) => {
        // Process successful response
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        
        // Get request ID for support if available
        if let Some(request_id) = e.request_id() {
            eprintln!("Request ID: {}", request_id);
        }
        
        // Check if error is retryable
        if e.is_retryable() {
            // Implement retry logic
        }
    }
}
```

### Working with Response Data

Responses contain both data and metadata:

```rust
// Pricing response
for (npi, rates) in response.data {
    for rate in rates {
        println!("NPI {}: ${:.2} average", npi, rate.avg_rate);
    }
}

// Access metadata
println!("Request ID: {}", response.meta.request_id);
println!("Processing time: {}ms", response.meta.processing_time_ms);
```