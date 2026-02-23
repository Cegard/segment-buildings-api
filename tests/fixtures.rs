// Test fixtures and helpers for testing
use image::{ImageBuffer, Rgb};

/// Creates a simple test image with a checkerboard pattern
pub fn create_checkerboard_image(width: u32, height: u32) -> Vec<u8> {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        if (x + y) % 2 == 0 {
            Rgb([255u8, 255u8, 255u8])
        } else {
            Rgb([0u8, 0u8, 0u8])
        }
    });
    encode_image_to_png(img)
}

/// Creates a solid color test image
pub fn create_solid_color_image(width: u32, height: u32, color: [u8; 3]) -> Vec<u8> {
    let img = ImageBuffer::from_pixel(width, height, Rgb(color));
    encode_image_to_png(img)
}

/// Creates a gradient test image
pub fn create_gradient_image(width: u32, height: u32) -> Vec<u8> {
    let img = ImageBuffer::from_fn(width, height, |x, _y| {
        let intensity = ((x as f32 / width as f32) * 255.0) as u8;
        Rgb([intensity, intensity, intensity])
    });
    encode_image_to_png(img)
}

/// Creates a test image with a white square in the center
pub fn create_centered_square_image(width: u32, height: u32, square_size: u32) -> Vec<u8> {
    let img = ImageBuffer::from_fn(width, height, |x, y| {
        let x_start = (width - square_size) / 2;
        let y_start = (height - square_size) / 2;
        let x_end = x_start + square_size;
        let y_end = y_start + square_size;

        if x >= x_start && x < x_end && y >= y_start && y < y_end {
            Rgb([255u8, 255u8, 255u8])
        } else {
            Rgb([0u8, 0u8, 0u8])
        }
    });
    encode_image_to_png(img)
}

/// Helper to encode an image buffer to PNG bytes
fn encode_image_to_png(img: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bytes = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )
    .unwrap();
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_checkerboard_image() {
        let bytes = create_checkerboard_image(10, 10);
        assert!(!bytes.is_empty());
        // Verify PNG signature
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_create_solid_color_image() {
        let bytes = create_solid_color_image(5, 5, [255, 0, 0]);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_create_gradient_image() {
        let bytes = create_gradient_image(20, 20);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_create_centered_square_image() {
        let bytes = create_centered_square_image(50, 50, 20);
        assert!(!bytes.is_empty());
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_all_fixtures_are_valid_pngs() {
        let fixtures = vec![
            create_checkerboard_image(10, 10),
            create_solid_color_image(10, 10, [128, 128, 128]),
            create_gradient_image(15, 15),
            create_centered_square_image(30, 30, 10),
        ];

        for fixture in fixtures {
            // Verify they can be decoded
            let img = image::load_from_memory(&fixture);
            assert!(img.is_ok(), "Failed to decode fixture image");
        }
    }
}
