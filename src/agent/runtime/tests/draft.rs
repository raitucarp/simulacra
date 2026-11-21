//! Agent answers keep the post marks and lose headings and other markdown (#38).

use crate::agent::runtime::PostDraft;
use crate::domain::engine::OutputMapping;
use serde_json::json;

#[test]
fn free_text_keeps_marks_and_drops_headings() {
    let draft = PostDraft::from_text("# Thesis\n\n**Being** is *becoming*.\n\n---");
    assert_eq!(draft.content, "Thesis\n\n**Being** is *becoming*.");
}

#[test]
fn structured_content_and_thread_parts_are_sanitized() {
    let mapping = OutputMapping { content: "post".into(), thread_parts: "more".into(), ..OutputMapping::default() };
    let answer = json!({ "post": "## Claim\n__underlined__ point", "more": ["> quoted `code`", "---"] });
    let draft = PostDraft::from_structured(&answer, &mapping).expect("a postable answer");
    assert_eq!(draft.content, "Claim\n__underlined__ point");
    assert_eq!(draft.thread_parts, vec!["quoted `code`".to_string()]);
}
