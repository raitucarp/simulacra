//! Visible length: markup is free, what the reader sees counts.

use super::*;

#[test]
fn marks_escapes_and_fences_do_not_count() {
    assert_eq!(visible_text_len("**bold** *it* __u__ ||s|| `c`"), "bold it u s c".chars().count());
    assert_eq!(visible_text_len(r"\*literal\*"), "*literal*".len());
    assert_eq!(visible_text_len("```rust\nlet x = 1;\n```"), "let x = 1;".len());
    assert_eq!(visible_text_len("**unclosed"), "**unclosed".len());
}

#[test]
fn links_count_their_target_and_tags_count_in_full() {
    assert_eq!(visible_text_len("see [[ post_1 ]]."), "see post_1.".len());
    assert_eq!(visible_text_len("#tag!"), "#tag!".len());
    let mention = BodyToken::Mention { handle: "@ann".to_string(), trailing: ",".to_string() };
    let url = BodyToken::Url { url: "https://a.b".to_string(), trailing: String::new() };
    assert_eq!(visible_len(&[mention, url]), "@ann,https://a.b".len());
}

#[test]
fn plain_text_counts_characters_not_bytes() {
    assert_eq!(visible_text_len("  実体は一つ  "), 5);
    assert_eq!(visible_text_len("é**é**"), 2);
    assert_eq!(visible_text_len(""), 0);
}

