//! "Test call" and "Test image" on the Engine step: one request through the draft's engine on
//! the background executor (like "Hear them speak"), with the reply, the image or a friendly,
//! key-redacted failure kept for the page.

use super::engine::{CallReply, Probe};
use super::ForgeView;
use crate::agent::custom_request;
use crate::agent::image::{self, ImageJob};
use crate::agent::llm::{redact_secrets, ChatRequest};
use crate::agent::runtime::{EnginePlan, Transport};
use crate::agent::DialecticEngine;
use crate::domain::engine::ImageTrigger;
use crate::ui::features::forge::text::tr;
use gpui_kit::*;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;

/// The user turn of a test call.
const CALL_PROMPT: &str = "This is a connection test. Reply with one short sentence, in character.";
/// What a test image shows (the agent's style prefix is added by the engine).
const IMAGE_PROMPT: &str = "a small lighthouse on a cliff at dusk, gentle light";
/// Scratch folder of test images (kept out of the vault's media folder).
const PROBE_DIR: &str = "simulacra-engine-probe";
const PROBE_TIMEOUT: Duration = Duration::from_secs(60);
const NOT_READY_KEY: &str = "forge.ui.engine.probe_not_ready";
const NO_IMAGE_KEY: &str = "forge.ui.engine.probe_no_image";

fn probe_dir() -> PathBuf {
    std::env::temp_dir().join(PROBE_DIR)
}

/// Runs `work` to completion on a private current-thread runtime (reqwest needs one).
fn block_on<T>(work: impl Future<Output = Result<T, String>>) -> Result<T, String> {
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build().map_err(|e| e.to_string())?;
    runtime.block_on(work)
}

/// One request through `plan`: the raw JSON of a custom request plus the text at its reply
/// path, or the native reply text.
fn call(plan: &EnginePlan, system: &str) -> Result<CallReply, String> {
    let request = ChatRequest::simple(system, CALL_PROMPT);
    let reply = block_on(async {
        match &plan.transport {
            Transport::Custom(custom) => {
                let http = reqwest::Client::builder().timeout(PROBE_TIMEOUT).build().map_err(|e| e.to_string())?;
                let vars = custom.variables(&request);
                let raw = custom_request::send(&http, &custom.request, &custom.secret, &custom.settings.model, &vars).await?;
                let text = custom.parse(&raw).map(|reply| reply.text);
                Ok(CallReply { raw: Some(serde_json::to_string_pretty(&raw).unwrap_or_default()), text })
            }
            Transport::Native(client) => client.chat(&request).await.map(|reply| CallReply { raw: None, text: Ok(reply.text) }),
        }
    });
    let secrets = plan.credentials.secrets();
    reply.map_err(|raw| plan.transport.describe_failure(&raw, &secrets).message)
}

fn draw(job: &ImageJob, secrets: &[String]) -> Result<PathBuf, String> {
    block_on(image::generate(job, IMAGE_PROMPT)).map_err(|e| redact_secrets(&e, secrets))
}

impl ForgeView {
    /// Sends one test request through the draft's text engine.
    pub fn test_engine_call(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.engine.call == Probe::Waiting {
            return;
        }
        let profile = self.draft.normalized();
        let plan = EnginePlan::resolve(&profile.config.engine, &self.context.llm, &self.context.credentials, probe_dir());
        if !plan.is_ready() {
            self.engine.call = Probe::Failed(tr(NOT_READY_KEY));
            cx.notify();
            return;
        }
        let system = DialecticEngine::system_prompt_in(&profile, &self.context.prompt);
        self.engine.call = Probe::Waiting;
        cx.notify();
        let task = cx.background_executor().spawn(async move { call(&plan, &system) });
        cx.spawn_in(window, async move |this, cx| {
            let reply = task.await;
            let _ = this.update_in(cx, |this, _, cx| {
                this.engine.call = reply.map_or_else(Probe::Failed, Probe::Done);
                cx.notify();
            });
        })
        .detach();
    }

    /// Draws one test image with the draft's image engine (whatever its trigger).
    pub fn test_engine_image(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.engine.image == Probe::Waiting {
            return;
        }
        let mut engine = self.engine().clone();
        engine.image.trigger = ImageTrigger::Always;
        let plan = EnginePlan::resolve(&engine, &self.context.llm, &self.context.credentials, probe_dir());
        let Some(job) = plan.image else {
            self.engine.image = Probe::Failed(tr(NO_IMAGE_KEY));
            cx.notify();
            return;
        };
        let secrets = plan.credentials.secrets();
        self.engine.image = Probe::Waiting;
        cx.notify();
        let task = cx.background_executor().spawn(async move { draw(&job, &secrets) });
        cx.spawn_in(window, async move |this, cx| {
            let drawn = task.await;
            let _ = this.update_in(cx, |this, _, cx| {
                this.engine.image = drawn.map_or_else(Probe::Failed, Probe::Done);
                cx.notify();
            });
        })
        .detach();
    }
}
