use crate::domain::entities::{ImageInput, SegmentationResult};
use anyhow::Result;

#[async_trait::async_trait]
pub trait Inferer: Send + Sync {
    async fn segment(&self, image_btytes: ImageInput) -> Result<SegmentationResult>;
}
