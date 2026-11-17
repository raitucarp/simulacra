use super::*;

fn jobs(count: usize) -> Vec<ChunkJob> {
    (0..count)
        .map(|i| ChunkJob { owner: post(&format!("p{i}")), chunk_index: 0, title: String::new(), text: format!("text {i}"), content_hash: format!("h{i}") })
        .collect()
}

#[test]
fn an_empty_plan_stays_idle_and_a_plan_starts_running() {
    let queue = IndexQueue::default().with_plan(Vec::new());
    assert_eq!(queue.status(), QueueStatus::Idle);
    assert_eq!(queue.progress(), None);
    let queue = queue.with_plan(jobs(5));
    assert_eq!(queue.status(), QueueStatus::Running);
    assert_eq!(queue.progress(), Some(Progress { done: 0, total: 5 }));
}

#[test]
fn batches_go_out_one_at_a_time_and_progress_counts_completed_chunks() {
    let (queue, batch) = IndexQueue::default().with_plan(jobs(5)).take_batch(2);
    let batch = batch.expect("a batch");
    assert_eq!(batch.jobs.len(), 2);
    let (queue, second) = queue.take_batch(2);
    assert!(second.is_none(), "one batch in flight at a time");
    assert!(queue.is_busy());
    let queue = queue.completed(batch.generation);
    assert_eq!(queue.progress(), Some(Progress { done: 2, total: 5 }));
    let (queue, batch) = queue.take_batch(10);
    let queue = queue.completed(batch.expect("the rest").generation);
    assert_eq!(queue.status(), QueueStatus::Idle, "all done");
    assert_eq!(queue.pending_len(), 0);
}

#[test]
fn a_replan_while_running_keeps_counting_and_skips_chunks_in_flight() {
    let (queue, batch) = IndexQueue::default().with_plan(jobs(3)).take_batch(1);
    let batch = batch.unwrap();
    let queue = queue.completed(batch.generation);
    let (queue, out) = queue.take_batch(1);
    let out = out.unwrap();
    // The plan is recomputed while p1 is out: it still lists p1 and p2, plus a new p3.
    let queue = queue.with_plan(jobs(4).into_iter().skip(1).collect());
    assert_eq!(queue.pending_len(), 2, "p1 is in flight, so only p2 and p3 wait");
    assert_eq!(queue.progress(), Some(Progress { done: 1, total: 4 }));
    let queue = queue.completed(out.generation);
    assert_eq!(queue.progress(), Some(Progress { done: 2, total: 4 }));
}

#[test]
fn a_failure_pauses_and_returns_the_batch_until_resumed() {
    let (queue, batch) = IndexQueue::default().with_plan(jobs(3)).take_batch(2);
    let batch = batch.unwrap();
    let queue = queue.failed(batch.generation, FailureKind::BadKey);
    assert_eq!(queue.status(), QueueStatus::Paused(FailureKind::BadKey));
    assert_eq!(queue.pending_len(), 3, "the failed chunks wait again");
    assert!(queue.clone().take_batch(2).1.is_none(), "paused queues hand out nothing");
    let queue = queue.with_plan(jobs(4));
    assert_eq!(queue.status(), QueueStatus::Paused(FailureKind::BadKey), "a new plan does not retry a bad key");
    let (queue, retry) = queue.resumed().take_batch(2);
    assert_eq!(retry.unwrap().jobs[0].owner, post("p0"), "the first chunk is retried first");
    assert_eq!(queue.status(), QueueStatus::Running);
}

#[test]
fn cancel_drops_everything_and_ignores_the_late_batch() {
    let (queue, batch) = IndexQueue::default().with_plan(jobs(3)).take_batch(2);
    let batch = batch.unwrap();
    let queue = queue.cancelled();
    assert_eq!(queue.status(), QueueStatus::Cancelled);
    assert!(!queue.is_current(batch.generation));
    let late = queue.clone().completed(batch.generation);
    assert_eq!(late, queue, "a result from before the cancel changes nothing");
    let late_failure = queue.clone().failed(batch.generation, FailureKind::Network);
    assert_eq!(late_failure.status(), QueueStatus::Cancelled);
    let queue = queue.with_plan(jobs(2));
    assert_eq!(queue.status(), QueueStatus::Cancelled, "a plan waits after a cancel");
    assert_eq!(queue.resumed().progress(), Some(Progress { done: 0, total: 2 }));
}
