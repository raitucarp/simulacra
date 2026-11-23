//! N6: forking a thread into what-if branches, the lines at a fork point, and never forking or
//! labelling from a locked post.

use super::locked::{locked_post, SECRET};
use crate::domain::post::branch::BranchPoint;
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::error::AppError;
use crate::state::store::{AppState, BranchLine};

fn reply(state: &mut AppState, to: &crate::domain::PostId, text: &str) -> crate::domain::PostId {
    state.submit_post_with_attachments(text, None, None, Some(to.clone()), None).expect("reply")
}

#[test]
fn a_fork_is_a_reply_carrying_its_branch_pointer() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("We should cache the feed.").expect("root");
    assert!(state.branch_lines(&root).is_empty(), "no branch, no lines");
    let fork = state.fork_branch(&root, "  What if   we don't cache? ", "Then every scroll hits the disk.").expect("fork");
    let post = state.get_post(&fork).expect("fork post");
    assert_eq!((post.reply_to_id.as_ref(), post.extras.branch.clone()), (Some(&root), BranchPoint::named("What if we don't cache?")));
    assert_eq!(state.branch_lines(&root), vec![BranchLine { label: Some("What if we don't cache?".into()), posts: vec![fork.clone()] }]);
    assert!(matches!(state.fork_branch(&root, "   ", "No label."), Err(AppError::EmptyContent)));
}

#[test]
fn lines_follow_each_branch_down_its_thread_and_leave_sub_branches_out() {
    let mut state = AppState::new_in_memory();
    let root = state.submit_post("Pick a database.").expect("root");
    let plain = reply(&mut state, &root, "Postgres, as usual.");
    let plain_next = reply(&mut state, &plain, "With a pool.");
    let fork = state.fork_branch(&root, "What if SQLite?", "One file, no server.").expect("fork");
    let deeper = reply(&mut state, &fork, "Backups are a copy.");
    let sub = state.fork_branch(&deeper, "What if it grows?", "Then shard by user.").expect("sub");
    let lines = state.branch_lines(&root);
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], BranchLine { label: None, posts: vec![plain, plain_next] });
    assert_eq!(lines[1], BranchLine { label: Some("What if SQLite?".into()), posts: vec![fork.clone(), deeper.clone()] });
    assert_eq!(state.branch_lines(&deeper)[0].posts, vec![sub.clone()], "the sub-branch forks at its own post");
    let sub_post = state.get_post(&sub).expect("sub");
    assert_eq!(state.branch_of(sub_post).map(|(head, label)| (head.id.clone(), label)), Some((sub, "What if it grows?".into())));
    let deeper_post = state.get_post(&deeper).expect("deeper");
    assert_eq!(state.branch_of(deeper_post).map(|(head, _)| head.id.clone()), Some(fork));
}

#[test]
fn a_locked_post_is_never_forked_and_a_locked_branch_shows_only_the_placeholder() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    let before = state.posts.len();
    assert_eq!(state.fork_branch(&locked, "What if?", "Reading a secret."), Err(AppError::PostLocked));
    assert_eq!(state.posts.len(), before, "nothing was published");
    let root = state.submit_post("Plan the trip.").expect("root");
    let mut sealed = state.get_post(&locked).expect("locked").clone();
    (sealed.id, sealed.reply_to_id, sealed.extras.branch) = ("post_sealed_branch".into(), Some(root.clone()), BranchPoint::named(SECRET));
    state.insert_reply_after_parent(sealed).expect("stored");
    let lines = state.branch_lines(&root);
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].label.as_deref(), Some(LOCKED_PLACEHOLDER), "the label never shows past the lock");
}
