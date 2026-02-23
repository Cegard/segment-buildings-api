use crate::domain::entities::{Dimensions, SegmentationResult};
use anyhow::Result;
use image::ImageReader;
use ndarray::{Array4, Axis};

pub(super) fn preprocess(image_bytes: Vec<u8>) -> Result<Array4<f32>> {
    let img = ImageReader::new(std::io::Cursor::new(image_bytes))
        .with_guessed_format()?
        .decode()?
        .to_rgb8();

    let (width, height) = img.dimensions();

    let mut array = Array4::<f32>::zeros((1, 3, height as usize, width as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;

        array[[0, 0, y as usize, x as usize]] = r as f32 / 255.0;
        array[[0, 1, y as usize, x as usize]] = g as f32 / 255.0;
        array[[0, 2, y as usize, x as usize]] = b as f32 / 255.0;
    }

    Ok(array)
}

pub(super) fn postprocess(output: Array4<f32>) -> Result<SegmentationResult> {
    let output = output.index_axis(Axis(0), 0);
    let output = output.index_axis(Axis(0), 0);

    let height = output.shape()[0];
    let width = output.shape()[1];

    let mask: Vec<u8> = output
        .iter()
        .map(|&x| if x > 0.5 { 255 } else { 0 })
        .collect();

    Ok(SegmentationResult {
        mask,
        dimensions: Dimensions { width, height },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{ImageBuffer, Rgb};

    #[test]
    fn test_preprocess_creates_correct_dimensions() {
        // Create a simple 2x2 test image
        let img = ImageBuffer::from_fn(2, 2, |x, y| {
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

        let result = preprocess(bytes).unwrap();
        let shape = result.shape();

        assert_eq!(shape, &[1, 3, 2, 2], "Shape should be [1, 3, 2, 2]");
    }

    #[test]
    fn test_preprocess_normalizes_values() {
        // Create a simple 1x1 white pixel image
        let img = ImageBuffer::from_pixel(1, 1, Rgb([255u8, 255u8, 255u8]));

        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

        let result = preprocess(bytes).unwrap();

        // All RGB channels should be normalized to 1.0
        assert_eq!(result[[0, 0, 0, 0]], 1.0);
        assert_eq!(result[[0, 1, 0, 0]], 1.0);
        assert_eq!(result[[0, 2, 0, 0]], 1.0);
    }

    #[test]
    fn test_preprocess_handles_black_pixel() {
        // Create a simple 1x1 black pixel image
        let img = ImageBuffer::from_pixel(1, 1, Rgb([0u8, 0u8, 0u8]));

        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

        let result = preprocess(bytes).unwrap();

        // All RGB channels should be normalized to 0.0
        assert_eq!(result[[0, 0, 0, 0]], 0.0);
        assert_eq!(result[[0, 1, 0, 0]], 0.0);
        assert_eq!(result[[0, 2, 0, 0]], 0.0);
    }

    #[test]
    fn test_preprocess_fails_with_invalid_image() {
        let invalid_bytes = vec![0, 1, 2, 3, 4];
        let result = preprocess(invalid_bytes);
        assert!(result.is_err(), "Should fail with invalid image data");
    }

    #[test]
    fn test_postprocess_thresholds_correctly() {
        // Create a 2x2 output array with values above and below threshold
        let mut output = Array4::<f32>::zeros((1, 1, 2, 2));
        output[[0, 0, 0, 0]] = 0.8; // Above threshold -> 255
        output[[0, 0, 0, 1]] = 0.3; // Below threshold -> 0
        output[[0, 0, 1, 0]] = 0.6; // Above threshold -> 255
        output[[0, 0, 1, 1]] = 0.1; // Below threshold -> 0

        let result = postprocess(output).unwrap();

        assert_eq!(result.dimensions.width, 2);
        assert_eq!(result.dimensions.height, 2);
        assert_eq!(result.mask, vec![255, 0, 255, 0]);
    }

    #[test]
    fn test_postprocess_edge_case_exactly_threshold() {
        // Test with value exactly at threshold
        let mut output = Array4::<f32>::zeros((1, 1, 1, 1));
        output[[0, 0, 0, 0]] = 0.5;

        let result = postprocess(output).unwrap();

        // At 0.5, the condition x > 0.5 is false, so it should be 0
        assert_eq!(result.mask, vec![0]);
    }

    #[test]
    fn test_postprocess_handles_large_array() {
        // Test with a larger array
        let output = Array4::<f32>::ones((1, 1, 10, 10));
        
        let result = postprocess(output).unwrap();

        assert_eq!(result.dimensions.width, 10);
        assert_eq!(result.dimensions.height, 10);
        assert_eq!(result.mask.len(), 100);
        assert!(result.mask.iter().all(|&v| v == 255));
    }
}
