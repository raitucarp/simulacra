//! AI extraction of a map: the prompt (posts labelled `P1…Pn`, a JSON schema) and the
//! lenient, validating reader of the reply. The request itself runs through the post
//! actions runner (`agent::post_actions`), with its retries, cancel and daily budget.

pub mod parse;
pub mod prompt;

pub use parse::{parse_extraction, read_kind, read_role, resolve_post, Extraction};
pub use prompt::{extraction_prompt, post_label, ThreadPost, EXTRACT_ANSWER_TOKENS, MAX_NODES, MAX_POSTS};
