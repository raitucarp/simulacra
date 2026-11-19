use super::*;

const MODEL: &str = "gemini-embedding-001";

fn spec(model: &str, force: bool) -> PlanSpec {
    PlanSpec { chunk: ChunkSpec { size: 40, overlap: 0, unit: ChunkUnit::Characters }, model: model.into(), signature: "gemini|768".into(), force }
}

fn sources() -> Vec<SourceText> {
    vec![
        SourceText { owner: post("p1"), title: String::new(), text: "Is the mind a graph of ideas?".into() },
        SourceText { owner: doc("d1"), title: "Ethics".into(), text: "God is substance. Substance is one. Mind and body are one.".into() },
    ]
}

/// The index after embedding every job of `plan` with `model`.
fn applied(index: VectorIndex, plan: &IndexPlan, model: &str) -> VectorIndex {
    index.with_chunks(plan.jobs.iter().map(|job| (job.owner.clone(), stored(job.chunk_index, model, &job.content_hash, &[1.0, 0.0]))))
}

#[test]
fn content_hash_is_stable_and_separates_parts() {
    assert_eq!(content_hash(&["abc"]), content_hash(&["abc"]));
    assert_eq!(content_hash(&[""]), "cbf29ce484222325", "FNV-1a offset basis for no bytes");
    assert_ne!(content_hash(&["ab", "c"]), content_hash(&["a", "bc"]));
    assert_eq!(content_hash(&["x"]).len(), 16);
}

#[test]
fn a_fresh_index_plans_every_chunk() {
    let plan = plan_index(&sources(), &VectorIndex::default(), &spec(MODEL, false));
    let doc_chunks = plan.jobs.iter().filter(|job| job.owner == doc("d1")).count();
    assert_eq!(plan.jobs.len(), 1 + doc_chunks);
    assert!(doc_chunks >= 2, "the document spans several chunks");
    assert!(plan.trims.is_empty() && plan.removals.is_empty());
}

#[test]
fn unchanged_text_is_skipped_and_edits_or_a_model_change_are_not() {
    let first = plan_index(&sources(), &VectorIndex::default(), &spec(MODEL, false));
    let index = applied(VectorIndex::default(), &first, MODEL);
    assert!(plan_index(&sources(), &index, &spec(MODEL, false)).jobs.is_empty(), "nothing changed");

    let mut edited = sources();
    edited[0].text = "Is the mind a forest of ideas?".into();
    let plan = plan_index(&edited, &index, &spec(MODEL, false));
    assert_eq!(plan.jobs.iter().map(|job| job.owner.clone()).collect::<Vec<_>>(), vec![post("p1")]);

    let other_model = plan_index(&sources(), &index, &spec("embeddinggemma", false));
    assert_eq!(other_model.jobs.len(), first.jobs.len(), "a model change re-embeds everything");
    let mut other_dims = spec(MODEL, false);
    other_dims.signature = "gemini|1536".into();
    assert_eq!(plan_index(&sources(), &index, &other_dims).jobs.len(), first.jobs.len());
    assert_eq!(plan_index(&sources(), &index, &spec(MODEL, true)).jobs.len(), first.jobs.len(), "a rebuild forces all");
}

#[test]
fn shorter_text_trims_and_deleted_items_are_removed() {
    let first = plan_index(&sources(), &VectorIndex::default(), &spec(MODEL, false));
    let index = applied(VectorIndex::default(), &first, MODEL);
    let shorter = vec![SourceText { owner: doc("d1"), title: "Ethics".into(), text: "God is substance.".into() }];
    let plan = plan_index(&shorter, &index, &spec(MODEL, false));
    assert_eq!(plan.trims, vec![(doc("d1"), 1)]);
    assert_eq!(plan.removals, vec![post("p1")]);
    assert_eq!(plan.jobs.len(), 1, "the first chunk's text changed (it was packed with more)");
}
