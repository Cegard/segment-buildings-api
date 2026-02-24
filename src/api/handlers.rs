use crate::{application::segment_image::SegmentImage, constants, domain::entities::ImageInput};
use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use utoipa::ToSchema;
use super::constants as super_constants;

#[derive(Clone)]
pub struct AppState {
    pub use_case: Arc<SegmentImage>,
}

/// Error response structure for API errors
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message describing what went wrong
    pub error: String,
}

/// Health check response
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Status of the server
    pub status: String,
}

/// Multipart form data request for segmentation
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SegmentRequest {
    /// Image file to segment
    #[schema(format = "binary")]
    pub image: String,
}

/// # Segment Buildings in Image
///
/// Uploads an image file and returns a binary segmentation mask identifying building structures.
///
/// ## How It Works
///
/// 1. **Accept**: Receives an image file via multipart form data
/// 2. **Preprocess**: 
///    - Decodes the image (supports PNG, JPG, BMP, TIFF, etc.)
///    - Normalizes RGB values from [0-255] to [0-1]
///    - Converts to tensor format [1, 3, height, width]
/// 3. **Inference**: 
///    - Passes tensor through ONNX model
///    - Returns probability scores for each pixel
/// 4. **Postprocess**:
///    - Applies threshold (0.5) to create binary mask
///    - Converts to [0, 255] grayscale values
/// 5. **Encode**: Outputs as PNG image
///
/// ## Processing Time
/// - Typical: 10-100ms per image (depending on size)
/// - With CUDA: 5-50ms per image
/// - Preprocessing: ~5ms
/// - Inference: ~50-90ms
/// - Postprocessing + encoding: ~5-10ms
///
/// ## Image Size Recommendations
/// - **Small**: 256x256 - fastest, least detailed
/// - **Medium**: 512x512 - balanced
/// - **Large**: 1024x1024+ - most detailed, slower
/// - **Maximum**: Limited by available RAM (typically 4096x4096)
///
/// ## Output Format
/// The result is a grayscale PNG image with the same dimensions as the input:
/// - **White pixels (255)**: Detected building areas
/// - **Black pixels (0)**: Background/non-building areas
/// - **Format**: PNG with lossless compression
///
/// ## cURL Examples
///
/// ### Basic usage:
/// ```bash
/// curl -X POST http://localhost:8080/segment -F "image=@photo.jpg" -o segmentation.png
/// ```
///
/// ### With response headers:
/// ```bash
/// curl -X POST http://localhost:8080/segment -F "image=@photo.jpg" -o segmentation.png -v
/// ```
///
/// ### Using different image format:
/// ```bash
/// curl -X POST http://localhost:8080/segment -F "image=@satellite.png" -o mask.png
/// ```
///
/// ## Python Example
/// ```python
/// import requests
/// 
/// with open('building.jpg', 'rb') as f:
///     files = {'image': f}
///     response = requests.post('http://localhost:8080/segment', files=files)
///     
/// if response.status_code == 200:
///     with open('segmentation.png', 'wb') as out:
///         out.write(response.content)
/// else:
///     print(f"Error: {response.text}")
/// ```
///
/// ## JavaScript/Node.js Example
/// ```javascript
/// const fs = require('fs');
/// const FormData = require('form-data');
/// const fetch = require('node-fetch');
/// 
/// const form = new FormData();
/// form.append('image', fs.createReadStream('building.jpg'));
/// 
/// fetch('http://localhost:8080/segment', {
///     method: 'POST',
///     body: form
/// })
/// .then(res => res.arrayBuffer())
/// .then(buffer => {
///     fs.writeFileSync('segmentation.png', Buffer.from(buffer));
/// });
/// ```
///
/// ## Error Handling
///
/// ### 400 Bad Request Examples
/// - **Missing image field**: Ensure the form field name is 'image'
/// - **Invalid multipart**: Check Content-Type header and boundary formatting
/// - **Unsupported format**: Use PNG, JPG, BMP, TIFF, or WebP
///
/// ### 500 Internal Server Error
/// - Model inference failed: Check MODEL_PATH environment variable
/// - Out of memory: Try with a smaller image
/// - PNG conversion failed: Ensure sufficient disk space
#[utoipa::path(
    post,
    path = "/segment",
    request_body(
        description = "Multipart form data containing the image to segment. Field name: 'image'. Supported formats: PNG, JPG, BMP, TIFF, WebP.",
        content = SegmentRequest,
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "Successful segmentation. Returns a grayscale PNG image where white (255) = building, black (0) = background.", content_type = "image/png", body = Vec<u8>),
        (status = 400, description = "Bad Request. Common causes: missing 'image' field, malformed multipart data, or unsupported image format.", content_type = "text/plain", body = String),
        (status = 500, description = "Internal Server Error. Causes: model inference failed, image preprocessing error, PNG conversion failed, or insufficient memory.", content_type = "text/plain", body = String)
    ),
    tag = "Image Segmentation"
)]
pub async fn segment_handler(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let image_field = match env::var(constants::ENV_IMAGE_FILE_NAME) {
        Ok(name) => name,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "IMAGE_FILE_NAME environment variable not set",
            )
                .into_response();
        }
    };

    let mut image_bytes: Option<Vec<u8>> = None;

    loop {
        let field = match multipart.next_field().await {
            Ok(Some(field)) => field,
            Ok(None) => break,
            Err(_) => {
                return (StatusCode::BAD_REQUEST, "Invalid multipart form data").into_response();
            }
        };

        if field.name() == Some(&image_field) {
            match field.bytes().await {
                Ok(bytes) => {
                    image_bytes = Some(bytes.to_vec());
                }
                Err(_) => {
                    return (StatusCode::BAD_REQUEST, "Failed to read image bytes").into_response();
                }
            }
        }
    }

    let image_bytes = match image_bytes {
        Some(bytes) => bytes,
        None => {
            return (StatusCode::BAD_REQUEST, "Missing image field").into_response();
        }
    };

    let image_input = ImageInput { image_bytes };

    let result = match state.use_case.execute(image_input).await {
        Ok(result) => result,
        Err(e) => {
            let error_msg = format!("{e}");
            let status_code = if error_msg.starts_with(super_constants::DIMENSIONS_ERROR_MSG) {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            return (status_code, format!("Segmentation failed: {e}")).into_response();
        }
    };

    let png_bytes = match super::toolkit::to_png(result) {
        Ok(bytes) => bytes,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to convert segmentation result to PNG: {e}"),
            )
                .into_response();
        }
    };

    (
        StatusCode::OK,
        [
            ("Content-Type", "image/png"),
            (
                "Content-Disposition",
                "attachment; filename=\"segmentation.png\"",
            ),
        ],
        png_bytes,
    )
        .into_response()
}

