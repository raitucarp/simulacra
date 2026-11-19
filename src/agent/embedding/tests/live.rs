//! The real Gemini API (ignored by default): `GEMINI_API_KEY=… cargo test -- --ignored live`.
//! The key is read from the environment and never printed.

use super::*;
use crate::agent::llm::wire::gemini::GEMINI_MODELS_URL;
use crate::domain::embedding::cosine;

const KEY_VARIABLE: &str = "GEMINI_API_KEY";

#[tokio::test]
#[ignore = "calls the real Gemini API; set GEMINI_API_KEY"]
async fn live_gemini_embeds_three_texts_in_a_sensible_order() {
    let Ok(api_key) = std::env::var(KEY_VARIABLE) else {
        eprintln!("{KEY_VARIABLE} is not set; skipping the live check");
        return;
    };
    let config = EmbeddingConfig { api_key, timeout: Duration::from_secs(30), ..config(ProviderKind::Gemini, "gemini-embedding-001", GEMINI_MODELS_URL) };
    let texts = [EmbedInput::text("The cat sat on the mat."), EmbedInput::text("A kitten rested on the rug."), EmbedInput::text("Quarterly tax filing deadlines.")];
    let vectors = match EmbeddingClient::new(config).embed(TaskType::SemanticSimilarity, &texts).await {
        Ok(vectors) => vectors,
        Err(err) => panic!("live embedding failed: {:?} (status {:?})", err.kind, err.status),
    };
    assert_eq!(vectors.len(), 3);
    assert!(vectors.iter().all(|v| v.len() == 768), "768 dimensions were asked for");
    assert!(cosine(&vectors[0], &vectors[1]) > cosine(&vectors[0], &vectors[2]), "the two cat sentences are closest");
}
