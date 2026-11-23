//! Running code blocks (epic P): which runtimes exist and how each is asked its version, and
//! what the last detection found. The running itself is in [`crate::runner`].
//!
//! - [`runtime`]: the runtimes and their version probes (P1).
//! - [`detected`]: the cached result of the probes (P1).
//! - [`commands`]: the language tags each runtime takes and its fixed command line (P3).
//! - [`run`]: a run saved on its post (P6).

pub mod commands; // P3
pub mod detected;
pub mod run; // P6
pub mod runtime;
#[cfg(test)]
mod tests;

pub use commands::{runtime_for, Invocation};
pub use detected::{DetectedRuntime, RuntimeCache};
pub use runtime::{parse_version, Probe, Runtime};
pub use run::{kept_output, with_run, CodeRun, RunEnd, SAVED_OUTPUT_CHARS};
