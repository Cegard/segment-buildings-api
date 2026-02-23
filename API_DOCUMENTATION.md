# Buildings Digitalizer API Documentation

## Overview

The Buildings Digitalizer API is a RESTful service that provides image segmentation capabilities for identifying building structures in photographs. It leverages deep learning models via ONNX Runtime to process images and return binary segmentation masks.

## Base URL

```
Development:  http://localhost:8080
Production:   https://api.buildingsdigitalizer.com
```

## API Endpoints

### 1. Segment Image

Segments buildings in the provided image and returns a binary mask.

#### Endpoint
```
POST /segment
```

#### Content-Type
```
multipart/form-data
```

#### Request Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| image | file | Yes | Image file to segment. Supported formats: PNG, JPG, BMP, TIFF, WebP, etc. |

#### Request Example

**Using cURL:**
```bash
curl -X POST http://localhost:8080/segment \
  -F "image=@/path/to/image.jpg"
```

**Using Python:**
```python
import requests

with open('image.jpg', 'rb') as f:
    files = {'image': f}
    response = requests.post('http://localhost:8080/segment', files=files)
    
# Save segmentation mask
with open('segmentation.png', 'wb') as f:
    f.write(response.content)
```

**Using JavaScript/Node.js:**
```javascript
const FormData = require('form-data');
const fs = require('fs');
const axios = require('axios');

const formData = new FormData();
formData.append('image', fs.createReadStream('image.jpg'));

axios.post('http://localhost:8080/segment', formData, {
    headers: formData.getHeaders()
}).then(response => {
    fs.writeFileSync('segmentation.png', response.data);
}).catch(error => {
    console.error('Error:', error.response.data);
});
```

#### Response

**Success Response (HTTP 200):**
- **Content-Type:** `image/png`
- **Body:** Binary PNG image data

The response is a grayscale PNG image where:
- **White (255):** Building/segmented area
- **Black (0):** Background/non-segmented area

**Example dimensions:** Same as input image

#### Error Responses

**Bad Request (HTTP 400):**
```
Missing image field
Invalid multipart form data
Failed to read image bytes
```

**Internal Server Error (HTTP 500):**
```
Segmentation failed: [error details]
Failed to convert segmentation result to PNG: [error details]
```

---

### 2. Health Check

Verify that the API server is running and responsive.

#### Endpoint
```
GET /health
```

#### Response

**Success Response (HTTP 200):**
```json
{
  "status": "ok"
}
```

#### Example

**Using cURL:**
```bash
curl http://localhost:8080/health
```

**Using Python:**
```python
import requests

response = requests.get('http://localhost:8080/health')
print(response.json())  # {'status': 'ok'}
```

---

### 3. OpenAPI Specification

Get the OpenAPI/Swagger specification of the API.

#### Endpoint
```
GET /api/openapi.json
```

#### Response

**Success Response (HTTP 200):**
```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "Buildings Digitalizer API",
    "version": "0.1.0",
    ...
  },
  "paths": {...},
  "components": {...}
}
```

#### Example

**Using cURL:**
```bash
curl http://localhost:8080/api/openapi.json | jq
```

---

## Data Structures

### SegmentationResult

Response structure returned during successful segmentation.

```rust
{
  "mask": [byte_array],        // Binary mask as Vec<u8>
  "dimensions": {
    "width": number,            // Image width in pixels
    "height": number            // Image height in pixels
  }
}
```

### Dimensions

```rust
{
  "width": number,    // Width in pixels
  "height": number    // Height in pixels
}
```

### ErrorResponse

```rust
{
  "error": string    // Error message
}
```

---

## Status Codes

| Code | Status | Description |
|------|--------|-------------|
| 200 | OK | Segmentation successful |
| 400 | Bad Request | Invalid request (missing field, malformed data) |
| 500 | Internal Server Error | Server error during processing |

---

## Authentication

Currently, the API does **not require authentication**. Future versions may implement API keys or OAuth2.

---

## Rate Limiting

Currently, **no rate limiting is enforced**. The server uses a semaphore to limit concurrent inference operations based on the `MAX_CON_CALLS` environment variable (default: 1).

---

## Error Handling

### Common Errors

#### 1. Missing Image Field
```bash
$ curl -X POST http://localhost:8080/segment

Response: 400 Bad Request
Body: Missing image field
```

**Solution:** Ensure the form data includes the `image` field.

#### 2. Invalid Image Format
```bash
$ curl -X POST http://localhost:8080/segment -F "image=@corrupt.dat"

Response: 400 Bad Request
Body: Segmentation failed: [image parsing error]
```

**Solution:** Ensure the file is a valid image format (PNG, JPG, BMP, etc.).

#### 3. Model Inference Error
```bash
Response: 500 Internal Server Error
Body: Segmentation failed: model inference error
```

**Solution:** 
- Check model file exists at `MODEL_PATH`
- Ensure ONNX Runtime is properly configured
- Check server logs for details

#### 4. PNG Conversion Error
```bash
Response: 500 Internal Server Error
Body: Failed to convert segmentation result to PNG: [error details]
```

**Solution:** 
- Ensure sufficient disk space
- Check memory availability

---

## Configuration

The API is configured via environment variables:

