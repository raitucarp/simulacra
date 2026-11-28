//! Tier A, embeddings: similarity order with the configured model, `gemini-embedding-2`'s
//! task-prefix encoding on a retrieval pair, and batching past the 100-text request limit.

use super::harness::{block_on, ensure, pass, run, Key, Model, Tier, GEMINI_MODELS_URL};
use simulacra::agent::embedding::{EmbedInput, EmbeddingClient, EmbeddingConfig, ProviderKind, TaskEncoding, TaskType};
use simulacra::domain::embedding::cosine;
use std::time::Duration;

const DIMENSIONS: u32 = 768;
const PREFIX_MODEL: &str = "gemini-embedding-2";
/// More texts than one `batchEmbedContents` call takes (100), so two requests go out.
const BATCH_TEXTS: usize = 120;
const UNIT_TOLERANCE: f32 = 1e-3;

/// A Gemini embedding configuration for `model`, built in memory.
pub fn config(model: &str, api_key: &str) -> EmbeddingConfig {
    EmbeddingConfig {
        provider: ProviderKind::Gemini,
        model: model.into(),
        endpoint: GEMINI_MODELS_URL.into(),
        api_key: api_key.into(),
        dimensions: DIMENSIONS,
        timeout: Duration::from_secs(60),
        max_retries: 2,
    }
}

pub fn embed(config: EmbeddingConfig, task: TaskType, inputs: &[EmbedInput]) -> Result<Vec<Vec<f32>>, String> {
    block_on(EmbeddingClient::new(config).embed(task, inputs)).map_err(|e| format!("{:?} (status {:?}): {}", e.kind, e.status, e.detail))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a07_embeddings_rank_similar_texts_closest() {
    run(Tier::A, "embeddings: similarity order", Model::Embed, Key::Needed, |live| {
        let texts = ["The cat sat on the mat.", "A kitten rested on the rug.", "Quarterly tax filing deadlines."].map(EmbedInput::text);
        let vectors = embed(config(&live.embed_model, live.key()), TaskType::SemanticSimilarity, &texts)?;
        ensure(vectors.len() == 3 && vectors.iter().all(|v| v.len() == DIMENSIONS as usize), || format!("{} vectors of the wrong size", vectors.len()))?;
        let (near, far) = (cosine(&vectors[0], &vectors[1]), cosine(&vectors[0], &vectors[2]));
        ensure(near > far, || format!("cat/kitten {near:.3} <= cat/tax {far:.3}"))?;
        pass(format!("cosine cat/kitten {near:.3} > cat/tax {far:.3}"))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a08_gemini_embedding_2_reads_task_prefixes() {
    run(Tier::A, "embeddings: gemini-embedding-2 prefix encoding", Model::Fixed(PREFIX_MODEL), Key::Needed, |live| {
        let config = config(PREFIX_MODEL, live.key());
        ensure(config.encoding() == TaskEncoding::Prefix, || format!("{PREFIX_MODEL} should use task prefixes, got {:?}", config.encoding()))?;
        let docs = [
            EmbedInput { title: "Ethics".into(), text: "Spinoza wrote the Ethics, arguing that God and Nature are one substance.".into() },
            EmbedInput { title: "Bread".into(), text: "Knead the dough, let it rise, then bake it hot.".into() },
        ];
        let doc_vectors = embed(config.clone(), TaskType::RetrievalDocument, &docs)?;
        let query = embed(config, TaskType::RetrievalQuery, &[EmbedInput::text("Who wrote the Ethics?")])?;
        let (hit, miss) = (cosine(&query[0], &doc_vectors[0]), cosine(&query[0], &doc_vectors[1]));
        ensure(hit > miss, || format!("the Ethics passage ranked below bread: {hit:.3} <= {miss:.3}"))?;
        pass(format!("query/ethics {hit:.3} > query/bread {miss:.3}"))
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn a09_embeddings_batch_past_the_request_limit() {
    run(Tier::A, "embeddings: 120 texts in two batches", Model::Embed, Key::Needed, |live| {
        let inputs: Vec<EmbedInput> = (0..BATCH_TEXTS).map(|i| EmbedInput::text(format!("Note {i}: thought number {i} about lenses and light."))).collect();
        let vectors = embed(config(&live.embed_model, live.key()), TaskType::RetrievalDocument, &inputs)?;
        ensure(vectors.len() == BATCH_TEXTS, || format!("{} vectors for {BATCH_TEXTS} texts", vectors.len()))?;
        let unit = vectors.iter().all(|v| (v.iter().map(|x| x * x).sum::<f32>().sqrt() - 1.0).abs() < UNIT_TOLERANCE);
        ensure(unit, || "a vector is not unit length".to_string())?;
        pass(format!("{} unit vectors of {DIMENSIONS} dimensions", vectors.len()))
    });
}
