use super::tool_kit::*;
use crate::application::definitions::Inferer;
use crate::domain::entities::{ImageInput, SegmentationResult};
use anyhow::Result;
use ort::{ep::CPU, ep::CUDA, inputs, session::Session, value::Tensor};
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};

const MODEL_PATH: &str = "../model/model.onnx";

pub struct OrtSegmenter {
    pub session: Arc<Mutex<Session>>,
    pub semaphore: Arc<Semaphore>,
}

impl OrtSegmenter {
    pub fn new(will_run_on_cuda: bool, max_concurrent_calls: Option<usize>) -> Result<Self> {
        let mut providers = vec![CPU::default().build()];

        if will_run_on_cuda {
            providers.insert(0, CUDA::default().build());
        }

        let session = Session::builder()?
            .with_execution_providers(providers)?
            .commit_from_file(MODEL_PATH)?;

        Ok(Self {
            session: Arc::new(Mutex::new(session)),
            semaphore: Arc::new(Semaphore::new(max_concurrent_calls.unwrap_or(1))),
        })
    }
}

#[async_trait::async_trait]
impl Inferer for OrtSegmenter {
    async fn segment(&self, image_input: ImageInput) -> Result<SegmentationResult> {
        let _permit = self.semaphore.acquire().await?;
        let input_data: Vec<usize> = vec![
            image_input.dimensions.height,
            image_input.dimensions.width,
            3,
        ];
        let mut session = self.session.lock().await;
        let input = Tensor::from_array((input_data, image_input.image_bytes))?;
        let outputs = session.run(inputs!["image" => input])?;
        let output_tensor = outputs["output0"].try_extract_array::<f32>()?;
        let result = postprocess(output_tensor)?;

        Ok(result)
    }
}
