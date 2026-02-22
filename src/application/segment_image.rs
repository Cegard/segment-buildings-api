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
        let input_tensor = super::tool_kit::preprocess(image_input.image_bytes)?;
        let raw_output = self.inferer.infer(input_tensor).await?;
        let segmentation_result = super::tool_kit::postprocess(raw_output)?;
        Ok(segmentation_result)
    }
}
