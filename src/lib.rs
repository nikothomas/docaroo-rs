//! # Docaroo Care Navigation Data API Client
//! 
//! A Rust client library for the Docaroo Care Navigation Data API, providing
//! healthcare provider pricing discovery and procedure likelihood analysis.
//! 
//! This client was developed by Nikolas Yanek-Chrones (<https://github.com/nikothomas>).
//! 
//! ## Features
//! 
//! - In-network contracted rates lookup for healthcare providers (NPIs)
//! - Procedure likelihood scoring for medical procedures
//! - Support for multiple medical billing code types (CPT, NDC, HCPCS, etc.)
//! - Bulk NPI lookups (up to 10 NPIs per request)
//! - Built with async/await support using Tokio
//! 
//! ## Usage
//! 
//! ```no_run
//! use docaroo_rs::{DocarooClient, models::PricingRequest};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = DocarooClient::new("your-api-key");
//!     
//!     let request = PricingRequest::builder()
//!         .npis(vec!["1043566623".to_string(), "1972767655".to_string()])
//!         .condition_code("99214")
//!         .plan_id("942404110")
//!         .build();
//!     
//!     let response = client.pricing().get_in_network_rates(request).await?;
//!     
//!     for (npi, rates) in response.data {
//!         println!("NPI {}: {} rates found", npi, rates.len());
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod error;
pub mod models;
pub mod pricing;
pub mod procedures;

pub use client::DocarooClient;
pub use error::{DocarooError, Result};

/// The base URL for the Docaroo API
pub const API_BASE_URL: &str = "https://care-navigation-gateway-ccg16t89.wl.gateway.dev";

/// Re-export commonly used types
pub mod prelude {
    pub use crate::{
        client::DocarooClient,
        error::{DocarooError, Result},
        models::{
            CodeType, LikelihoodRequest, LikelihoodResponse, PricingRequest, PricingResponse,
        },
    };
}