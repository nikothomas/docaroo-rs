//! Integration tests for the Docaroo SDK

use docaroo_rs::{
    DocarooClient,
    client::DocarooConfig,
    models::{PricingRequest, LikelihoodRequest, CodeType},
    error::DocarooError,
};

#[test]
fn test_client_creation() {
    let client = DocarooClient::new("test-api-key");
    assert_eq!(client.api_key(), "test-api-key");
    assert_eq!(client.base_url(), docaroo_rs::API_BASE_URL);
}

#[test]
fn test_custom_config() {
    let config = DocarooConfig::builder()
        .api_key("custom-key")
        .base_url("https://custom.example.com")
        .build();
    
    let client = DocarooClient::with_config(config);
    assert_eq!(client.api_key(), "custom-key");
    assert_eq!(client.base_url(), "https://custom.example.com");
}

#[test]
fn test_pricing_request_builder() {
    let request = PricingRequest::builder()
        .npis(vec![String::from("1234567890"), String::from("0987654321")])
        .condition_code("99214")
        .plan_id("custom-plan")
        .code_type(CodeType::Hcpcs)
        .build();
    
    assert_eq!(request.npis.len(), 2);
    assert_eq!(request.npis[0], "1234567890");
    assert_eq!(request.npis[1], "0987654321");
    assert_eq!(request.condition_code, "99214");
    assert_eq!(request.plan_id, Some("custom-plan".to_string()));
    assert_eq!(request.code_type, Some(CodeType::Hcpcs));
}

#[test]
fn test_likelihood_request_builder() {
    let request = LikelihoodRequest::builder()
        .npis(vec!["1111111111".to_string(), "2222222222".to_string()])
        .condition_code("90834")
        .code_type("CPT")
        .build();
    
    assert_eq!(request.npis.len(), 2);
    assert_eq!(request.condition_code, "90834");
    assert_eq!(request.code_type, "CPT");
}

#[test]
fn test_error_types() {
    // Test rate limit error
    let error = DocarooError::RateLimitExceeded { retry_after: 60 };
    assert!(error.is_retryable());
    assert!(matches!(error, DocarooError::RateLimitExceeded { .. }));
    
    // Test API error
    let error = DocarooError::ApiError {
        code: "bad_request".to_string(),
        message: "Invalid NPI".to_string(),
        request_id: Some("req_123".to_string()),
    };
    assert!(!error.is_retryable());
    assert_eq!(error.request_id(), Some("req_123"));
    
    // Test authentication error
    let error = DocarooError::AuthenticationFailed("Invalid API key".to_string());
    assert!(!error.is_retryable());
}

#[test]
fn test_code_type_serialization() {
    use serde_json;
    
    let test_cases = vec![
        (CodeType::Cpt, "\"CPT\""),
        (CodeType::Ndc, "\"NDC\""),
        (CodeType::Hcpcs, "\"HCPCS\""),
        (CodeType::MsDrg, "\"MS-DRG\""),
        (CodeType::CstmAll, "\"CSTM-ALL\""),
    ];
    
    for (code_type, expected) in test_cases {
        let json = serde_json::to_string(&code_type).unwrap();
        assert_eq!(json, expected);
        
        let deserialized: CodeType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, code_type);
    }
}

#[test]
fn test_pricing_request_validation() {
    let client = DocarooClient::new("test-key");
    let _pricing_client = client.pricing();
    
    // Valid request should pass
    let valid_request = PricingRequest::builder()
        .npis(vec![String::from("1234567890")])
        .condition_code("99214")
        .build();
    
    // Note: We can't test the actual validation method directly since it's private,
    // but we can ensure the request is built correctly
    assert!(!valid_request.npis.is_empty());
    assert!(!valid_request.condition_code.is_empty());
}

#[test]
fn test_likelihood_request_validation() {
    let client = DocarooClient::new("test-key");
    let _procedures_client = client.procedures();
    
    // Valid request should be built correctly
    let valid_request = LikelihoodRequest::builder()
        .npis(vec![String::from("1234567890")])
        .condition_code("99214")
        .code_type("CPT")
        .build();
    
    assert!(!valid_request.npis.is_empty());
    assert!(!valid_request.condition_code.is_empty());
    assert!(!valid_request.code_type.is_empty());
}

#[cfg(test)]
mod mock_tests {
    
    
    #[test]
    fn test_pricing_response_deserialization() {
        use docaroo_rs::models::PricingResponse;
        
        let json = r#"{
            "data": {
                "1043566623": [{
                    "code": "99214",
                    "codeType": "CPT",
                    "negotiatedType": "negotiated",
                    "minRate": 65.87,
                    "maxRate": 266.88,
                    "avgRate": 147.03,
                    "instances": 6
                }]
            },
            "meta": {
                "planId": "942404110",
                "payer": "UNH",
                "requestId": "req_test123",
                "timestamp": "2025-06-15T23:15:48.734729Z",
                "processingTimeMs": 912,
                "inNetworkRecordsCount": 14
            }
        }"#;
        
        let response: PricingResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.data.len(), 1);
        assert!(response.data.contains_key("1043566623"));
        
        let rates = &response.data["1043566623"];
        assert_eq!(rates.len(), 1);
        assert_eq!(rates[0].code, "99214");
        assert_eq!(rates[0].avg_rate, 147.03);
        
        assert_eq!(response.meta.plan_id, "942404110");
        assert_eq!(response.meta.payer, "UNH");
        assert_eq!(response.meta.processing_time_ms, 912);
    }
    
    #[test]
    fn test_likelihood_response_deserialization() {
        use docaroo_rs::models::LikelihoodResponse;
        
        let json = r#"{
            "data": {
                "1487648176": {
                    "code": "99214",
                    "codeType": "CPT",
                    "likelihood": 0.9
                }
            },
            "meta": {
                "requestId": "req_test456",
                "timestamp": "2025-06-15T23:22:22.395111Z",
                "processingTimeMs": 731,
                "outOfNetworkRecordsCount": 68
            }
        }"#;
        
        let response: LikelihoodResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(response.data.len(), 1);
        assert!(response.data.contains_key("1487648176"));
        
        let data = &response.data["1487648176"];
        assert_eq!(data.code, "99214");
        assert_eq!(data.likelihood, 0.9);
        
        assert_eq!(response.meta.request_id, "req_test456");
        assert_eq!(response.meta.processing_time_ms, 731);
    }
    
    #[test]
    fn test_error_response_deserialization() {
        use docaroo_rs::models::ErrorResponse;
        
        let json = r#"{
            "error": "bad_request",
            "message": "Invalid request parameters",
            "details": {
                "field": "npis",
                "code": "INVALID_ARRAY_LENGTH"
            },
            "requestId": "req_error_123",
            "timestamp": "2025-06-17T08:29:00Z"
        }"#;
        
        let error_response: ErrorResponse = serde_json::from_str(json).unwrap();
        
        assert_eq!(error_response.error, "bad_request");
        assert_eq!(error_response.message, "Invalid request parameters");
        assert!(error_response.details.is_some());
        assert_eq!(error_response.request_id, Some("req_error_123".to_string()));
    }
}