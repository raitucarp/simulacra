//! Agent belief memory and its revision log (sb F1).
//!
//! Each agent keeps a small set of propositions with a credence from 0 to 1, where each came
//! from (a post, a reply, a thread read, a debate, an experiment) and when. After an agent
//! posts, replies, debates or reads a thread, a cheap extraction step proposes beliefs to add
//! or revise; every change is logged with the old and new credence, the reason and the
//! source. The most salient beliefs enter the agent's prompt as a short section.
//!
//! - [`model`]: beliefs, credences, sources and revisions.
//! - [`book`]: the loaded collection and its queries; changes apply by value.
//! - [`matching`]: whether two propositions state the same belief.
//! - [`salience`], [`dossier`]: ranking, and the bounded prompt section.
//! - [`extract`], [`update`]: the extraction prompt and reader, and the pure merge.
//! - [`user`]: pin, edit and forget from the profile.
//! - [`quota`]: the per-agent switch and daily cap.
//! - [`history`]: each belief's credence over time.
//!
//! Only network material reaches these prompts: never the private journal, mood, focus or
//! capsule records (see the privacy test).

pub mod book;
pub mod dossier;
pub mod extract;
pub mod history;
pub mod matching;
pub mod model;
pub mod quota;
pub mod salience;
pub mod update;
pub mod user;

#[cfg(test)]
mod tests;

pub use book::{BeliefBook, BeliefChange};
pub use dossier::beliefs_section;
pub use model::{Belief, BeliefId, BeliefOrigin, BeliefSource, Credence, Revision, RevisionCause};
pub use quota::BeliefSettings;
pub use update::{apply_proposals, Proposal};
pub use user::{apply_user_action, BeliefAction};
