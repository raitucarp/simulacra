//! WS7 live model suite: real Gemini API calls through the app's own clients, runtime and
//! parsers. Every test is `#[ignore]`d, so `cargo test` never sends a request; run the suite
//! with `cargo live` (see `tests/live/README.md`). Keys come from the environment only, are
//! never printed, and every message is redacted before it is shown or written to the report.

mod live;
