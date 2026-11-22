use super::*;

const IDLE: Duration = Duration::from_secs(300);

fn opened(text: &str) -> Zeroizing<String> {
    Zeroizing::new(text.to_string())
}

#[test]
fn an_unlocked_post_shows_its_text_until_it_is_locked_again() {
    let start = Instant::now();
    let mut unlocked = UnlockedPosts::new(IDLE, start);
    let id = PostId::new("p1");
    assert!(unlocked.is_empty() && unlocked.text(&id).is_none());
    unlocked.insert(id.clone(), opened("a private thought"), start);
    assert_eq!(unlocked.text(&id), Some("a private thought"));
    assert_eq!(unlocked.texts().get(&id).map(|text| text.as_str()), Some("a private thought"));
    assert!(unlocked.relock(&id));
    assert!(!unlocked.is_unlocked(&id) && !unlocked.relock(&id));
}

#[test]
fn everything_locks_again_after_the_idle_time_without_activity() {
    let start = Instant::now();
    let mut unlocked = UnlockedPosts::new(IDLE, start);
    unlocked.insert(PostId::new("p1"), opened("one"), start);
    unlocked.insert(PostId::new("p2"), opened("two"), start);
    assert!(!unlocked.relock_if_idle(start + IDLE - Duration::from_secs(1)), "not idle long enough");
    unlocked.touch(start + Duration::from_secs(200));
    assert!(!unlocked.relock_if_idle(start + IDLE + Duration::from_secs(1)), "activity pushed it back");
    assert_eq!(unlocked.len(), 2);
    assert!(unlocked.relock_if_idle(start + Duration::from_secs(200) + IDLE));
    assert!(unlocked.is_empty());
    assert!(!unlocked.relock_if_idle(start + IDLE * 3), "nothing left to lock");
}

#[test]
fn an_earlier_touch_never_moves_the_activity_back() {
    let start = Instant::now();
    let mut unlocked = UnlockedPosts::new(IDLE, start + Duration::from_secs(100));
    unlocked.insert(PostId::new("p1"), opened("one"), start + Duration::from_secs(100));
    unlocked.touch(start);
    assert!(!unlocked.relock_if_idle(start + IDLE + Duration::from_secs(50)));
}

#[test]
fn unlocking_never_reaches_the_store_the_indexes_or_the_agents() {
    use crate::domain::crypto::KdfParams;
    use crate::domain::post::lock::{LockPassword, LOCKED_PLACEHOLDER};
    use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};
    let mut state = AppState::new_in_memory();
    let password = LockPassword::new("seven tidy lanterns");
    let fast = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
    let ids = state.submit_locked_thread(&[ThreadDraft::text("Zeppelin harbour diary")], &ThreadAnchor::default(), &password, fast).expect("locked");
    let mut unlocked = UnlockedPosts::default();
    let post = state.get_post(&ids[0]).expect("post");
    unlocked.insert(ids[0].clone(), post.unlock(&password).expect("opens"), Instant::now());
    assert_eq!(unlocked.text(&ids[0]), Some("Zeppelin harbour diary"));
    assert_eq!(state.get_post(&ids[0]).expect("post").content, LOCKED_PLACEHOLDER, "the store keeps the placeholder");
    let network = state.active_network_id.clone();
    let context = crate::agent::tools::ToolContext::capture(&network, &state.posts, &state.user_profiles, &state.documents);
    assert!(context.posts.iter().all(|post| !post.content.contains("Zeppelin")), "the agents' context");
    assert!(crate::state::semantic::index_sources(&state).iter().all(|source| !source.text.contains("Zeppelin")), "the meaning index");
}

#[test]
fn a_few_wrong_passwords_are_free_then_the_wait_doubles_to_a_cap() {
    use super::backoff::{wait_after, FIRST_WAIT, FREE_ATTEMPTS, MAX_WAIT};
    assert!((0..FREE_ATTEMPTS).all(|failures| wait_after(failures).is_none()));
    assert_eq!(wait_after(FREE_ATTEMPTS), Some(FIRST_WAIT));
    assert_eq!(wait_after(FREE_ATTEMPTS + 1), Some(FIRST_WAIT * 2));
    assert_eq!(wait_after(FREE_ATTEMPTS + 2), Some(FIRST_WAIT * 4));
    assert_eq!(wait_after(FREE_ATTEMPTS + 30), Some(MAX_WAIT));
    assert_eq!(wait_after(u32::MAX), Some(MAX_WAIT));
}

#[test]
fn attempts_wait_per_post_and_the_right_password_clears_it() {
    use super::backoff::{UnlockAttempts, FIRST_WAIT, FREE_ATTEMPTS};
    let (start, post, other) = (Instant::now(), PostId::new("p1"), PostId::new("p2"));
    let mut attempts = UnlockAttempts::default();
    for _ in 1..FREE_ATTEMPTS {
        assert_eq!(attempts.failed(&post, start), None);
    }
    assert_eq!(attempts.wait(&post, start), None, "still free");
    assert_eq!(attempts.failed(&post, start), Some(FIRST_WAIT));
    assert_eq!(attempts.wait(&post, start + Duration::from_secs(2)), Some(FIRST_WAIT - Duration::from_secs(2)));
    assert_eq!(attempts.wait(&post, start + FIRST_WAIT), None, "the wait is over");
    assert_eq!(attempts.wait(&other, start), None, "other posts are not held back");
    assert_eq!(attempts.failed(&post, start + FIRST_WAIT), Some(FIRST_WAIT * 2), "the next wrong one waits longer");
    attempts.succeeded(&post);
    assert_eq!(attempts.wait(&post, start + FIRST_WAIT), None);
    assert_eq!(attempts.failed(&post, start), None, "free again after the right password");
}
