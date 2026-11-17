mod classify;
mod config;
mod live;
mod served;
mod wire;

use super::*;
use std::time::Duration;

/// A configuration for `provider` against `endpoint` with a test key.
pub(super) fn config(provider: ProviderKind, model: &str, endpoint: &str) -> EmbeddingConfig {
    EmbeddingConfig {
        provider,
        model: model.into(),
        endpoint: endpoint.into(),
        api_key: "test-key-0000".into(),
        dimensions: 768,
        timeout: Duration::from_secs(5),
        max_retries: 2,
    }
}

pub(super) fn inputs() -> Vec<EmbedInput> {
    vec![EmbedInput { title: "Ethics".into(), text: "God is substance.".into() }, EmbedInput::text("Is the mind a graph?")]
}
