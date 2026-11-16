//! "Hear them speak" on the Voice step: asks the configured LLM for one sample post written
//! with the draft's own compiled system prompt, on the background executor (the same way
//! "describe them in words" runs), and keeps the reply or a friendly failure.

use super::ForgeView;
use crate::agent::{DialecticEngine, LlmClient};
use crate::ui::features::forge::text::tr;
use gpui_kit::*;

/// The user turn of the sample request.
const HEAR_PROMPT: &str = "Write one short, original post for the network, fully in your own voice and writing style. Reply with the post text only, without quotes or commentary.";
/// Appended when the draft lists favorite topics.
const TOPICS_LEAD: &str = " If it fits, touch on one of your favorite topics: ";
const EMPTY_REPLY_KEY: &str = "forge.ui.voice.hear_empty";

/// The user prompt asking for one sample post, mentioning `preferred_topics` when set.
pub fn hear_prompt(preferred_topics: &str) -> String {
    let topics = preferred_topics.trim();
    if topics.is_empty() {
        HEAR_PROMPT.to_string()
    } else {
        format!("{HEAR_PROMPT}{TOPICS_LEAD}{topics}")
    }
}

/// Where "Hear them speak" stands.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum HearStatus {
    #[default]
    Idle,
    Waiting,
    /// The sample post the AI wrote.
    Spoke(String),
    /// No API key configured.
    NoKey,
    /// The request failed; the message is display-safe.
    Failed(String),
}

/// Runs the request on a private single-thread tokio runtime (the HTTP client needs one).
fn request(client: &LlmClient, system: &str, user: &str) -> Result<String, String> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
    runtime.block_on(client.generate(system, user)).map_err(|raw| client.describe_failure(&raw).message)
}

impl ForgeView {
    /// Asks the AI for one sample post in the draft's voice (or explains why it cannot).
    pub fn hear_them_speak(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.hear == HearStatus::Waiting {
            return;
        }
        let client = LlmClient::new(self.context.llm.clone());
        if !client.has_api_key() {
            self.hear = HearStatus::NoKey;
            cx.notify();
            return;
        }
        self.hear = HearStatus::Waiting;
        self.guide.thinking = true;
        cx.notify();
        let profile = self.draft.normalized();
        let system = DialecticEngine::system_prompt_in(&profile, &self.context.prompt);
        let user = hear_prompt(&profile.config.preferred_topics);
        let task = cx.background_executor().spawn(async move { request(&client, &system, &user) });
        cx.spawn_in(window, async move |this, cx| {
            let reply = task.await;
            let _ = this.update_in(cx, |this, _, cx| this.finish_hear(reply, cx));
        })
        .detach();
    }

    fn finish_hear(&mut self, reply: Result<String, String>, cx: &mut Context<Self>) {
        self.guide.thinking = false;
        self.hear = match reply.map(|text| text.trim().to_string()) {
            Ok(text) if text.is_empty() => HearStatus::Failed(tr(EMPTY_REPLY_KEY)),
            Ok(text) => HearStatus::Spoke(text),
            Err(message) => HearStatus::Failed(message),
        };
        cx.notify();
    }
}

#[cfg(test)]
mod tests {
    use super::{hear_prompt, HEAR_PROMPT};

    #[test]
    fn the_sample_request_mentions_favorite_topics_only_when_set() {
        assert_eq!(hear_prompt("  "), HEAR_PROMPT);
        let with_topics = hear_prompt(" #Ethics, #Time ");
        assert!(with_topics.starts_with(HEAR_PROMPT));
        assert!(with_topics.ends_with("#Ethics, #Time"));
    }
}
