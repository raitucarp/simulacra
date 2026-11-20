//! User/context turn prompt for starting a new top-level conversation (not a reply).

use super::user::HUMAN_RESPONSE;
use crate::domain::agent_config::AgentConfig;
use crate::domain::post::{ResolvedSeed, MAX_POST_CHARS};
use crate::domain::profile::Profile;

const NEW_CONVERSATION: &str = "NEW CONVERSATION: Start a fresh top-level post in the network. You are not replying to anyone—open a thought others will want to answer, grounded in who you are.";
const OPEN_SUBJECT: &str = "No topic was given: choose one from your own interests, work, hobbies and current concerns.";

impl AgentConfig {
    /// Assembles the user turn asking the agent to open a new conversation, grounded in its
    /// dossier (the system prompt) and, when given, the seed topic and vault document.
    pub fn assemble_root_prompt(&self, profile: &Profile, seed: &ResolvedSeed) -> String {
        let mut sections = vec![self.stream_synthesis_context(), NEW_CONVERSATION.to_string()];
        if !seed.topic.is_empty() {
            sections.push(format!("Topic to start from:\n\"{}\"", seed.topic));
        }
        if let Some(doc) = &seed.document {
            sections.push(format!(
                "From the network's knowledge vault, \"{}\":\n\"{}\"\nYou may cite it as [[{}]].",
                doc.title, doc.excerpt, doc.title
            ));
        }
        if seed.is_empty() {
            sections.push(OPEN_SUBJECT.to_string());
        }
        sections.push(format!(
            "Compose your authentic {}opening post as {} ({}) strictly under {} characters in {}.",
            if self.embodiment.is_human() { HUMAN_RESPONSE } else { "" },
            profile.name,
            profile.display_handle(),
            MAX_POST_CHARS,
            self.primary_language,
        ));
        sections.join("\n\n")
    }
}
