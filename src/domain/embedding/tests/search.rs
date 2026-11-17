use super::*;

const MODEL: &str = "gemini-embedding-001";

fn index() -> VectorIndex {
    VectorIndex::from_rows([
        (doc("ethics"), stored(0, MODEL, "h", &[1.0, 0.0, 0.0])),
        (doc("ethics"), stored(1, MODEL, "h", &[0.0, 1.0, 0.0])),
        (doc("optics"), stored(0, MODEL, "h", &[0.9, 0.1, 0.0])),
        (post("p1"), stored(0, MODEL, "h", &[0.0, 0.0, 1.0])),
        (post("p2"), stored(0, MODEL, "h", &[0.1, 0.0, 0.9])),
        (post("old"), stored(0, "other-model", "h", &[1.0, 0.0, 0.0])),
    ])
}

#[test]
fn cosine_dot_and_normalisation() {
    assert!((cosine(&[1.0, 0.0], &[1.0, 0.0]) - 1.0).abs() < 1e-6);
    assert!(cosine(&[1.0, 0.0], &[0.0, 2.0]).abs() < 1e-6);
    assert!((cosine(&[1.0, 1.0], &[-1.0, -1.0]) + 1.0).abs() < 1e-6);
    assert_eq!(cosine(&[0.0, 0.0], &[1.0, 0.0]), 0.0, "a zero vector is similar to nothing");
    assert_eq!(cosine(&[1.0], &[1.0, 0.0]), 0.0, "different lengths never compare");
    let unit = l2_normalized(vec![3.0, 4.0]);
    assert!((dot(&unit, &unit) - 1.0).abs() < 1e-6);
    assert_eq!(l2_normalized(vec![0.0, 0.0]), vec![0.0, 0.0]);
    assert_eq!(mean_vector(&[&[1.0, 3.0], &[3.0, 5.0]]), Some(vec![2.0, 4.0]));
    assert_eq!(mean_vector(&[&[1.0], &[1.0, 2.0]]), None);
}

#[test]
fn blobs_round_trip_as_little_endian_f32() {
    let vector = vec![0.5_f32, -1.25, 3.0e-7];
    let bytes = encode_f32_le(&vector);
    assert_eq!(bytes.len(), 12);
    assert_eq!(&bytes[..4], &0.5_f32.to_le_bytes());
    assert_eq!(decode_f32_le(&bytes), Some(vector));
    assert_eq!(decode_f32_le(&[1, 2, 3]), None);
}

#[test]
fn top_k_ranks_owners_by_their_best_chunk_of_the_same_model() {
    let hits = top_k(&index(), &[0.0, 1.0, 0.0], MODEL, 2, |_| true);
    assert_eq!(hits[0].owner, doc("ethics"));
    assert_eq!(hits[0].best_chunk, 1, "the second chunk of ethics matches");
    assert!((hits[0].score - 1.0).abs() < 1e-5);
    assert_eq!(hits.len(), 2);
    let all = top_k(&index(), &[1.0, 0.0, 0.0], MODEL, 10, |_| true);
    assert!(all.iter().all(|hit| hit.owner != post("old")), "other models are skipped");
    assert!(all.windows(2).all(|pair| pair[0].score >= pair[1].score));
    let posts_only = top_k(&index(), &[1.0, 0.0, 0.0], MODEL, 10, |owner| owner.kind == OwnerKind::Post);
    assert!(posts_only.iter().all(|hit| hit.owner.kind == OwnerKind::Post));
    assert!(top_k(&index(), &[1.0, 0.0], MODEL, 10, |_| true).is_empty(), "query of other dimensions");
}

#[test]
fn similar_items_exclude_themselves_and_keep_their_kind() {
    let index = index();
    let posts = similar_posts(&index, MODEL, &crate::domain::PostId::new("p1"), 5);
    assert_eq!(posts.iter().map(|hit| hit.owner.clone()).collect::<Vec<_>>(), vec![post("p2")]);
    let docs = similar_documents(&index, MODEL, "optics", 5);
    assert_eq!(docs[0].owner, doc("ethics"));
    assert!(similar_documents(&index, MODEL, "missing", 5).is_empty());
    let any = similar_to(&index, &doc("optics"), MODEL, 1, |_| true);
    assert_eq!(any.len(), 1);
}

#[test]
fn owner_vectors_give_one_unit_vector_per_owner_of_a_kind() {
    let vectors = owner_vectors(&index(), OwnerKind::Document, MODEL);
    assert_eq!(vectors.len(), 2);
    let (owner, ethics) = &vectors[0];
    assert_eq!(owner, &doc("ethics"));
    assert!((dot(ethics, ethics) - 1.0).abs() < 1e-5);
    assert!((ethics[0] - ethics[1]).abs() < 1e-6, "the mean of its two chunks");
    assert!(owner_vectors(&index(), OwnerKind::Post, "other-model").iter().all(|(o, _)| o == &post("old")));
}

#[test]
fn the_index_upserts_trims_and_removes_by_value() {
    let index = index().with_chunks([(doc("ethics"), stored(1, MODEL, "new", &[0.0, 0.0, 1.0]))]);
    assert_eq!(index.chunks(&doc("ethics")).len(), 2);
    assert_eq!(index.chunk(&doc("ethics"), 1).map(|c| c.content_hash.as_str()), Some("new"));
    let index = index.truncated(&doc("ethics"), 1).without(&post("p1"));
    assert_eq!(index.chunks(&doc("ethics")).len(), 1);
    assert!(index.chunks(&post("p1")).is_empty());
    assert_eq!(index.len(), 4);
    assert_eq!(index.count_for_model(MODEL), 3);
}
