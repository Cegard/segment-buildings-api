use anyhow::Result;
use ndarray::Array4;

#[async_trait::async_trait]
pub trait Inferer: Send + Sync {
    async fn infer(&self, input: Array4<f32>) -> Result<Array4<f32>>;
}
