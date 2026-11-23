//! N11: transcribing a voice note from its post's menu. The configured engine must take audio
//! (said before anything is spent); then the spend check, the shared AI budget, and the post
//! actions policy (timeout, retries) on the background executor, the audio read there too.
//! The transcript lands on the note. A locked post is never sent.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::llm::transcribe::AudioClip;
use crate::agent::post_actions::{run_transcription_blocking, ActionFailure, CancelToken, TranscribeJob};
use crate::agent::LlmClient;
use crate::domain::post::voice::VoiceNote;
use crate::domain::post_actions::{ActionFailureKind, ActionPrompt, RequestPolicy};
use crate::domain::spend::SpendEstimate;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::tr;
use gpui_kit::*;
use rust_i18n::t;

/// What a transcription is charged as: a request of about this many tokens each way.
const TRANSCRIPT_TOKENS: u32 = 1_000;

fn clip_of(note: &VoiceNote) -> Result<AudioClip, ActionFailure> {
    let bytes = std::fs::read(&note.path).map_err(|err| ActionFailure { kind: ActionFailureKind::Rejected, detail: err.to_string() })?;
    Ok(AudioClip { bytes, file_name: note.file_name(), mime: note.mime.clone() })
}

impl AppView {
    /// Transcribes the voice note of the user's post `id`.
    pub(in crate::ui::features::shell) fn transcribe_voice(&mut self, id: PostId, cx: &mut Context<Self>) {
        let Some(note) = self.state.get_post(&id).filter(|post| !post.is_locked()).and_then(|post| post.extras.voice.as_deref().cloned()) else { return };
        if self.data.ideas.transcribing.is_some() {
            return;
        }
        let policy = RequestPolicy::from_prefs(&self.state.preferences().ai);
        let client = LlmClient::new(self.state.llm_settings.clone()).with_timeout(policy.timeout);
        if !client.has_api_key() {
            return self.notify_soon(t!("posts.voice.transcribe_failed", reason = failure_message(&ActionFailure::local(ActionFailureKind::NoKey))).to_string(), cx);
        }
        if !client.can_transcribe() {
            return self.notify_soon(tr("posts.voice.transcribe_unsupported"), cx);
        }
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.transcribe_voice(resume, cx)), cx) {
            return;
        }
        let charge = ActionPrompt { system: String::new(), user: String::new() };
        if let Err(failure) = self.charge_ai_request(&charge, TRANSCRIPT_TOKENS) {
            return self.notify_soon(t!("posts.voice.transcribe_failed", reason = failure_message(&failure)).to_string(), cx);
        }
        self.data.ideas.transcribing = Some(id.clone());
        let task = cx.background_executor().spawn(async move { clip_of(&note).and_then(|clip| run_transcription_blocking(&TranscribeJob { client, clip, policy, cancel: CancelToken::new() })) });
        cx.spawn(async move |this, cx| {
            let outcome = task.await;
            let _ = this.update(cx, |this, cx| this.finish_transcript(&id, outcome, cx));
        })
        .detach();
        cx.notify();
    }

    fn finish_transcript(&mut self, id: &PostId, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        self.data.ideas.transcribing = None;
        let text = match outcome.map(|raw| self.state.set_transcript(id, &raw)) {
            Ok(Ok(())) => tr("posts.voice.transcribed"),
            Ok(Err(err)) => publish_error_message(&err),
            Err(failure) => t!("posts.voice.transcribe_failed", reason = failure_message(&failure)).to_string(),
        };
        self.notify_soon(text, cx);
        cx.notify();
    }
}
