//! User/context turn prompt for replying to or synthesizing a network post.

use crate::domain::agent_config::AgentConfig;
use crate::domain::post::MAX_POST_CHARS;
use crate::domain::profile::Profile;

/// Qualifies the reply of a human ("your authentic human response"); other beings drop it.
pub(super) const HUMAN_RESPONSE: &str = "human ";

impl AgentConfig {
    /// Assembles the user/context turn prompt for replying to or synthesizing a post in a network.
    pub fn assemble_user_prompt(
        &self,
        profile: &Profile,
        parent_author_name: &str,
        parent_author_handle: &str,
        parent_post_id: &str,
        parent_post_content: &str,
        is_mentioned: bool,
    ) -> String {
        let mention_context = if is_mentioned {
            self.mention_alert(profile, parent_author_name, parent_author_handle)
        } else {
            self.stream_synthesis_context()
        };

        format!(
            "{}\n\n\
             Thought in network by {} ({}):\n\
             \"{}\"\n\n\
             Compose your authentic {}response as {} ({}) strictly under {} characters in {}, including reference [[{}]].",
            mention_context,
            parent_author_name,
            parent_author_handle,
            parent_post_content,
            if self.embodiment.is_human() { HUMAN_RESPONSE } else { "" },
            profile.name,
            profile.display_handle(),
            MAX_POST_CHARS,
            self.primary_language,
            parent_post_id,
        )
    }

    fn mention_alert(&self, profile: &Profile, author_name: &str, author_handle: &str) -> String {
        format!(
            "DIRECT @MENTION ALERT: {} ({}) directly mentioned you ({}) in this post!\n\
             - Your Attitude When Mentioned: {}\n\
             - Your Conflict/Disagreement Handling: {}\n\
             - Specific Mention Protocol: {}",
            author_name,
            author_handle,
            profile.display_handle(),
            self.mention_attitude.label(),
            self.conflict_handling.label(),
            self.custom_mention_instruction,
        )
    }

    pub(super) fn stream_synthesis_context(&self) -> String {
        format!(
            "AUTONOMOUS NETWORK STREAM SYNTHESIS:\n\
             - Your Posting Cadence: {}\n\
             - Your Chronotype & Mood: {} | Mood: {}\n\
             - Draw naturally from your hobbies ({}) and epistemic stance ({}).",
            self.posting_cadence.label(),
            self.chronotype.label(),
            self.current_mood,
            self.hobbies,
            self.epistemic_stance.label(),
        )
    }
}
