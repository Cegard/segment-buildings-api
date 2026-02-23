use crate::domain::entities::SegmentationResult;
use image::{ImageBuffer, Luma};

pub(super) fn to_png(result: SegmentationResult) -> anyhow::Result<Vec<u8>> {
    let img: ImageBuffer<Luma<u8>, _> = ImageBuffer::from_raw(
        result.dimensions.width as u32,
        result.dimensions.height as u32,
        result.mask.clone(),
    )
    .ok_or_else(|| anyhow::anyhow!("Failed to create image buffer from segmentation result"))?;
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    )?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::entities::Dimensions;

    #[test]
    fn test_to_png_creates_valid_png() {
        let result = SegmentationResult {
            mask: vec![0, 128, 255, 64],
            dimensions: Dimensions {
                width: 2,
                height: 2,
            },
        };

        let png_bytes = to_png(result).unwrap();

        // Verify PNG signature (first 8 bytes)
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_to_png_with_single_pixel() {
        let result = SegmentationResult {
            mask: vec![255],
            dimensions: Dimensions {
                width: 1,
                height: 1,
            },
        };

        let png_bytes = to_png(result).unwrap();

        assert!(!png_bytes.is_empty());
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_to_png_with_all_black() {
        let result = SegmentationResult {
            mask: vec![0; 100],
            dimensions: Dimensions {
                width: 10,
                height: 10,
            },
        };

        let png_bytes = to_png(result).unwrap();

        assert!(!png_bytes.is_empty());
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_to_png_with_all_white() {
        let result = SegmentationResult {
            mask: vec![255; 100],
            dimensions: Dimensions {
                width: 10,
                height: 10,
            },
        };

        let png_bytes = to_png(result).unwrap();

        assert!(!png_bytes.is_empty());
        assert_eq!(&png_bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }

    #[test]
    fn test_to_png_fails_with_mismatched_dimensions() {
        let result = SegmentationResult {
            mask: vec![255, 0, 128],
            dimensions: Dimensions {
                width: 2,
                height: 2,
            },
        };

        let png_result = to_png(result);
        assert!(png_result.is_err());
    }

    #[test]
    fn test_to_png_roundtrip() {
        let original_mask = vec![0, 50, 100, 150, 200, 255];
        let result = SegmentationResult {
            mask: original_mask.clone(),
            dimensions: Dimensions {
                width: 3,
                height: 2,
            },
        };

        let png_bytes = to_png(result).unwrap();

        // Decode the PNG and verify the data
        let img = image::load_from_memory(&png_bytes).unwrap();
        let gray_img = img.to_luma8();

        assert_eq!(gray_img.dimensions(), (3, 2));
        
        let decoded_mask: Vec<u8> = gray_img.into_raw();
        assert_eq!(decoded_mask, original_mask);
    }
}
