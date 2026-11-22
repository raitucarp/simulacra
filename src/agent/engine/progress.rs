//! Progress replies (D12): an agent the user handed a to-do (`@handle` on an item of their
//! post) replies to that post with how the item is going. The model is asked for it on top of
//! the usual reply prompt; without one, a short canned update in the agent's style is posted.

use super::DialecticEngine;
use crate::domain::{AuthoredPost, Profile};

impl DialecticEngine {
    /// The user turn asking `agent` for a progress update on `item` of `parent`.
    pub fn progress_user_prompt(parent: &AuthoredPost, agent: &Profile, item: &str) -> String {
        let reply = Self::persona_user_prompt(parent, agent);
        format!(
            "{reply}\n\nDELEGATED TO-DO: {author} handed you this item of the post above: \"{item}\". \
             Reply with a short progress update on it in your own voice: what you have done or found, \
             what comes next, or what you need from them. Do not claim it is finished unless it is.",
            author = parent.author.name,
        )
    }

    /// The offline progress update on `item`.
    pub(super) fn generate_progress_text(agent: &Profile, item: &str) -> String {
        super::heuristic::styled(agent, format!("On it: {item}. I will post here as it moves along."))
    }
}
