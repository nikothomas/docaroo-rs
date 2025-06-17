//! Example demonstrating how to use the procedure likelihood API

use docaroo_rs::{DocarooClient, models::LikelihoodRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get API key from environment variable
    let api_key = env::var("DOCAROO_API_KEY")
        .expect("Please set DOCAROO_API_KEY environment variable");

    // Create client
    let client = DocarooClient::new(api_key);

    // Example 1: Basic likelihood check
    println!("Example 1: Basic likelihood check");
    println!("---------------------------------");
    
    let request = LikelihoodRequest::builder()
        .npis(vec!["1487648176".to_string()])
        .condition_code("99214")
        .code_type("CPT")
        .build();

    match client.procedures().get_likelihood(request).await {
        Ok(response) => {
            println!("Request ID: {}", response.meta.request_id);
            println!("Processing time: {}ms", response.meta.processing_time_ms);
            println!("Out-of-network records analyzed: {}", response.meta.out_of_network_records_count);
            
            for (npi, data) in &response.data {
                println!("\nNPI {}", npi);
                println!("  Code: {} ({})", data.code, data.code_type);
                println!("  Likelihood: {:.1}%", data.likelihood * 100.0);
                
                // Interpret the score
                let interpretation = match data.likelihood {
                    x if x >= 0.8 => "Highly likely to perform this procedure",
                    x if x >= 0.6 => "Likely to perform this procedure",
                    x if x >= 0.4 => "Moderately likely to perform this procedure",
                    x if x >= 0.2 => "Unlikely to perform this procedure",
                    _ => "Very unlikely to perform this procedure",
                };
                println!("  Interpretation: {}", interpretation);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            if let Some(request_id) = e.request_id() {
                eprintln!("Request ID for support: {}", request_id);
            }
        }
    }

    // Example 2: Check multiple providers
    println!("\n\nExample 2: Check multiple providers");
    println!("-----------------------------------");
    
    let npis = vec!["1487648176", "1043566623", "1972767655"];
    
    match client.procedures()
        .check_providers(&npis, "99214", "CPT")
        .await 
    {
        Ok(response) => {
            println!("Checking {} providers for procedure 99214", npis.len());
            
            // Sort by likelihood score
            let mut results: Vec<_> = response.data.iter().collect();
            results.sort_by(|a, b| b.1.likelihood.partial_cmp(&a.1.likelihood).unwrap());
            
            println!("\nRanked by likelihood:");
            for (i, (npi, data)) in results.iter().enumerate() {
                println!("{}. NPI {}: {:.1}%", 
                    i + 1, npi, data.likelihood * 100.0);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    // Example 3: Check different procedures for a provider
    println!("\n\nExample 3: Check different procedures for a provider");
    println!("---------------------------------------------------");
    
    let npi = "1487648176";
    let procedures = vec![
        ("99213", "Office visit - Low complexity"),
        ("99214", "Office visit - Moderate complexity"),
        ("99215", "Office visit - High complexity"),
        ("90834", "Psychotherapy - 45 minutes"),
        ("20610", "Arthrocentesis"),
    ];

    println!("Provider NPI: {}", npi);
    
    for (code, description) in procedures {
        let request = LikelihoodRequest::builder()
            .npis(vec![npi.to_string()])
            .condition_code(code)
            .code_type("CPT")
            .build();

        match client.procedures().get_likelihood(request).await {
            Ok(response) => {
                if let Some(data) = response.data.get(npi) {
                    println!("\n{} ({}): {:.1}%", 
                        description, code, data.likelihood * 100.0);
                }
            }
            Err(e) => {
                println!("\nError checking {} ({}): {}", description, code, e);
            }
        }
    }

    Ok(())
}