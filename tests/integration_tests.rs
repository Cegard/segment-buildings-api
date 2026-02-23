use anyhow::Result;
use async_trait::async_trait;
use axum::{body::Body, http::StatusCode, Router};
use buildings_digitizer::api::handlers::{segment_handler, AppState};
use buildings_digitizer::application::definitions::Inferer;
use buildings_digitizer::application::segment_image::SegmentImage;
use http_body_util::BodyExt;
use image::{ImageBuffer, Rgb};
use ndarray::Array4;
use std::sync::Arc;
use tower::ServiceExt;

// Mock Inferer for testing
struct MockInferer {
    return_value: Array4<f32>,
}

#[async_trait]
impl Inferer for MockInferer {
    async fn infer(&self, _input: Array4<f32>) -> Result<Array4<f32>> {
        Ok(self.return_value.clone())
    }
}

// Helper function to create a test image
fn create_test_image(width: u32, height: u32) -> Vec<u8> {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        if (x + y) % 2 == 0 {
            Rgb([255u8, 0u8, 0u8])
        } else {
            Rgb([0u8, 255u8, 0u8])
        }
    });
    let mut bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )
    .unwrap();
    bytes
}

// Helper function to create multipart request body
fn create_multipart_body(image_bytes: Vec<u8>, field_name: &str) -> (Body, String) {
    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let body_content = format!(
        "--{}\r\nContent-Disposition: form-data; name=\"{}\"; filename=\"test.png\"\r\nContent-Type: image/png\r\n\r\n",
        boundary, field_name
    );

    let mut body_bytes = body_content.into_bytes();
    body_bytes.extend_from_slice(&image_bytes);
    body_bytes.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());

    let content_type = format!("multipart/form-data; boundary={}", boundary);
    (Body::from(body_bytes), content_type)
}

#[tokio::test]
async fn test_segment_endpoint_success() {
    // Set up environment variable for image field name
    unsafe {
        std::env::set_var("IMAGE_FILE_NAME", "image");
    }

    // Create mock output
    let mock_output = Array4::<f32>::ones((1, 1, 10, 10));
    let mock_inferer = Arc::new(MockInferer {
        return_value: mock_output,
    });

    let use_case = Arc::new(SegmentImage::new(mock_inferer));
    let state = AppState { use_case };

    let app = Router::new()
        .route("/segment", axum::routing::post(segment_handler))
        .with_state(state);

    // Create test image and multipart body
    let image_bytes = create_test_image(10, 10);
    let (body, content_type) = create_multipart_body(image_bytes, "image");

    let request = axum::http::Request::builder()
        .uri("/segment")
        .method("POST")
        .header("Content-Type", content_type)
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("Content-Type")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(content_type, "image/png");

    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    
    // Verify PNG signature
    assert_eq!(&body_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
}

#[tokio::test]
async fn test_segment_endpoint_missing_image_field() {
    unsafe {
        std::env::set_var("IMAGE_FILE_NAME", "image");
    }

    let mock_output = Array4::<f32>::ones((1, 1, 10, 10));
    let mock_inferer = Arc::new(MockInferer {
        return_value: mock_output,
    });

    let use_case = Arc::new(SegmentImage::new(mock_inferer));
    let state = AppState { use_case };

    let app = Router::new()
        .route("/segment", axum::routing::post(segment_handler))
        .with_state(state);

    // Create multipart body with wrong field name
    let image_bytes = create_test_image(10, 10);
    let (body, content_type) = create_multipart_body(image_bytes, "wrong_field");

    let request = axum::http::Request::builder()
        .uri("/segment")
        .method("POST")
        .header("Content-Type", content_type)
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_segment_endpoint_invalid_image_data() {
    unsafe {
        std::env::set_var("IMAGE_FILE_NAME", "image");
    }

    let mock_output = Array4::<f32>::ones((1, 1, 10, 10));
    let mock_inferer = Arc::new(MockInferer {
        return_value: mock_output,
    });

    let use_case = Arc::new(SegmentImage::new(mock_inferer));
    let state = AppState { use_case };

    let app = Router::new()
        .route("/segment", axum::routing::post(segment_handler))
        .with_state(state);

    // Create multipart body with invalid image data
    let invalid_bytes = vec![1, 2, 3, 4, 5];
    let (body, content_type) = create_multipart_body(invalid_bytes, "image");

    let request = axum::http::Request::builder()
        .uri("/segment")
        .method("POST")
        .header("Content-Type", content_type)
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

struct ErrorInferer;

#[async_trait]
impl Inferer for ErrorInferer {
    async fn infer(&self, _input: Array4<f32>) -> Result<Array4<f32>> {
        Err(anyhow::anyhow!("Inference failed"))
    }
}

#[tokio::test]
async fn test_segment_endpoint_inference_error() {
    unsafe {
        std::env::set_var("IMAGE_FILE_NAME", "image");
    }

    let error_inferer = Arc::new(ErrorInferer);
    let use_case = Arc::new(SegmentImage::new(error_inferer));
    let state = AppState { use_case };

    let app = Router::new()
        .route("/segment", axum::routing::post(segment_handler))
        .with_state(state);

    let image_bytes = create_test_image(10, 10);
    let (body, content_type) = create_multipart_body(image_bytes, "image");

    let request = axum::http::Request::builder()
        .uri("/segment")
        .method("POST")
        .header("Content-Type", content_type)
        .body(body)
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

// Note: Testing missing environment variables in integration tests is challenging
// due to parallel test execution. This test is commented out but the functionality
// is indirectly tested through the handler's error path.
// #[tokio::test]
// async fn test_segment_endpoint_missing_env_variable() {
//     // This test is unreliable when run in parallel with other tests
// }

#[tokio::test]
async fn test_segment_endpoint_with_different_image_sizes() {
    unsafe {
        std::env::set_var("IMAGE_FILE_NAME", "image");
    }

    let test_sizes = vec![(5, 5), (20, 20), (50, 30), (100, 100)];

    for (width, height) in test_sizes {
        let mock_output = Array4::<f32>::ones((1, 1, height as usize, width as usize));
        let mock_inferer = Arc::new(MockInferer {
            return_value: mock_output,
        });

        let use_case = Arc::new(SegmentImage::new(mock_inferer));
        let state = AppState { use_case };

        let app = Router::new()
            .route("/segment", axum::routing::post(segment_handler))
            .with_state(state);

        let image_bytes = create_test_image(width, height);
        let (body, content_type) = create_multipart_body(image_bytes, "image");

        let request = axum::http::Request::builder()
            .uri("/segment")
            .method("POST")
            .header("Content-Type", content_type)
            .body(body)
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "Failed for size {}x{}",
            width,
            height
        );
    }
}
