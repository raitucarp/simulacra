use super::*;

fn no_posts(_: &str) -> bool {
    false
}

#[test]
fn wikilinks_and_hashtags_come_first() {
    let text = "Thinking about [[Bayesian Epistemology]] and #free_will today, with more considerations.";
    let concepts = extract_concepts(text, no_posts);
    assert_eq!(concepts[0], "Bayesian Epistemology");
    assert_eq!(concepts[1], "free will");
    assert!(concepts.contains(&"considerations".to_string()));
}

#[test]
fn links_to_other_thoughts_are_not_concepts() {
    let concepts = extract_concepts("See [[p-123]] about [[Stoicism]].", |link| link.starts_with("p-"));
    assert_eq!(concepts.first().map(String::as_str), Some("Stoicism"));
    assert!(!concepts.iter().any(|c| c == "p-123"));
}

#[test]
fn capitalised_phrases_are_kept_but_sentence_openers_are_not() {
    let text = "Yesterday I reread what Baruch Spinoza wrote. Necessity matters. The Ethics is hard.";
    let concepts = extract_concepts(text, no_posts);
    assert!(concepts.contains(&"Baruch Spinoza".to_string()), "{concepts:?}");
    assert!(concepts.contains(&"Ethics".to_string()), "{concepts:?}");
    assert!(!concepts.contains(&"Yesterday".to_string()), "{concepts:?}");
    let possessive = extract_concepts("Even Baruch Spinoza's critics admit it.", no_posts);
    assert_eq!(possessive.first().map(String::as_str), Some("Baruch Spinoza"), "{possessive:?}");
}

#[test]
fn content_words_rank_by_frequency_and_skip_noise() {
    let text = "Determinism shapes freedom; determinism also shapes responsibility. Something else entirely 2024000.";
    let concepts = extract_concepts(&text.to_lowercase(), no_posts);
    assert_eq!(concepts[0], "determinism");
    assert!(concepts.contains(&"responsibility".to_string()));
    assert!(!concepts.iter().any(|c| c == "something" || c == "2024000"));
}

#[test]
fn concepts_are_unique_capped_and_safe_on_any_script() {
    let many = (0..30).map(|i| format!("#topic{i}")).collect::<Vec<_>>().join(" ");
    assert_eq!(extract_concepts(&many, no_posts).len(), MAX_CONCEPTS);
    assert_eq!(extract_concepts("#Freedom #freedom [[freedom]]", no_posts), vec!["freedom".to_string()]);
    assert!(extract_concepts("自由とは必然性の理解である。", no_posts).len() <= MAX_CONCEPTS);
    assert!(extract_concepts("", no_posts).is_empty());
    assert!(extract_concepts("[[unclosed and ]] #", no_posts).iter().all(|c| !c.is_empty()));
}
