mod network;
mod offline;
mod served;

use super::context::{DocumentSnapshot, PostSnapshot, ProfileSnapshot, ToolContext};
use chrono::{TimeZone, Utc};

/// Three documents: one on monism, one on attention, one on cooking.
pub(super) fn documents() -> Vec<DocumentSnapshot> {
    let doc = |id: &str, title: &str, content: &str| DocumentSnapshot { id: id.into(), title: title.into(), content: content.into() };
    vec![
        doc("doc_ethics", "Ethics", "Substance is one; thought and extension are attributes of a single substance. Monism unifies mind and body."),
        doc("doc_attention", "Gravity and Grace", "Attention is the rarest and purest form of generosity. Waiting attentively before a contradiction."),
        doc("doc_bread", "Bread", "Knead the dough, let it rise, bake at high heat until the crust sings."),
    ]
}

/// A small network: two profiles, a root post with one reply and an unrelated post.
pub(super) fn context() -> ToolContext {
    let at = |minute| Utc.with_ymd_and_hms(2026, 11, 17, 9, minute, 0).single().expect("time");
    let post = |id: &str, handle: &str, content: &str, minute, reply_to: Option<&str>| PostSnapshot {
        id: id.into(),
        author_handle: handle.into(),
        author_name: handle.trim_start_matches('@').into(),
        content: content.into(),
        created_at: at(minute),
        reply_to: reply_to.map(str::to_string),
    };
    let profile = |id: &str, handle: &str, is_agent| ProfileSnapshot {
        id: id.into(),
        name: handle.trim_start_matches('@').into(),
        handle: handle.into(),
        occupation: "Philosopher".into(),
        bio: "Thinks.".into(),
        is_agent,
    };
    ToolContext {
        posts: vec![
            post("p3", "@weil", "Unrelated musings on bread", 30, None),
            post("p2", "@spinoza", "God or Nature, one substance", 20, Some("p1")),
            post("p1", "@arya", "Is the mind separate from the body?", 10, None),
        ],
        profiles: vec![profile("u1", "@arya", false), profile("a1", "@spinoza", true)],
        documents: documents(),
        ..ToolContext::default()
    }
}
