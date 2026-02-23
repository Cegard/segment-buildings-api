use super::definitions::Inferer;
use crate::domain::entities::{ImageInput, SegmentationResult};
use anyhow::Result;
use std::sync::Arc;

pub struct SegmentImage {
    inferer: Arc<dyn Inferer>,
}

impl SegmentImage {
    pub fn new(inferer: Arc<dyn Inferer>) -> Self {
        Self { inferer }
    }

    pub async fn execute(&self, image_input: ImageInput) -> Result<SegmentationResult> {
        let input_tensor = super::toolkit::preprocess(image_input.image_bytes)?;
        let raw_output = self.inferer.infer(input_tensor).await?;
        let segmentation_result = super::toolkit::postprocess(raw_output)?;
        Ok(segmentation_result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use image::{ImageBuffer, Rgb};
    use ndarray::Array4;

    struct MockInferer {
        return_value: Array4<f32>,
    }

    #[async_trait]
    impl Inferer for MockInferer {
        async fn infer(&self, _input: Array4<f32>) -> Result<Array4<f32>> {
            Ok(self.return_value.clone())
        }
    }

    #[tokio::test]
    async fn test_segment_image_executes_successfully() {
        // Create a simple test image
        let img = ImageBuffer::from_fn(10, 10, |_x, _y| Rgb([255u8, 0u8, 0u8]));
        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

        // Create mock output (all values above threshold)
        let mock_output = Array4::<f32>::ones((1, 1, 10, 10));
        let mock_inferer = Arc::new(MockInferer {
            return_value: mock_output,
        });

        let segment_image = SegmentImage::new(mock_inferer);
        let image_input = ImageInput { image_bytes: bytes };

        let result = segment_image.execute(image_input).await.unwrap();

        assert_eq!(result.dimensions.width, 10);
        assert_eq!(result.dimensions.height, 10);
        assert_eq!(result.mask.len(), 100);
        assert!(result.mask.iter().all(|&v| v == 255));
    }

    #[tokio::test]
    async fn test_segment_image_with_mixed_output() {
        // Create a simple test image
        let img = ImageBuffer::from_fn(2, 2, |_x, _y| Rgb([0u8, 255u8, 0u8]));
        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

        // Create mock output with mixed values
        let mut mock_output = Array4::<f32>::zeros((1, 1, 2, 2));
        mock_output[[0, 0, 0, 0]] = 0.8; // Above threshold
        mock_output[[0, 0, 0, 1]] = 0.2; // Below threshold
        mock_output[[0, 0, 1, 0]] = 0.9; // Above threshold
        mock_output[[0, 0, 1, 1]] = 0.1; // Below threshold

        let mock_inferer = Arc::new(MockInferer {
            return_value: mock_output,
        });

        let segment_image = SegmentImage::new(mock_inferer);
        let image_input = ImageInput { image_bytes: bytes };

        let result = segment_image.execute(image_input).await.unwrap();

        assert_eq!(result.dimensions.width, 2);
        assert_eq!(result.dimensions.height, 2);
        assert_eq!(result.mask, vec![255, 0, 255, 0]);
    }

    #[tokio::test]
    async fn test_segment_image_with_invalid_image() {
        let invalid_bytes = vec![1, 2, 3, 4, 5];

        let mock_output = Array4::<f32>::zeros((1, 1, 1, 1));
        let mock_inferer = Arc::new(MockInferer {
            return_value: mock_output,
        });

        let segment_image = SegmentImage::new(mock_inferer);
        let image_input = ImageInput {
            image_bytes: invalid_bytes,
        };

        let result = segment_image.execute(image_input).await;
        assert!(result.is_err());
    }

    struct ErrorInferer;

    #[async_trait]
    impl Inferer for ErrorInferer {
        async fn infer(&self, _input: Array4<f32>) -> Result<Array4<f32>> {
            Err(anyhow::anyhow!("Inference failed"))
        }
    }

    #[tokio::test]
    async fn test_segment_image_handles_inference_error() {
        // Create a valid test image
        let img = ImageBuffer::from_fn(5, 5, |_x, _y| Rgb([100u8, 100u8, 100u8]));
        let mut bytes = Vec::new();
        img.write_to(
            &mut std::io::Cursor::new(&mut bytes),
            image::ImageFormat::Png,
        )
        .unwrap();

        let error_inferer = Arc::new(ErrorInferer);
        let segment_image = SegmentImage::new(error_inferer);
        let image_input = ImageInput { image_bytes: bytes };

        let result = segment_image.execute(image_input).await;
        assert!(result.is_err());
    }
}
