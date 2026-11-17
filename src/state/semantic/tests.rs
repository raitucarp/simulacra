use super::*;
use crate::domain::embedding::{ChunkSpec, ChunkUnit, EmbeddingOwner, OwnerKind, ScoredOwner};
use crate::state::AppState;

fn spec() -> ChunkSpec {
    ChunkSpec { size: 512, overlap: 64, unit: ChunkUnit::Tokens }
}

fn state() -> AppState {
    let mut state = AppState::new_in_memory();
    state.add_document("Optics", "Light bends when it enters water. Lenses focus light.").expect("document");
    state.add_document("Ethics", "Substance is one. God or nature.").expect("document");
    state
}

#[test]
fn sources_cover_posts_with_text_and_every_document() {
    let state = state();
    let sources = index_sources(&state);
    let documents = sources.iter().filter(|s| s.owner.kind == OwnerKind::Document).count();
    assert_eq!(documents, state.documents.len());
    assert!(sources.iter().all(|s| !s.text.trim().is_empty()));
    let optics = sources.iter().find(|s| s.title == "Optics").expect("titled document");
    assert!(optics.text.contains("Lenses"));
}

#[test]
fn the_index_key_changes_with_content_and_configuration() {
    let mut state = state();
    let before = IndexKey::of(&state, "gemini|768");
    assert_eq!(before, IndexKey::of(&state, "gemini|768"));
    assert!(!before.same_config(&IndexKey::of(&state, "local|768")));
    state.add_document("Third", "More text.").expect("document");
    let after = IndexKey::of(&state, "gemini|768");
    assert_ne!(before, after);
    assert!(before.same_config(&after));
}

#[test]
fn hits_resolve_titles_excerpts_and_drop_missing_owners() {
    let state = state();
    let optics = state.documents.iter().find(|d| d.title == "Optics").unwrap().id.clone();
    let scored = vec![
        ScoredOwner { owner: EmbeddingOwner::document(optics.clone()), score: 1.4, best_chunk: 0 },
        ScoredOwner { owner: EmbeddingOwner::document("gone"), score: 0.9, best_chunk: 0 },
    ];
    let hits = hits_for(&state, &scored, spec());
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].target, HitTarget::Document(optics));
    assert_eq!(hits[0].score, 1.0, "scores are clamped into 0..=1");
    assert!(hits[0].excerpt.starts_with("Light bends"));
    assert!(state.is_in_active_network(&hits[0].target.owner()));
}

#[test]
fn the_word_fallback_ranks_by_bm25_relative_to_the_best() {
    let state = state();
    let hits = lexical_hits(&state, "light lenses", 5, spec());
    assert_eq!(hits.first().map(|h| h.title.as_str()), Some("Optics"));
    assert_eq!(hits[0].score, 1.0);
    assert!(hits.iter().all(|h| (0.0..=1.0).contains(&h.score)));
    assert!(lexical_hits(&state, "zzzqqq", 5, spec()).is_empty());
}

/// Plan, embed through the Gemini wire against the local mock server, store, search: the
/// whole indexing pipeline without a real provider.
#[test]
fn indexing_end_to_end_against_a_mock_gemini_server() {
    use crate::agent::embedding::mock::mock_vector;
    use crate::agent::embedding::{embed_blocking, EmbedInput, EmbeddingClient, EmbeddingConfig, ProviderKind, TaskType};
    use crate::agent::mock_server::{MockReply, MockServer};
    use crate::domain::embedding::{plan_index, top_k, PlanSpec, StoredChunk, VectorIndex};
    use serde_json::json;

    let state = state();
    let sources: Vec<_> = index_sources(&state).into_iter().filter(|s| s.owner.kind == OwnerKind::Document).collect();
    let plan_spec = PlanSpec { chunk: spec(), model: "gemini-embedding-001".into(), signature: "gemini|64|field".into(), force: false };
    let plan = plan_index(&sources, &VectorIndex::default(), &plan_spec);
    let reply = |texts: Vec<&str>| MockReply::json(json!({ "embeddings": texts.iter().map(|t| json!({ "values": mock_vector(t, 64) })).collect::<Vec<_>>() }));
    let server = MockServer::start(vec![reply(plan.jobs.iter().map(|j| j.text.as_str()).collect()), reply(vec!["light lenses"])]);
    let config = EmbeddingConfig {
        provider: ProviderKind::Gemini, model: "gemini-embedding-001".into(), endpoint: server.url.clone(), api_key: "test-key-0000".into(),
        dimensions: 64, timeout: std::time::Duration::from_secs(5), max_retries: 0,
    };
    let client = EmbeddingClient::new(config);
    let inputs: Vec<EmbedInput> = plan.jobs.iter().map(|j| EmbedInput { title: j.title.clone(), text: j.text.clone() }).collect();
    let vectors = embed_blocking(&client, TaskType::RetrievalDocument, &inputs).expect("documents embedded");
    let rows = plan.jobs.iter().zip(vectors).map(|(job, vector)| {
        (job.owner.clone(), StoredChunk { index: job.chunk_index, model: plan_spec.model.clone(), content_hash: job.content_hash.clone(), vector })
    });
    let index = VectorIndex::default().with_chunks(rows);
    assert!(plan_index(&sources, &index, &plan_spec).jobs.is_empty(), "unchanged text is not embedded again");

    let query = embed_blocking(&client, TaskType::RetrievalQuery, &[EmbedInput::text("light lenses")]).expect("query embedded");
    let ranked = top_k(&index, &query[0], &plan_spec.model, 3, |owner| state.is_in_active_network(owner));
    let hits = hits_for(&state, &ranked, spec());
    assert_eq!(hits.first().map(|h| h.title.as_str()), Some("Optics"));
    let requests = server.requests();
    assert_eq!(requests[0].json()["requests"][0]["taskType"], "RETRIEVAL_DOCUMENT");
    assert_eq!(requests[1].json()["requests"][0]["taskType"], "RETRIEVAL_QUERY");
}
