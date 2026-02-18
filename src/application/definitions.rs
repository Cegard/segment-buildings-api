use anyhow::Result;

#[async_trait::async_trait]
pub trait Segmenter: Send + Sync {
    async fn segment(&self, image_btytes: &[u8]) -> Result<Vec<u8>>;
}
