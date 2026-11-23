//! Runs the user started from a profile (A5): queued, writing, done, and cancel.

use super::manual::{CancelOutcome, ManualRuns, RunStatus};
use crate::domain::post::{PostSeed, TriggerMode};
use crate::domain::{PostId, ProfileId};

fn agent() -> ProfileId {
    ProfileId::new("agent")
}

fn new_post() -> TriggerMode {
    TriggerMode::NewPost { seed: PostSeed::topic("tides") }
}

#[test]
fn a_free_agent_writes_at_once_and_its_post_is_kept() {
    let mut runs = ManualRuns::default();
    assert_eq!(runs.request(&agent(), new_post(), false), Some(RunStatus::Writing));
    assert_eq!(runs.request(&agent(), TriggerMode::Reply, false), None, "one run at a time");
    assert!(runs.finished(&agent(), Some(PostId::new("p")), false));
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Done { post: Some(PostId::new("p")) }));
    runs.dismiss(&agent());
    assert_eq!(runs.status(&agent()), None);
}

#[test]
fn a_busy_agent_queues_the_run_until_its_generation_ends() {
    let mut runs = ManualRuns::default();
    assert_eq!(runs.request(&agent(), new_post(), true), Some(RunStatus::Queued));
    assert!(runs.finished(&agent(), Some(PostId::new("autonomous")), false), "the running generation was not the user's");
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Queued));
    assert_eq!(runs.take_queued(&agent()), Some(new_post()));
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Writing));
    assert_eq!(runs.take_queued(&agent()), None);
    assert!(runs.finished(&agent(), None, true));
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Failed));
}

#[test]
fn cancelling_a_queued_run_sends_nothing() {
    let mut runs = ManualRuns::default();
    runs.request(&agent(), new_post(), true);
    assert_eq!(runs.cancel(&agent()), CancelOutcome::Dequeued);
    assert_eq!(runs.take_queued(&agent()), None);
    assert!(runs.finished(&agent(), Some(PostId::new("autonomous")), false), "the other generation still lands");
    assert_eq!(runs.cancel(&agent()), CancelOutcome::Nothing);
}

#[test]
fn cancelling_while_writing_drops_the_reply_even_after_a_new_request() {
    let mut runs = ManualRuns::default();
    runs.request(&agent(), new_post(), false);
    assert_eq!(runs.cancel(&agent()), CancelOutcome::WillDiscard);
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Cancelled));
    runs.dismiss(&agent());
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Cancelled), "kept until the owed reply is dropped");
    assert_eq!(runs.request(&agent(), TriggerMode::Reply, true), Some(RunStatus::Queued));
    assert!(!runs.finished(&agent(), Some(PostId::new("late")), false), "the cancelled reply is dropped");
    assert_eq!(runs.take_queued(&agent()), Some(TriggerMode::Reply));
    assert!(runs.finished(&agent(), Some(PostId::new("fresh")), false));
    assert_eq!(runs.status(&agent()), Some(&RunStatus::Done { post: Some(PostId::new("fresh")) }));
}
