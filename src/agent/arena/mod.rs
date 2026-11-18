//! The Arena's requests (sb F1), effectful side: debate turns, agent judges and experiment
//! answers run through each agent's engine (the agent runtime) with the post actions
//! runner's contract: timeout, retries with backoff, cancel. Prompts and parsing are pure
//! (`domain::arena`); the shell charges every request to the shared daily AI budget first.

mod runner;

#[cfg(test)]
mod tests;

pub use runner::{run_call, run_call_blocking, EngineCall};
