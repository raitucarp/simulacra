//! "Describe them in words": sends the description through the configured LLM on the
//! background executor, parses the reply and applies it as one undo step.

use super::state::{DescribeStatus, Notice};
use super::ForgeView;
use crate::agent::LlmClient;
use crate::state::forge::{describe_prompt, parse_described, DraftPatch, ParseError, PatchWarning};
use crate::ui::features::forge::text::{field_label, tr, tr_with};
use gpui_kit::*;

/// System prompt for the character-sheet request (the user prompt carries the instructions).
const DESCRIBE_SYSTEM_PROMPT: &str = "You turn short descriptions into character sheets and reply with one JSON object only.";
const APPLIED_KEY: &str = "forge.ui.start.describe_applied";
const EMPTY_KEY: &str = "forge.ui.start.describe_empty";
const NO_KEY_KEY: &str = "forge.ui.start.describe_no_key";
const FAILED_KEY: &str = "forge.ui.start.describe_failed";
const ARG_ERROR: &str = "error";
const ARG_COUNT: &str = "count";
const ARG_FIELD: &str = "field";
const ARG_VALUE: &str = "value";
const ARG_DETAIL: &str = "detail";

/// Runs the request on a private single-thread tokio runtime (the HTTP client needs one).
fn request(client: &LlmClient, prompt: &str) -> Result<String, String> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
    runtime
        .block_on(client.generate(DESCRIBE_SYSTEM_PROMPT, prompt))
        .map_err(|raw| client.describe_failure(&raw).message)
}

/// A user-facing line for one skipped part of the reply.
fn warning_text(warning: &PatchWarning) -> String {
    let value = match warning {
        PatchWarning::InvalidOption { value, .. } | PatchWarning::OutOfRange { value, .. } => value.clone(),
        PatchWarning::UnknownField(_) | PatchWarning::WrongType(_) => String::new(),
    };
    let args = [(ARG_FIELD, field_label(warning.field())), (ARG_VALUE, value)];
    tr_with(&warning.message_key(), &args)
}

fn parse_error_text(error: &ParseError) -> String {
    let detail = match error {
        ParseError::Malformed(detail) => detail.clone(),
        ParseError::NoJsonObject => String::new(),
    };
    tr_with(&error.message_key(), &[(ARG_DETAIL, detail)])
}

impl ForgeView {
    /// Keeps the description text as it is typed.
    pub fn set_describe_text(&mut self, text: String, _: &mut Window, cx: &mut Context<Self>) {
        self.start.describe_text = text;
        if self.start.describe_status == DescribeStatus::Empty {
            self.start.describe_status = DescribeStatus::Idle;
            cx.notify();
        }
    }

    /// Opens or closes the describe panel on the start screen.
    pub fn toggle_describe(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.start.describing = !self.start.describing;
        self.start.rolled = None;
        cx.notify();
    }

    /// Sends the description to the AI (or explains why it cannot).
    pub fn describe(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.start.describe_text.trim().to_string();
        let client = LlmClient::new(self.context.llm.clone());
        self.start.describe_status = if text.is_empty() {
            DescribeStatus::Empty
        } else if !client.has_api_key() {
            DescribeStatus::NoKey
        } else {
            DescribeStatus::Waiting
        };
        cx.notify();
        if self.start.describe_status != DescribeStatus::Waiting {
            return;
        }
        self.guide.thinking = true;
        let prompt = describe_prompt(&text);
        let task = cx.background_executor().spawn(async move { request(&client, &prompt) });
        cx.spawn_in(window, async move |this, cx| {
            let reply = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_describe(reply, window, cx));
        })
        .detach();
    }

    fn finish_describe(&mut self, reply: Result<String, String>, window: &mut Window, cx: &mut Context<Self>) {
        self.guide.thinking = false;
        match reply.map(|text| parse_described(&text)) {
            Ok(Ok(patch)) => self.apply_described(patch, window, cx),
            Ok(Err(error)) => self.start.describe_status = DescribeStatus::Failed(parse_error_text(&error)),
            Err(message) => self.start.describe_status = DescribeStatus::Failed(message),
        }
        cx.notify();
    }

    fn apply_described(&mut self, patch: DraftPatch, window: &mut Window, cx: &mut Context<Self>) {
        self.draft.apply_patch(&patch);
        self.class = None;
        self.ensure_face();
        self.start.describe_status = DescribeStatus::Idle;
        let headline = tr_with(APPLIED_KEY, &[(ARG_COUNT, patch.edits.len().to_string())]);
        let details = patch.warnings.iter().map(warning_text).collect();
        self.notice = Some(Notice::info(headline, details));
        self.enter_steps(window, cx);
    }

    /// The failure message to show under the describe box, if any.
    pub fn describe_message(&self) -> Option<String> {
        match &self.start.describe_status {
            DescribeStatus::Idle | DescribeStatus::Waiting => None,
            DescribeStatus::Empty => Some(tr(EMPTY_KEY)),
            DescribeStatus::NoKey => Some(tr(NO_KEY_KEY)),
            DescribeStatus::Failed(error) => Some(tr_with(FAILED_KEY, &[(ARG_ERROR, error.clone())])),
        }
    }
}
