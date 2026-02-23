# Quick Test Reference

## Running Tests

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test --verbose

# Run a specific test
cargo test test_segment_image_executes_successfully

# Run tests matching a pattern
cargo test preprocess

# Run tests in a specific file
cargo test --test integration_tests

# Run unit tests only
cargo test --lib

# Show output from tests
cargo test -- --nocapture

# Run tests sequentially (not in parallel)
cargo test -- --test-threads=1
```

## Test Organization

```
rest_api_server/
├── src/
│   ├── lib.rs                          # Exposes modules for testing
│   ├── api/
│   │   └── toolkit.rs                  # 6 unit tests for PNG conversion
│   ├── application/
│   │   ├── toolkit.rs                  # 7 unit tests for pre/post-processing
│   │   └── segment_image.rs            # 4 unit tests for use case
│   └── ...
├── tests/
│   ├── integration_tests.rs            # 5 integration tests for API
│   └── fixtures.rs                     # 5 fixture tests + helpers
├── TESTING.md                          # Detailed testing guide
└── TEST_SUMMARY.md                     # Complete test summary
```

## Test Count by Category

| Category | Count | Location |
|----------|-------|----------|
| Unit Tests (Application) | 11 | `src/application/*.rs` |
| Unit Tests (API) | 6 | `src/api/toolkit.rs` |
| Integration Tests | 5 | `tests/integration_tests.rs` |
| Fixture Tests | 5 | `tests/fixtures.rs` |
| **Total** | **44** | **Multiple files** |

## Common Test Commands

```bash
# Quick test run
cargo test

# Detailed test run with timing
cargo test --verbose

# Test a specific module
cargo test --lib toolkit

# Test with coverage (requires tarpaulin)
cargo tarpaulin --out Html

# Watch mode (requires cargo-watch)
cargo watch -x test
```

## Test Fixtures Available

```rust
use fixtures::*;

// Create test images
let checkerboard = create_checkerboard_image(10, 10);
let solid = create_solid_color_image(10, 10, [255, 0, 0]);
let gradient = create_gradient_image(20, 20);
let square = create_centered_square_image(50, 50, 20);
```

## Mock Objects

```rust
// MockInferer - Returns predefined output
struct MockInferer {
    return_value: Array4<f32>,
}

// ErrorInferer - Always returns errors
struct ErrorInferer;
```

## CI/CD Integration

### GitHub Actions
```yaml
- name: Run tests
  run: cargo test --verbose
```

### GitLab CI
```yaml
test:
  script:
    - cargo test --verbose
```

## Test Results Summary

✅ **44/44 tests passing**
⏱️ **Execution time: ~0.05s**
📊 **Coverage: Core functionality covered**

---

For more details, see [TESTING.md](TESTING.md)
