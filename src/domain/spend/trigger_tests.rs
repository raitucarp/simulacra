use super::trigger::*;

#[test]
fn a_run_without_a_model_sends_nothing() {
    let estimate = trigger_estimate(&["system", "user"], 300, 4, false);
    assert!(!estimate.uses_model());
    assert_eq!((estimate.requests, estimate.tokens_in, estimate.tokens_out), (0, 0, 0));
}

#[test]
fn a_run_counts_the_prompt_and_the_reply_cap_per_request() {
    let system = "x".repeat(400);
    let one = trigger_estimate(&[system.as_str(), "abcd"], 300, 0, true);
    assert_eq!((one.requests, one.tokens_in, one.tokens_out), (1, 101, 300));
    let tools = trigger_estimate(&[system.as_str()], 300, 3, true);
    assert_eq!((tools.requests, tools.tokens_out), (3, 900), "each tool step may send a request");
}