| Variable | Type | Default | Description |
|----------|------|---------|-------------|
| `ON_ADDRESS` | string | 0.0.0.0 | Server bind address |
| `PORT` | number | 8080 | Server port |
| `MODEL_PATH` | string | - | Path to ONNX model file |
| `ON_CUDA` | boolean | false | Enable CUDA acceleration |
| `MAX_CON_CALLS` | number | 1 | Max concurrent inferences |
| `IMAGE_FILE_NAME` | string | image | Form field name for image |

---

## Usage Examples

### Complete End-to-End Example

**Python:**
```python
import requests
from PIL import Image
import numpy as np

# 1. Load input image
input_image = Image.open('building.jpg')

# 2. Send to API
with open('building.jpg', 'rb') as f:
    files = {'image': f}
    response = requests.post('http://localhost:8080/segment', files=files)

# 3. Check response
if response.status_code == 200:
    # 4. Save segmentation mask
    mask = Image.open(io.BytesIO(response.content))
    mask.save('segmentation_mask.png')
    
    # 5. Visualize results
    mask.show()
else:
    print(f"Error: {response.text}")
```

**cURL with Image Processing:**
```bash
#!/bin/bash

# Segment image
curl -X POST http://localhost:8080/segment \
  -F "image=@input.jpg" \
  -o segmentation.png

# Verify output
file segmentation.png

# Display info
identify segmentation.png
```

---

## Performance Considerations

### Image Size Limits
- **Recommended:** 256x256 to 2048x2048 pixels
- **Minimum:** 1x1 pixels
- **Maximum:** Limited by available memory

### Processing Time
- Depends on image size and model complexity
- CUDA acceleration recommended for large batches
- Single inference typically completes in milliseconds

### Concurrent Requests
- Limited by `MAX_CON_CALLS` environment variable
- Queued requests wait for available inference slot
- Each inference is thread-safe

---

## Integration Guides

### React/TypeScript
```typescript
const segmentImage = async (file: File): Promise<Blob> => {
    const formData = new FormData();
    formData.append('image', file);
    
    const response = await fetch('http://localhost:8080/segment', {
        method: 'POST',
        body: formData,
    });
    
    if (!response.ok) {
        throw new Error(`API error: ${response.statusText}`);
    }
    
    return response.blob();
};
```

### Go
```go
package main

import (
    "bytes"
    "fmt"
    "io/ioutil"
    "mime/multipart"
    "net/http"
    "os"
)

func segmentImage(imagePath string) ([]byte, error) {
    file, err := os.Open(imagePath)
    if err != nil {
        return nil, err
    }
    defer file.Close()
    
    body := &bytes.Buffer{}
    writer := multipart.NewWriter(body)
    
    part, err := writer.CreateFormFile("image", imagePath)
    if err != nil {
        return nil, err
    }
    
    io.Copy(part, file)
    writer.Close()
    
    resp, err := http.Post(
        "http://localhost:8080/segment",
        writer.FormDataContentType(),
        body,
    )
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    return ioutil.ReadAll(resp.Body)
}
```

---

## OpenAPI/Swagger Support

The API provides OpenAPI 3.0 specification at:

```
GET /api/openapi.json
```

### Using with Swagger UI

1. Download Swagger UI
2. Point it to: `http://localhost:8080/api/openapi.json`
3. Interact with the API through the web interface

### Using with Postman

1. Open Postman
2. File → Import
3. Paste URL: `http://localhost:8080/api/openapi.json`
4. Import collection
5. Start testing endpoints

---

## Troubleshooting

### API Not Responding
```bash
# Test connectivity
curl http://localhost:8080/health

# Check server is running
ps aux | grep buildings_digitizer

# Check logs
docker logs container_name  # if using Docker
```

### Segmentation Failing
```bash
# Verify model file exists
ls -la $MODEL_PATH

# Test with simple image
curl -X POST http://localhost:8080/segment \
  -F "image=@simple_test.jpg" \
  -v
```

### Performance Issues
- Increase `MAX_CON_CALLS` for concurrent processing
- Enable CUDA: `ON_CUDA=true`
- Reduce image size
- Add server resources

---

## FAQ

**Q: What image formats are supported?**
A: PNG, JPG, BMP, TIFF, WebP, and any format supported by the Rust `image` crate.

**Q: What's the maximum image size?**
A: Limited by available memory. Typically handles images up to 4096x4096 pixels on standard hardware.

**Q: Can I use the API without authentication?**
A: Yes, the current version requires no authentication. Future versions may add API key support.

**Q: How do I deploy this in production?**
A: Use Docker, Kubernetes, or run as a systemd service. See deployment guide for details.

**Q: Can I customize the model used?**
A: Yes, by setting the `MODEL_PATH` environment variable to your ONNX model file.

**Q: Does it support batch processing?**
A: Currently, single images per request. Batch support may be added in future versions.

---

## API Versioning

Current API Version: **0.1.0**

The API follows semantic versioning:
- **MAJOR:** Breaking changes
- **MINOR:** New features (backward compatible)
- **PATCH:** Bug fixes

Version is included in OpenAPI spec and response headers.

---

## Support

For issues, questions, or feature requests:
- Email: support@buildingsdigitalizer.com
- Documentation: See TESTING.md and TESTING_QUICK_REF.md
- GitHub Issues: [Repository URL]

---

## License

This API is provided under the MIT License. See LICENSE file for details.
