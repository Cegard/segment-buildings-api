/// Simple test to verify OpenAPI spec documents all system endpoints
#[test]
fn test_openapi_spec_has_all_endpoints() {
    use std::fs;

    let spec_content = fs::read_to_string("openapi.yaml")
        .expect("openapi.yaml should exist");

    // Verify OpenAPI spec structure
    assert!(
        spec_content.contains("openapi: 3.0.0"),
        "OpenAPI spec should specify version 3.0.0"
    );

    assert!(
        spec_content.contains("info:"),
        "OpenAPI spec should have info section"
    );

    assert!(
        spec_content.contains("paths:"),
        "OpenAPI spec should have paths section"
    );

    // Verify all 3 main endpoints are documented
    assert!(
        spec_content.contains("/segment:"),
        "OpenAPI spec should document /segment endpoint"
    );

    assert!(
        spec_content.contains("/health:"),
        "OpenAPI spec should document /health endpoint"
    );

    assert!(
        spec_content.contains("/api/openapi.json:"),
        "OpenAPI spec should document /api/openapi.json endpoint"
    );
}

/// Test that OpenAPI spec documents the HTTP methods for each endpoint
#[test]
fn test_openapi_spec_documents_methods() {
    use std::fs;

    let spec_content = fs::read_to_string("openapi.yaml")
        .expect("openapi.yaml should exist");

    // Check that HTTP methods are documented
    assert!(
        spec_content.contains("post:"),
        "OpenAPI spec should document POST method"
    );

    assert!(
        spec_content.contains("get:"),
        "OpenAPI spec should document GET method"
    );
}
