//! N6–N10 on the post card: the what-if chip, and nothing past a lock.

use crate::domain::crypto::KdfParams;
use crate::domain::post::branch::BranchPoint;
use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
use crate::domain::Post;

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

fn locked(mut post: Post) -> Post {
    post.lock_with(&LockPassword::new("seven tidy lanterns"), FAST).expect("locked");
    post
}

#[test]
fn a_branch_wears_its_label_but_never_past_a_lock() {
    let theme = crate::theme::ThemeTokens::default();
    let mut post = Post::create_reply("b", "n1", "u1", "Then every scroll hits the disk.", crate::domain::PostId::new("root"), None, None).expect("valid");
    assert!(super::branch_chip::render_branch_chip(&post, &theme).is_none(), "a plain reply has no chip");
    post.extras.branch = BranchPoint::named("What if we skip the cache?");
    assert!(super::branch_chip::branch_chip_text(&post).is_some_and(|text| text.contains("What if we skip the cache?")));
    assert!(super::branch_chip::render_branch_chip(&post, &theme).is_some());
    let sealed = locked(post);
    assert_eq!(super::branch_chip::branch_chip_text(&sealed).filter(|text| !text.contains("cache")).map(|text| text.contains(LOCKED_PLACEHOLDER)), Some(true));
    for key in ["menu", "title", "subtitle", "explain", "label", "label_placeholder", "first_post", "first_post_placeholder", "send", "name_first", "write_first", "failed", "sent", "chip", "main"].map(|key| format!("posts.branch.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn a_pinned_quote_shows_its_passage_and_flag_but_never_on_a_locked_post() {
    use crate::domain::post::pinned::{PinnedQuote, QuoteDrift};
    let mut post = Post::create("q", "n1", "u1", "Worth remembering.").expect("valid");
    assert_eq!(super::pinned_card::pinned_view(&post, None), None);
    post.extras.pinned = PinnedQuote::of("doc1", "On memory", "Memory is reconstructive.", 0..25).map(Box::new);
    let view = super::pinned_card::pinned_view(&post, Some(QuoteDrift::Intact)).expect("view");
    assert_eq!((view.passage.as_str(), view.flag.is_none()), ("Memory is reconstructive.", true));
    assert!(view.source.contains("On memory"));
    assert!(super::pinned_card::pinned_view(&post, Some(QuoteDrift::PassageGone)).and_then(|view| view.flag).is_some());
    assert_eq!(super::pinned_card::pinned_view(&locked(post), Some(QuoteDrift::PassageGone)), None, "a locked post shows only its placeholder");
    let keys = ["button", "title", "explain", "find_placeholder", "passages", "comment_label", "comment_placeholder", "send", "pick_first", "write_first", "sent", "source", "drift.intact", "drift.changed", "drift.gone", "drift.missing", "repin", "repinned", "repin_failed"];
    for key in keys.map(|key| format!("posts.pinned.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn the_quote_sheet_lists_matching_passages_only() {
    use crate::ui::features::data::listed_passages;
    let passages = vec![(0..11, "Alpha beta.".to_string()), (12..24, "Gamma delta.".to_string())];
    let at = |query: &str| listed_passages(&passages, query).into_iter().map(|(at, _)| at).collect::<Vec<_>>();
    assert_eq!((at(""), at("gamma"), at("zeta")), (vec![0, 1], vec![1], vec![]));
}

#[test]
fn a_scoped_post_says_who_may_read_it() {
    use crate::domain::post::audience::Audience;
    use crate::domain::ProfileId;
    let name_of = |id: &ProfileId| if id.as_str() == "ada" { "Ada".to_string() } else { id.to_string() };
    let mut post = Post::create("s", "n1", "u1", "For a few readers.").expect("valid");
    assert_eq!(super::audience_badge::audience_text(&post, name_of), None, "everyone: no badge");
    post.extras.audience = Some(Box::new(Audience::Agents { agents: vec![ProfileId::new("ada")] }));
    assert!(super::audience_badge::audience_text(&post, name_of).is_some_and(|text| text.contains("Ada")));
    post.extras.audience = Some(Box::new(Audience::OnlyMe));
    assert!(super::audience_badge::audience_text(&post, name_of).is_some());
    for key in ["menu", "everyone", "only_me", "shared_with", "set_everyone", "set_only_me", "set_agents"].map(|key| format!("posts.audience.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}

#[test]
fn a_question_names_who_was_asked_but_never_past_a_lock() {
    use crate::domain::post::question::{Asked, Question};
    use crate::domain::ProfileId;
    let name_of = |id: &ProfileId| id.to_string();
    let mut post = Post::create("q", "n1", "u1", "Why does light bend?").expect("valid");
    assert_eq!(super::question_chip::question_text(&post, name_of), None);
    post.extras.question = Some(Box::new(Question { asked: vec![Asked { agent: ProfileId::new("ada"), fit: 900 }], accepted: None }));
    assert!(super::question_chip::question_text(&post, name_of).is_some_and(|text| text.contains("ada")));
    assert_eq!(super::question_chip::question_text(&locked(post), name_of), None, "a locked question shows only its placeholder");
    for key in ["menu", "asked", "nobody", "failed", "answers_one", "answers_other", "accept", "accepted", "chip", "chip_answered"].map(|key| format!("posts.question.{key}")) {
        assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
    }
}
