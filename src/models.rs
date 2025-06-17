//! Data models for the Docaroo API

use bon::Builder;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Medical billing code types supported by the API
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum CodeType {
    /// Current Procedural Terminology
    #[serde(rename = "CPT")]
    Cpt,
    /// National Drug Code
    #[serde(rename = "NDC")]
    Ndc,
    /// Healthcare Common Procedure Coding System
    #[serde(rename = "HCPCS")]
    Hcpcs,
    /// Revenue Code
    #[serde(rename = "RC")]
    Rc,
    /// International Classification of Diseases
    #[serde(rename = "ICD")]
    Icd,
    /// Medicare Severity Diagnosis Related Group
    #[serde(rename = "MS-DRG")]
    MsDrg,
    /// Refined Diagnosis Related Group
    #[serde(rename = "R-DRG")]
    RDrg,
    /// Severity Diagnosis Related Group
    #[serde(rename = "S-DRG")]
    SDrg,
    /// All Patient Severity Diagnosis Related Group
    #[serde(rename = "APS-DRG")]
    ApsDrg,
    /// All Patient Diagnosis Related Group
    #[serde(rename = "AP-DRG")]
    ApDrg,
    /// All Patient Refined Diagnosis Related Group
    #[serde(rename = "APR-DRG")]
    AprDrg,
    /// Ambulatory Payment Classification
    #[serde(rename = "APC")]
    Apc,
    /// Local code
    #[serde(rename = "LOCAL")]
    Local,
    /// Enhanced Ambulatory Patient Grouping
    #[serde(rename = "EAPG")]
    Eapg,
    /// Health Insurance Prospective Payment System
    #[serde(rename = "HIPPS")]
    Hipps,
    /// Current Dental Terminology
    #[serde(rename = "CDT")]
    Cdt,
    /// Custom All
    #[serde(rename = "CSTM-ALL")]
    CstmAll,
}

impl Default for CodeType {
    fn default() -> Self {
        Self::Cpt
    }
}

/// Request for in-network pricing lookup
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct PricingRequest {
    /// List of National Provider Identifiers (NPIs) to lookup pricing for
    /// Must be 10-digit identifiers, 1-10 items allowed
    #[builder(into)]
    pub npis: Vec<String>,
    
    /// Medical billing code to retrieve pricing for
    #[builder(into)]
    pub condition_code: String,
    
    /// Insurance plan identifier (EIN, HIOS ID, or Custom Plan ID)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(into)]
    pub plan_id: Option<String>,
    
    /// Medical billing code standard
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_type: Option<CodeType>,
}

/// Request for procedure likelihood evaluation
#[derive(Debug, Clone, Serialize, Builder)]
#[serde(rename_all = "camelCase")]
pub struct LikelihoodRequest {
    /// List of National Provider Identifiers (NPIs) to evaluate
    #[builder(into)]
    pub npis: Vec<String>,
    
    /// Medical billing code to evaluate likelihood for
    #[builder(into)]
    pub condition_code: String,
    
    /// Medical billing code standard
    #[builder(into)]
    pub code_type: String,
}

/// Response containing pricing data
#[derive(Debug, Clone, Deserialize)]
pub struct PricingResponse {
    /// Pricing data organized by NPI
    pub data: HashMap<String, Vec<RateData>>,
    /// Response metadata
    pub meta: PricingMeta,
}

/// Response containing likelihood scores
#[derive(Debug, Clone, Deserialize)]
pub struct LikelihoodResponse {
    /// Likelihood scores organized by NPI
    pub data: HashMap<String, LikelihoodData>,
    /// Response metadata
    pub meta: LikelihoodMeta,
}

/// Rate data for a specific billing code
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateData {
    /// Medical billing code
    pub code: String,
    /// Medical billing code standard
    pub code_type: String,
    /// Type of negotiated rate
    pub negotiated_type: String,
    /// Minimum contracted rate
    pub min_rate: f64,
    /// Maximum contracted rate
    pub max_rate: f64,
    /// Average contracted rate
    pub avg_rate: f64,
    /// Number of rate instances found
    pub instances: u32,
}

/// Likelihood data for a specific billing code
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikelihoodData {
    /// Medical billing code
    pub code: String,
    /// Medical billing code standard
    pub code_type: String,
    /// Likelihood score from 0.0 (unlikely) to 1.0 (highly likely)
    pub likelihood: f64,
}

/// Metadata for pricing responses
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricingMeta {
    /// Insurance plan identifier
    pub plan_id: String,
    /// Insurance payer code
    pub payer: String,
    /// Unique request identifier
    pub request_id: String,
    /// Request timestamp in ISO 8601 format
    pub timestamp: DateTime<Utc>,
    /// Processing time in milliseconds
    pub processing_time_ms: u32,
    /// Number of in-network records found
    pub in_network_records_count: u32,
}

/// Metadata for likelihood responses
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LikelihoodMeta {
    /// Unique request identifier
    pub request_id: String,
    /// Request timestamp in ISO 8601 format
    pub timestamp: DateTime<Utc>,
    /// Processing time in milliseconds
    pub processing_time_ms: u32,
    /// Number of out-of-network records analyzed
    pub out_of_network_records_count: u32,
}

/// Error response from the API
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    /// Error type
    pub error: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Request identifier for support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// Error timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pricing_request_builder() {
        let request = PricingRequest::builder()
            .npis(vec![String::from("1043566623"), String::from("1972767655")])
            .condition_code("99214")
            .plan_id("942404110")
            .code_type(CodeType::Cpt)
            .build();

        assert_eq!(request.npis.len(), 2);
        assert_eq!(request.condition_code, "99214");
        assert_eq!(request.plan_id, Some("942404110".to_string()));
        assert_eq!(request.code_type, Some(CodeType::Cpt));
    }

    #[test]
    fn test_likelihood_request_builder() {
        let request = LikelihoodRequest::builder()
            .npis(vec!["1487648176".to_string()])
            .condition_code("99214")
            .code_type("CPT")
            .build();

        assert_eq!(request.npis.len(), 1);
        assert_eq!(request.condition_code, "99214");
        assert_eq!(request.code_type, "CPT");
    }

    #[test]
    fn test_code_type_serialization() {
        let code_type = CodeType::Cpt;
        let json = serde_json::to_string(&code_type).unwrap();
        assert_eq!(json, r#""CPT""#);

        let deserialized: CodeType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, CodeType::Cpt);
    }
}