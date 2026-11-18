//! Image ideas (sb D3), effectful side: "Write it for me" goes through the post actions runner
//! (timeout, retries, cancel) after the shared daily AI budget allows it; "Generate" draws
//! with the image engine on the background executor. Results land only if still current.

use super::super::view::AppView;
use crate::agent::image::generate_with;
use crate::agent::post_actions::{run_action_blocking, ActionFailure, ActionJob, CancelToken};
use crate::agent::runtime::resolve_image_job;
use crate::agent::LlmClient;
use crate::domain::engine::ImageEngineConfig;
use crate::domain::image_prompt::{candidates_prompt, parse_candidates, CANDIDATE_ANSWER_TOKENS};
use crate::domain::post_actions::{ActionPrompt, RequestPolicy};
use crate::domain::spend::SpendEstimate;
use crate::ui::features::image_prompt::{from_action, from_image_error, CandidatesStatus, GenerationStatus, IdeasFailure};
use gpui_kit::*;

impl AppView {
    /// Charges one request of `prompt` through the one budget path, or says why it cannot run.
    fn charge_ai_budget(&mut self, prompt: &ActionPrompt) -> Result<(), IdeasFailure> {
        self.charge_ai_request(prompt, CANDIDATE_ANSWER_TOKENS).map_err(|failure| from_action(failure).unwrap_or(IdeasFailure::Budget))
    }

    /// "Write it for me": three prompts from the configured language model.
    pub(in crate::ui::features::shell) fn write_image_ideas(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.image_ideas.sheet.as_ref() else { return };
        let prompt = candidates_prompt(&sheet.choices, &sheet.context);
        let policy = RequestPolicy::from_prefs(&self.state.preferences().ai);
        let mut settings = self.state.llm_settings.clone();
        settings.max_tokens = settings.max_tokens.max(CANDIDATE_ANSWER_TOKENS);
        let client = LlmClient::new(settings).with_timeout(policy.timeout);
        if client.has_api_key() && self.confirm_spend(SpendEstimate::one(), Box::new(|this, window, cx| this.write_image_ideas(window, cx)), cx) {
            return;
        }
        let ready = if client.has_api_key() { self.charge_ai_budget(&prompt) } else { Err(IdeasFailure::NoTextKey) };
        if let Err(failure) = ready {
            return self.set_candidates(CandidatesStatus::Failed(failure), cx);
        }
        self.image_ideas.cancel_running();
        let cancel = CancelToken::new();
        self.image_ideas.cancel = Some(cancel.clone());
        let (job, ticket) = (ActionJob { client, prompt, policy, cancel }, self.image_ideas.writing);
        let task = cx.background_executor().spawn(async move { run_action_blocking(&job) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, _, cx| this.finish_image_ideas(ticket, outcome, cx));
        })
        .detach();
        self.set_candidates(CandidatesStatus::Running, cx);
    }

    fn finish_image_ideas(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        if ticket != self.image_ideas.writing {
            return;
        }
        self.image_ideas.cancel = None;
        let status = match outcome.map(|text| parse_candidates(&text)) {
            Ok(ideas) if ideas.is_empty() => CandidatesStatus::Failed(IdeasFailure::Failed(String::new())),
            Ok(ideas) => CandidatesStatus::Ready(ideas),
            Err(failure) => from_action(failure).map_or(CandidatesStatus::Idle, CandidatesStatus::Failed),
        };
        self.set_candidates(status, cx);
    }

    fn set_candidates(&mut self, status: CandidatesStatus, cx: &mut Context<Self>) {
        if let Some(sheet) = self.image_ideas.sheet.as_mut() {
            sheet.candidates = status;
        }
        cx.notify();
    }

    /// "Generate": draws the edited prompt with the sheet's engine.
    pub(in crate::ui::features::shell) fn generate_image_idea(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.image_ideas.sheet.as_ref().filter(|sheet| sheet.engine_problem.is_none()) else { return };
        let config = ImageEngineConfig { style_prefix: String::new(), size: sheet.engine.size.clone(), ..sheet.engine_config.clone() };
        let job = resolve_image_job(&config, &self.state.credentials, self.state.media_dir());
        let (prompt, negative) = (self.image_ideas.prompt.read(cx).value().trim().to_string(), sheet.negative.clone());
        self.image_ideas.drawing += 1;
        let ticket = self.image_ideas.drawing;
        let task = cx.background_executor().spawn(async move {
            let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
            runtime.block_on(generate_with(&job, &prompt, negative.as_deref()))
        });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, _, cx| this.finish_image_drawing(ticket, outcome.map(|path| path.to_string_lossy().into_owned()), cx));
        })
        .detach();
        self.set_drawing(GenerationStatus::Running, cx);
    }

    fn finish_image_drawing(&mut self, ticket: u64, outcome: Result<String, String>, cx: &mut Context<Self>) {
        if ticket != self.image_ideas.drawing {
            return;
        }
        let secrets = self.state.credentials.secrets();
        let status = match outcome {
            Ok(path) => {
                if let Some(sheet) = self.image_ideas.sheet.as_mut() {
                    sheet.results.insert(0, path);
                }
                GenerationStatus::Idle
            }
            Err(error) => GenerationStatus::Failed(from_image_error(crate::agent::llm::redact_secrets(&error, &secrets))),
        };
        self.set_drawing(status, cx);
    }

    fn set_drawing(&mut self, status: GenerationStatus, cx: &mut Context<Self>) {
        if let Some(sheet) = self.image_ideas.sheet.as_mut() {
            sheet.generation = status;
        }
        cx.notify();
    }

    /// Stops writing and stops waiting for the drawing (a late picture is dropped).
    pub(in crate::ui::features::shell) fn cancel_image_ideas(&mut self, cx: &mut Context<Self>) {
        self.image_ideas.cancel_running();
        if let Some(sheet) = self.image_ideas.sheet.as_mut() {
            sheet.candidates = CandidatesStatus::Idle;
            sheet.generation = GenerationStatus::Idle;
        }
        cx.notify();
    }
}
