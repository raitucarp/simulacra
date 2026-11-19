//! The embedding HTTP client: provider dispatch, batching to the provider's limit, retries
//! with backoff on rate limits and server errors, and unit-length results.

use super::backoff::retry_delay;
use super::config::{EmbeddingConfig, ProviderKind};
use super::error::EmbeddingError;
use super::mock::mock_vector;
use super::task::TaskType;
use super::wire::{gemini, local, openai, EmbedInput, WireRequest};
use crate::domain::embedding::{l2_normalized, FailureKind};
use serde_json::Value;

/// Header with the provider's requested wait.
const RETRY_AFTER_HEADER: &str = "retry-after";

/// Embeds texts with one configuration.
#[derive(Debug, Clone)]
pub struct EmbeddingClient {
    config: EmbeddingConfig,
    http: reqwest::Client,
}

impl EmbeddingClient {
    pub fn new(config: EmbeddingConfig) -> Self {
        let http = reqwest::Client::builder().timeout(config.timeout).build().unwrap_or_else(|_| reqwest::Client::new());
        Self { config, http }
    }

    pub fn config(&self) -> &EmbeddingConfig {
        &self.config
    }

    /// One unit vector per input, in order.
    pub async fn embed(&self, task: TaskType, inputs: &[EmbedInput]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        self.config.readiness().map_err(EmbeddingError::local)?;
        if self.config.provider == ProviderKind::Mock {
            return Ok(inputs.iter().map(|input| mock_vector(&input.text, self.config.dimensions as usize)).collect());
        }
        let mut vectors = Vec::with_capacity(inputs.len());
        for batch in inputs.chunks(self.max_batch()) {
            let request = self.request(task, batch);
            vectors.extend(self.send_with_retries(&request, batch.len()).await?);
        }
        Ok(vectors.into_iter().map(l2_normalized).collect())
    }

    fn max_batch(&self) -> usize {
        match self.config.provider {
            ProviderKind::Gemini => gemini::MAX_BATCH,
            ProviderKind::LocalServer => local::MAX_BATCH,
            ProviderKind::OpenAiCompatible | ProviderKind::Mock => openai::MAX_BATCH,
        }
    }

    fn request(&self, task: TaskType, batch: &[EmbedInput]) -> WireRequest {
        match self.config.provider {
            ProviderKind::Gemini => gemini::request(&self.config, task, batch),
            ProviderKind::LocalServer => local::request(&self.config, task, batch),
            ProviderKind::OpenAiCompatible | ProviderKind::Mock => openai::request(&self.config, task, batch),
        }
    }

    fn parse(&self, reply: &Value, expected: usize) -> Result<Vec<Vec<f32>>, String> {
        match self.config.provider {
            ProviderKind::Gemini => gemini::parse(reply, expected),
            ProviderKind::LocalServer => local::parse(reply, expected),
            ProviderKind::OpenAiCompatible | ProviderKind::Mock => openai::parse(reply, expected),
        }
    }

    /// Sends `request`, retrying retryable failures up to `max_retries` times.
    async fn send_with_retries(&self, request: &WireRequest, expected: usize) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut attempt = 0;
        loop {
            match self.send_once(request, expected).await {
                Err(err) if err.kind.is_retryable() && attempt < self.config.max_retries => {
                    tokio::time::sleep(retry_delay(attempt, err.retry_after)).await;
                    attempt += 1;
                }
                outcome => return outcome,
            }
        }
    }

    async fn send_once(&self, request: &WireRequest, expected: usize) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let secrets = [self.config.api_key.clone()];
        let builder = request.headers.iter().fold(self.http.post(&request.url), |b, (name, value)| b.header(*name, value));
        let response = builder.json(&request.body).send().await.map_err(|e| EmbeddingError::network(&e.to_string(), &secrets))?;
        let status = response.status();
        if !status.is_success() {
            let retry_after = response.headers().get(RETRY_AFTER_HEADER).and_then(|v| v.to_str().ok()).map(str::to_string);
            let body = response.text().await.unwrap_or_default();
            return Err(EmbeddingError::from_status(status.as_u16(), &body, retry_after.as_deref(), &secrets));
        }
        let reply: Value = response.json().await.map_err(|e| EmbeddingError::unreadable(&e.to_string(), &secrets))?;
        self.parse(&reply, expected).map_err(|e| EmbeddingError::unreadable(&e, &secrets))
    }
}

/// [`EmbeddingClient::embed`] on a fresh single-thread runtime, for a background thread.
pub fn embed_blocking(client: &EmbeddingClient, task: TaskType, inputs: &[EmbedInput]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| EmbeddingError { detail: e.to_string(), ..EmbeddingError::local(FailureKind::Network) })?;
    runtime.block_on(client.embed(task, inputs))
}