/// # Health Check
///
/// Returns the health status of the API server. Use this endpoint to:
/// - Verify the server is running and responsive
/// - Monitor service availability
/// - Implement health checks in load balancers
/// - Check server readiness in container orchestration
///
/// ## Usage
///
/// ### Simple health check:
/// ```bash
/// curl http://localhost:8080/health
/// ```
///
/// ### With response time tracking:
/// ```bash
/// curl -w "Response time: %{time_total}s\n" http://localhost:8080/health
/// ```
///
/// ### In a monitoring script:
/// ```bash
/// STATUS=$(curl -s http://localhost:8080/health | jq -r '.status')
/// if [ "$STATUS" = "ok" ]; then
///   echo "API is healthy"
/// else
///   echo "API is unhealthy"
///   exit 1
/// fi
/// ```
///
/// ### Python monitoring:
/// ```python
/// import requests
/// import time
/// 
/// while True:
///     try:
///         response = requests.get('http://localhost:8080/health', timeout=5)
///         if response.status_code == 200:
///             print("✓ API is healthy")
///         else:
///             print("✗ API returned status:", response.status_code)
///     except requests.exceptions.ConnectionError:
///         print("✗ Cannot connect to API")
///     time.sleep(30)
/// ```
///
/// ### Docker health check:
/// ```dockerfile
/// HEALTHCHECK --interval=30s --timeout=10s --start-period=5s \
///   CMD curl -f http://localhost:8080/health || exit 1
/// ```
///
/// ### Kubernetes readiness probe:
/// ```yaml
/// readinessProbe:
///   httpGet:
///     path: /health
///     port: 8080
///   initialDelaySeconds: 5
///   periodSeconds: 10
/// ```
///
/// ### Kubernetes liveness probe:
/// ```yaml
/// livenessProbe:
///   httpGet:
///     path: /health
///     port: 8080
///   initialDelaySeconds: 15
///   periodSeconds: 20
/// ```
///
/// ## Response Codes
/// - **200 OK**: Server is healthy and ready to serve requests
/// - **Connection refused**: Server is not running
/// - **Timeout**: Server is not responding
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Server is healthy and ready. Response contains status field with value 'ok'.", body = HealthResponse)
    ),
    tag = "Health & Status"
)]
pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}
