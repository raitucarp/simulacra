//! Running code blocks (epic P): which runtimes exist and how each is asked its version, and
//! what the last detection found. The running itself is in [`crate::runner`].
//!
//! - [`runtime`]: the runtimes and their version probes (P1).
//! - [`detected`]: the cached result of the probes (P1).

pub mod detected;
pub mod runtime;
#[cfg(test)]
mod tests;

pub use detected::{DetectedRuntime, RuntimeCache};
pub use runtime::{parse_version, Probe, Runtime};
