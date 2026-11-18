use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::concept::{concept_key, concepts_of, marked_concepts, mentions};

#[test]
fn the_ladder_climbs_and_descends_within_its_ends() {
    assert_eq!(BloomLevel::Remember.rank(), 1);
    assert_eq!(BloomLevel::Create.rank(), 6);
    assert_eq!(BloomLevel::Remember.next(), BloomLevel::Understand);
    assert_eq!(BloomLevel::Create.next(), BloomLevel::Create);
    assert_eq!(BloomLevel::Apply.previous(), BloomLevel::Understand);
    assert_eq!(BloomLevel::Remember.previous(), BloomLevel::Remember);
    assert!(BloomLevel::Analyse > BloomLevel::Apply, "variant order is the ladder");
    assert!((BloomLevel::Create.share() - 1.0).abs() < 1e-9);
}

#[test]
fn levels_keep_their_wire_keys() {
    for level in BloomLevel::ALL {
        assert_eq!(serde_json::to_string(level).unwrap(), format!("\"{}\"", level.key()));
        assert_eq!(BloomLevel::from_key(level.key()), Some(*level));
    }
}

#[test]
fn concept_keys_compare_names_written_differently() {
    assert_eq!(concept_key("  #Free_Will "), "free will");
    assert_eq!(concept_key("Free-will"), "free will");
    assert!(mentions("Is [[Free will]] an illusion?", "free will"));
    assert!(mentions("#free_will again", "free will"));
    assert!(!mentions("freedom of the will", "free will"), "whole words only");
    assert!(!mentions("anything", ""));
}

#[test]
fn marked_concepts_come_first_and_once() {
    let text = "On [[Necessity]] and #determinism. Necessity again, with Baruch Spinoza.";
    assert_eq!(marked_concepts(text), vec!["necessity".to_string(), "determinism".to_string()]);
    let all = concepts_of(text);
    assert_eq!(&all[..2], &["necessity".to_string(), "determinism".to_string()]);
    assert!(all.contains(&"baruch spinoza".to_string()), "{all:?}");
    assert_eq!(all.iter().filter(|c| *c == "necessity").count(), 1);
}
