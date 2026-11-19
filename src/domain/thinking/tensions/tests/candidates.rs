use super::super::*;
use super::{close, item};

const SLEEP: &str = "Exercise improves sleep quality at night.";
const SLEEP_NOT: &str = "Exercise does not improve sleep quality at night.";
const SLEEP_NEVER: &str = "Exercise never improves sleep quality at night.";

fn keys(candidates: &[Candidate]) -> Vec<(String, String)> {
    candidates.iter().map(|c| (c.left.key(), c.right.key())).collect()
}

#[test]
fn without_vectors_pairs_come_from_shared_words() {
    let items = [item("a", SLEEP, None), item("b", SLEEP_NOT, None), item("c", "Banana bread recipe with walnuts.", None)];
    let ranked = rank_candidates(&items, &RankOptions::default());
    assert_eq!(keys(&ranked), vec![("post:a".to_string(), "post:b".to_string())]);
    let only = &ranked[0];
    assert_eq!(only.similarity, None);
    assert!(only.lexical.negation);
    assert!(close(only.score, 0.45 * 1.0 + 0.55 * 0.4), "jaccard 1 plus a negation: {}", only.score);
}

#[test]
fn with_vectors_pairs_come_from_meaning_only() {
    let items = [
        item("a", "Coffee helps concentration in the morning.", Some(vec![1.0, 0.0])),
        item("b", "Coffee harms concentration in the morning.", Some(vec![0.99, 0.141])),
        item("c", "Coffee helps concentration in the morning, truly.", Some(vec![0.0, 1.0])),
    ];
    let ranked = rank_candidates(&items, &RankOptions::default());
    assert_eq!(keys(&ranked), vec![("post:a".to_string(), "post:b".to_string())], "a and c share words but not meaning");
    let similarity = ranked[0].similarity.expect("found by meaning");
    assert!(similarity > 0.98);
    assert_eq!(ranked[0].lexical.antonyms, vec![("help".to_string(), "harm".to_string())]);
    assert!(close(ranked[0].score, candidate_score(similarity, &ranked[0].lexical)));
}

#[test]
fn a_very_close_pair_is_kept_without_a_signal_and_a_loose_one_is_not() {
    let items = [
        item("a", "The team meets on Monday mornings.", Some(vec![1.0, 0.0])),
        item("b", "The team gathers on Monday mornings.", Some(vec![1.0, 0.0])),
        item("c", "Budgets are planned yearly.", Some(vec![0.6, 0.8])),
    ];
    let ranked = rank_candidates(&items, &RankOptions::default());
    assert_eq!(keys(&ranked), vec![("post:a".to_string(), "post:b".to_string())]);
    assert!(ranked[0].lexical.is_empty());
}

#[test]
fn an_item_without_a_vector_still_pairs_by_words() {
    let items = [
        item("a", "Remote work boosts productivity overall.", Some(vec![1.0, 0.0])),
        item("b", "Remote work never boosts productivity overall.", None),
    ];
    let ranked = rank_candidates(&items, &RankOptions::default());
    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0].similarity, None);
    assert!(ranked[0].lexical.negation);
}

#[test]
fn focus_keeps_its_pairs_first_and_an_unknown_focus_finds_nothing() {
    let items = [item("a", SLEEP, None), item("b", SLEEP_NOT, None), item("c", SLEEP_NEVER, None)];
    let all = rank_candidates(&items, &RankOptions::default());
    assert_eq!(all.len(), 2, "b and c are both negated, so they agree: {:?}", keys(&all));
    assert!(all.windows(2).all(|w| w[0].score >= w[1].score));
    let focused = rank_candidates(&items, &RankOptions { focus: Some("post:b".into()), ..RankOptions::default() });
    assert_eq!(keys(&focused), vec![("post:b".to_string(), "post:a".to_string())]);
    let unknown = RankOptions { focus: Some("post:zzz".into()), ..RankOptions::default() };
    assert!(rank_candidates(&items, &unknown).is_empty());
    let limited = RankOptions { limit: 1, ..RankOptions::default() };
    assert_eq!(rank_candidates(&items, &limited).len(), 1);
}

#[test]
fn identical_texts_and_repeated_keys_never_pair() {
    let same_text = [item("a", "The team meets on Monday.", Some(vec![1.0, 0.0])), item("b", "the team  meets on Monday.", Some(vec![1.0, 0.0]))];
    assert!(rank_candidates(&same_text, &RankOptions::default()).is_empty());
    let same_key = [item("a", "The team meets on Monday.", Some(vec![1.0, 0.0])), item("a", "The team gathers on Monday.", Some(vec![1.0, 0.0]))];
    assert!(rank_candidates(&same_key, &RankOptions::default()).is_empty());
}

#[test]
fn each_item_keeps_at_most_its_nearest_neighbours() {
    let focus = item("f", "Coffee helps concentration.", Some(vec![1.0, 0.0]));
    let others = (1..=12).map(|i| item(&format!("o{i}"), &format!("Coffee harms concentration, case {i}."), Some(vec![1.0, 0.01 * i as f32])));
    let items: Vec<CandidateInput> = std::iter::once(focus).chain(others).collect();
    let ranked = rank_candidates(&items, &RankOptions { focus: Some("post:f".into()), ..RankOptions::default() });
    assert_eq!(ranked.len(), NEIGHBOURS);
    assert!(ranked.iter().all(|c| c.left.key() == "post:f"));
    assert_eq!(ranked[0].right.key(), "post:o1", "the nearest comes first");
}

#[test]
fn the_score_weighs_topic_and_opposition() {
    let negated = LexicalSignals { negation: true, ..LexicalSignals::default() };
    assert!(close(candidate_score(0.5, &negated), 0.45 * 0.5 + 0.55 * 0.4));
    assert!(close(candidate_score(2.0, &LexicalSignals::default()), 0.45), "the topic is clamped");
}

#[test]
fn passages_pair_with_posts() {
    let items = [
        CandidateInput { side: TensionSide::passage("doc", 2, "Ethics", "Freedom is understanding necessity."), vector: None },
        item("p", "Freedom is not understanding necessity.", None),
    ];
    let ranked = rank_candidates(&items, &RankOptions::default());
    assert_eq!(keys(&ranked), vec![("passage:doc#2".to_string(), "post:p".to_string())]);
}
