# Test Suite for Buildings Digitalizer

This document describes the test suite for the Buildings Digitalizer REST API server.

## Test Structure

The test suite is organized into three main categories:

### 1. Unit Tests

Unit tests are located inline within their respective modules:

- **`src/application/toolkit.rs`**: Tests for image preprocessing and postprocessing
  - `test_preprocess_creates_correct_dimensions`: Verifies tensor shape after preprocessing
  - `test_preprocess_normalizes_values`: Ensures RGB values are normalized to [0,1]
  - `test_preprocess_handles_black_pixel`: Tests normalization of black pixels
  - `test_preprocess_fails_with_invalid_image`: Validates error handling for invalid images
  - `test_postprocess_thresholds_correctly`: Tests binary thresholding logic
  - `test_postprocess_edge_case_exactly_threshold`: Tests threshold boundary condition
  - `test_postprocess_handles_large_array`: Validates handling of large output arrays

- **`src/api/toolkit.rs`**: Tests for PNG conversion
  - `test_to_png_creates_valid_png`: Verifies PNG generation with valid signature
  - `test_to_png_with_single_pixel`: Tests edge case with 1x1 image
  - `test_to_png_with_all_black`: Tests all-zero mask conversion
  - `test_to_png_with_all_white`: Tests all-255 mask conversion
  - `test_to_png_fails_with_mismatched_dimensions`: Validates dimension mismatch handling
  - `test_to_png_roundtrip`: Tests encode-decode cycle preserves data

- **`src/application/segment_image.rs`**: Tests for the SegmentImage use case
  - `test_segment_image_executes_successfully`: Tests successful segmentation flow
  - `test_segment_image_with_mixed_output`: Tests mixed threshold outputs
  - `test_segment_image_with_invalid_image`: Validates error handling for invalid images
  - `test_segment_image_handles_inference_error`: Tests inference error propagation

### 2. Integration Tests

Integration tests are located in `tests/integration_tests.rs`:

- **`test_segment_endpoint_success`**: End-to-end test of successful segmentation request
- **`test_segment_endpoint_missing_image_field`**: Tests handling of missing image field
- **`test_segment_endpoint_invalid_image_data`**: Tests handling of invalid image data
- **`test_segment_endpoint_inference_error`**: Tests handling of inference errors
- **`test_segment_endpoint_missing_env_variable`**: Tests handling of missing environment variables
- **`test_segment_endpoint_with_different_image_sizes`**: Tests various image dimensions

### 3. Test Fixtures

Test fixtures and helper functions are located in `tests/fixtures.rs`:

- **`create_checkerboard_image`**: Creates a checkerboard pattern test image
- **`create_solid_color_image`**: Creates a solid color test image
- **`create_gradient_image`**: Creates a horizontal gradient test image
- **`create_centered_square_image`**: Creates an image with a centered white square

## Running Tests

### Run All Tests

```bash
cargo test
```

### Run Unit Tests Only

```bash
cargo test --lib
```

### Run Integration Tests Only

```bash
cargo test --test integration_tests
```

### Run Tests for a Specific Module

```bash
cargo test --lib toolkit
```

### Run a Specific Test

```bash
cargo test test_segment_endpoint_success
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

### Run Tests in Parallel (default)

```bash
cargo test -- --test-threads=4
```

### Run Tests Sequentially

```bash
cargo test -- --test-threads=1
```

## Test Coverage

To generate test coverage reports, you can use `tarpaulin`:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html --output-dir coverage
```

## Mock Objects

The test suite uses custom mock implementations:

- **MockInferer**: A mock implementation of the `Inferer` trait that returns predefined tensor outputs
- **ErrorInferer**: A mock that always returns errors for testing error handling

## Environment Variables for Testing

Integration tests set the following environment variables:

- `IMAGE_FILE_NAME`: The name of the multipart form field containing the image (default: "image")

## Best Practices

1. **Isolation**: Each test is independent and doesn't rely on shared state
2. **Fast Execution**: Unit tests use small test images for quick execution
3. **Comprehensive Coverage**: Tests cover success paths, error paths, and edge cases
4. **Clear Naming**: Test names clearly describe what is being tested
5. **Fixtures**: Reusable test fixtures are provided for common test scenarios

## Continuous Integration

The test suite is designed to run in CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Run tests
  run: cargo test --verbose
```

## Adding New Tests

When adding new tests:

1. **Unit Tests**: Add them in the same file as the code being tested, in a `#[cfg(test)]` module
2. **Integration Tests**: Add them to `tests/integration_tests.rs` or create a new file in the `tests/` directory
3. **Test Fixtures**: Add reusable test data to `tests/fixtures.rs`

## Known Limitations

- Integration tests do not test the actual ONNX model inference (uses mocks instead)
- Performance tests are not included in the current suite
- Load testing requires separate tools (e.g., Apache Bench, wrk)

## Troubleshooting

### Tests Failing Due to Missing Dependencies

```bash
cargo clean
cargo build --tests
cargo test
```

### Tests Timing Out

Increase the timeout for async tests:

```rust
#[tokio::test(flavor = "multi_thread")]
async fn my_test() {
    // test code
}
```
