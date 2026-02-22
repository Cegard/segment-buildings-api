use crate::application::definitions::Inferer;
use crate::constants::ENV_MODEL_PATH;
use anyhow::Result;
use ndarray::{Array4, Ix4};
use ort::{ep::CPU, ep::CUDA, inputs, session::Session, value::Tensor};
use std::env;
use tokio::sync::{Mutex, Semaphore};

pub struct OrtSegmenter {
    pub session: Mutex<Session>,
    pub semaphore: Semaphore,
}

impl OrtSegmenter {
    pub fn new(will_run_on_cuda: bool, max_concurrent_calls: Option<usize>) -> Result<Self> {
        let mut providers = vec![CPU::default().build()];
        let model_path = env::var(ENV_MODEL_PATH)?;

        if will_run_on_cuda {
            providers.insert(0, CUDA::default().build());
        }

        let session = Session::builder()?
            .with_execution_providers(providers)?
            .commit_from_file(model_path)?;

        Ok(Self {
            session: Mutex::new(session),
            semaphore: Semaphore::new(max_concurrent_calls.unwrap_or(1)),
        })
    }
}

#[async_trait::async_trait]
impl Inferer for OrtSegmenter {
    async fn infer(&self, input: Array4<f32>) -> Result<Array4<f32>> {
        let _permit = self.semaphore.acquire().await?;

        let mut session = self.session.lock().await;

        let tensor = Tensor::from_array(input)?;
        let outputs = session.run(inputs![super::constants::SESSION_INPUT_NAME => tensor])?;
        let output = outputs[super::constants::SESSION_OUTPUT_NAME].try_extract_array::<f32>()?;

        let owned = output.into_dimensionality::<Ix4>()?.to_owned();

        Ok(owned)
    }
}
