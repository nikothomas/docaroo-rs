//! Example demonstrating how to use the pricing API

use docaroo_rs::{DocarooClient, models::{PricingRequest, CodeType}};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("DOCAROO_API_KEY")
        .expect("Please set DOCAROO_API_KEY environment variable");

    // Create client
    let client = DocarooClient::new(api_key);

    // Example 1: Basic pricing lookup
    println!("Example 1: Basic pricing lookup");
    println!("-------------------------------");
    
    let request = PricingRequest::builder()
        .npis(vec!["1043566623".to_string()])
        .condition_code("99214")
        .build();

    match client.pricing().get_in_network_rates(request).await {
        Ok(response) => {
            println!("Request ID: {}", response.meta.request_id);
            println!("Payer: {}", response.meta.payer);
            println!("Processing time: {}ms", response.meta.processing_time_ms);
            
            for (npi, rates) in &response.data {
                println!("\nNPI {}: {} rates found", npi, rates.len());
                for rate in rates {
                    println!("  Code: {} ({})", rate.code, rate.code_type);
                    println!("  Min: ${:.2}, Max: ${:.2}, Avg: ${:.2}",
                        rate.min_rate, rate.max_rate, rate.avg_rate);
                    println!("  Instances: {}", rate.instances);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(request_id) = e.request_id() {
                eprintln!("Request ID for support: {}", request_id);
            }
        }
    }

    // Example 2: Multiple NPIs with custom plan
    println!("\n\nExample 2: Multiple NPIs with custom plan");
    println!("-----------------------------------------");
    
    let request = PricingRequest::builder()
        .npis(vec!["1043566623".to_string(), "1972767655".to_string()])
        .condition_code("99214")
        .plan_id("942404110")
        .code_type(CodeType::Cpt)
        .build();

    match client.pricing().get_in_network_rates(request).await {
        Ok(response) => {
            println!("Plan ID: {}", response.meta.plan_id);
            println!("Total in-network records: {}", response.meta.in_network_records_count);
            
            for (npi, rates) in &response.data {
                if let Some(first_rate) = rates.first() {
                    println!("\nNPI {}: Avg rate ${:.2}", npi, first_rate.avg_rate);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // Example 3: Different code types
    println!("\n\nExample 3: Different code types");
    println!("-------------------------------");
    
    let code_examples = vec![
        ("99214", CodeType::Cpt, "Office visit"),
        ("J0180", CodeType::Hcpcs, "Injection"),
        ("0260", CodeType::Rc, "IV therapy"),
    ];

    for (code, code_type, description) in code_examples {
        println!("\nLooking up {} - {}", code, description);
        
        let request = PricingRequest::builder()
            .npis(vec!["1043566623".to_string()])
            .condition_code(code)
            .code_type(code_type)
            .build();

        match client.pricing().get_in_network_rates(request).await {
            Ok(response) => {
                if let Some(rates) = response.data.get("1043566623") {
                    if let Some(rate) = rates.first() {
                        println!("  Found rate: ${:.2} avg", rate.avg_rate);
                    } else {
                        println!("  No rates found");
                    }
                } else {
                    println!("  No data for this NPI");
                }
            }
            Err(e) => {
                println!("  Error: {}", e);
            }
        }
    }

    Ok(())
}