# API Quick Reference

## Endpoints Summary

| Method | Path | Description | Auth |
|--------|------|-------------|------|
| POST | `/segment` | Segment buildings in image | No |
| GET | `/health` | Health check | No |
| GET | `/api/openapi.json` | OpenAPI specification | No |

---

## Quick Start

### 1. Segment an Image (cURL)

```bash
curl -X POST http://localhost:8080/segment \
  -F "image=@building.jpg" \
  -o result.png
```

### 2. Check Server Status

```bash
curl http://localhost:8080/health
# Response: {"status":"ok"}
```

### 3. Get OpenAPI Spec

```bash
curl http://localhost:8080/api/openapi.json
```

---

## Request/Response Examples

### POST /segment

**Request:**
```
POST /segment HTTP/1.1
Host: localhost:8080
Content-Type: multipart/form-data; boundary=----FormBoundary

------FormBoundary
Content-Disposition: form-data; name="image"; filename="photo.jpg"
Content-Type: image/jpeg

[binary image data]
------FormBoundary--
```

**Response (200 OK):**
```
HTTP/1.1 200 OK
Content-Type: image/png
Content-Disposition: attachment; filename="segmentation.png"

[binary PNG data]
```

**Response (400 Bad Request):**
```
HTTP/1.1 400 Bad Request
Content-Type: text/plain

Missing image field
```

**Response (500 Internal Server Error):**
```
HTTP/1.1 500 Internal Server Error
Content-Type: text/plain

Segmentation failed: model inference error
```

### GET /health

**Response (200 OK):**
```json
{
  "status": "ok"
}
```

---

## Python Client Example

```python
import requests
from PIL import Image
import io

def segment_image(image_path):
    """Segment a building image."""
    with open(image_path, 'rb') as f:
        files = {'image': f}
        response = requests.post(
            'http://localhost:8080/segment',
            files=files
        )
    
    if response.status_code == 200:
        mask = Image.open(io.BytesIO(response.content))
        return mask
    else:
        raise Exception(f"API Error: {response.text}")

# Usage
result = segment_image('building.jpg')
result.save('segmentation.png')
```

---

## JavaScript Fetch Example

```javascript
async function segmentImage(file) {
    const formData = new FormData();
    formData.append('image', file);
    
    const response = await fetch('http://localhost:8080/segment', {
        method: 'POST',
        body: formData
    });
    
    if (response.ok) {
        return await response.blob();
    } else {
        throw new Error(`API error: ${response.statusText}`);
    }
}

// Usage
const input = document.getElementById('imageInput');
const blob = await segmentImage(input.files[0]);
const url = URL.createObjectURL(blob);
document.getElementById('result').src = url;
```

---

## Environment Variables

```bash
# Server configuration
ON_ADDRESS=0.0.0.0              # Bind address
SERVING_PORT=8080               # Server port
MODEL_PATH=/path/to/model.onnx  # ONNX model
ON_CUDA=false                   # CUDA acceleration
MAX_CON_CALLS=1                 # Max concurrent requests
IMAGE_FILE_NAME=image           # Form field name
```

---

## HTTP Status Codes

| Code | Meaning | Common Causes |
|------|---------|---------------|
| 200 | Success | Request processed successfully |
| 400 | Bad Request | Missing/invalid image, malformed data |
| 500 | Server Error | Model error, inference failed |

---

## Testing

### Test Connectivity
```bash
curl http://localhost:8080/health
```

### Test Segmentation
```bash
# With actual image
curl -X POST http://localhost:8080/segment \
  -F "image=@test.jpg" \
  -o output.png

# Verify output is valid PNG
file output.png
```

### Test OpenAPI Endpoint
```bash
curl http://localhost:8080/api/openapi.json | jq '.'
```

---

## Common Issues

| Issue | Solution |
|-------|----------|
| Connection refused | Server not running, check port |
| Missing image field | Ensure form field name is "image" |
| Invalid image format | Use supported format: PNG, JPG, BMP, etc |
| Memory error | Reduce image size or increase system RAM |
| Poor response | Enable CUDA, increase MAX_CON_CALLS |

---

## Integration Examples

### Docker
```bash
docker run -p 8080:8080 \
  -e MODEL_PATH=/app/model.onnx \
  buildings-digitalizer
```

### Kubernetes Service
```yaml
apiVersion: v1
kind: Service
metadata:
  name: buildings-api
spec:
  ports:
  - port: 8080
    targetPort: 8080
  selector:
    app: buildings-digitalizer
```

### Using with Insomnia/Postman
1. Import: `http://localhost:8080/api/openapi.json`
2. Create request to POST /segment
3. Select image file
4. Send request

---

## Performance Tips

- Enable CUDA: `ON_CUDA=true`
- Increase concurrency: `MAX_CON_CALLS=4`
- Optimize image size: 256-1024px
- Use compression for network transfer

---

## For More Information

- Full API Documentation: [API_DOCUMENTATION.md](API_DOCUMENTATION.md)
- OpenAPI Specification: [openapi.yaml](../openapi.yaml)
- Testing Guide: [TESTING.md](TESTING.md)
- Test Summary: [TEST_SUMMARY.md](TEST_SUMMARY.md)
