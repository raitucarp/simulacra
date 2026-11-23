//! N1: `((target))` and `((target@version))` embeds.

use super::*;
use crate::domain::data::history::SemVer;

fn embed(target: &str, version: Option<SemVer>, trailing: &str) -> BodyToken {
    BodyToken::Embed { target: target.to_string(), version, trailing: trailing.to_string() }
}

#[test]
fn an_embed_names_its_target_and_may_pin_a_version() {
    assert_eq!(tokenize_post_body("((post_1))"), vec![embed("post_1", None, "")]);
    assert_eq!(tokenize_post_body("see ((post_1@1.2.0)),"), vec![text("see "), embed("post_1", Some(SemVer::new(1, 2, 0)), ",")]);
}

#[test]
fn malformed_embeds_stay_text() {
    for raw in ["((post_1", "(( post_1))", "((post_1@1.2))", "((@1.0.0))", "(())", "((a b))", "(post_1)", "((!))"] {
        assert_eq!(tokenize_post_body(raw), vec![text(raw)], "{raw}");
    }
    assert_eq!(tokenize_post_body(r"\((post_1))"),vec![text("((post_1))")], "an escaped parenthesis is literal");
}

#[test]
fn embeds_inside_code_are_code_and_inside_marks_are_found() {
    assert_eq!(tokenize_post_body("`((post_1))`"), vec![BodyToken::Code("((post_1))".to_string())]);
    let marked = tokenize_post_body("**((post_1))**");
    assert_eq!(embed_targets(&marked), vec![("post_1".to_string(), None)]);
}

#[test]
fn an_embed_counts_only_its_trailing_punctuation() {
    assert_eq!(visible_text_len("((post_1@1.0.0))!"), 1);
    assert_eq!(visible_text(&tokenize_post_body("hi ((post_1))")), "hi ");
}

#[test]
fn the_markup_round_trips() {
    let pinned = embed_markup("post_1", Some(SemVer::new(2, 0, 1)));
    assert_eq!(pinned, "((post_1@2.0.1))");
    assert_eq!(tokenize_post_body(&pinned), vec![embed("post_1", Some(SemVer::new(2, 0, 1)), "")]);
    assert!(has_rich_markup(&embed_markup("x", None)));
}

#[test]
fn unpinned_embeds_are_pinned_to_the_version_shown_and_code_is_left_alone() {
    let now = |id: &str| (id == "a").then_some(SemVer::new(1, 1, 0));
    assert_eq!(pin_embeds("((a)) and ((a@1.0.0)) and ((b)), ((a))", now), "((a@1.1.0)) and ((a@1.0.0)) and ((b)), ((a@1.1.0))");
    assert_eq!(pin_embeds("((a)) `((a))`", now), "((a)) `((a))`", "a target also written as code stays as written");
    assert_eq!(pin_embeds("plain words", now), "plain words");
}
