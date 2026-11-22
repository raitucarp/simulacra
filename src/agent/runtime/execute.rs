//! One generation: prompt → (tools) → answer → post draft → (image).

use super::converse::{converse, Answer};
use super::draft::PostDraft;
use super::plan::EnginePlan;
use super::structured::{instruction_with_schema, parse_or_repair, EMIT_DESCRIPTION, EMIT_INSTRUCTION, JSON_INSTRUCTION};
use crate::agent::image::{self, should_fire};
use crate::agent::llm::wire::STRUCTURED_OUTPUT_NAME;
use crate::agent::llm::{redact_secrets, ChatMessage, ChatRequest, SchemaStrategy, ToolChoice, ToolSpec};
use crate::agent::tools::{ToolBox, ToolContext};
use crate::domain::image_prompt::default_post_prompt;
use crate::domain::engine::StructuredOutput;
use crate::domain::post::{PostExtras, SourceRef, Tint, ToolUse};

const EMPTY_ANSWER_ERROR: &str = "The model returned an empty post";

/// What a successful generation produced.
#[derive(Debug, Clone, Default)]
pub struct EngineResult {
    pub draft: PostDraft,
    pub tools: Vec<ToolUse>,
    pub sources: Vec<SourceRef>,
    pub image_path: Option<String>,
    /// Why a triggered image could not be made (redacted); the post is kept.
    pub image_error: Option<String>,
}

impl EngineResult {
    /// The post extras recording mood, image prompt, tools and sources.
    pub fn extras(&self) -> PostExtras {
        PostExtras {
            mood: self.draft.mood.clone(),
            image_prompt: self.draft.image_prompt.clone(),
            tools: self.tools.clone(),
            sources: self.sources.clone(),
            tint: self.draft.tint,
            ..PostExtras::default()
        }
    }
}

/// Runs the plan for one `system` / `user` prompt pair. `tool_ctx` feeds granted tools;
/// `roll` (`0..100`) decides chance-triggered images.
pub async fn run_engine(plan: &EnginePlan, system: &str, user: &str, tool_ctx: Option<ToolContext>, roll: u8) -> Result<EngineResult, String> {
    let toolbox = tool_ctx
        .filter(|_| !plan.tools.is_empty())
        .map(|ctx| ToolBox::new(&plan.tools, ctx, &plan.credentials, plan.transport.native_web_search()));
    let (request, strategy) = build_request(plan, system, user, toolbox.as_ref());
    let emit = strategy == Some(SchemaStrategy::EmitTool);
    let outcome = converse(&plan.transport, request, toolbox.as_ref(), emit, plan.max_steps).await?;
    let mut draft = match (&plan.structured, outcome.answer) {
        (Some(structured), Answer::Structured(json)) => PostDraft::from_structured(&json, &structured.mapping).unwrap_or_default(),
        (Some(structured), Answer::Text(text)) => parse_or_repair(&plan.transport, outcome.request, text, structured).await?,
        (None, Answer::Text(text)) => PostDraft::from_text(&text),
        (None, Answer::Structured(json)) => PostDraft::from_text(&json.to_string()),
    };
    if draft.content.trim().is_empty() {
        return Err(EMPTY_ANSWER_ERROR.to_string());
    }
    draft.tint = allowed_tint(plan.structured.as_ref(), draft.tint);
    draft.reactions = allowed_reactions(plan.structured.as_ref(), std::mem::take(&mut draft.reactions));
    draft.feeling = allowed_feeling(plan.structured.as_ref(), draft.feeling.take()); // M5
    let mut result = EngineResult { draft, tools: outcome.tools, sources: outcome.sources, ..EngineResult::default() };
    illustrate(plan, &mut result, roll).await;
    Ok(result)
}

/// The answer's tint, kept only when the agent may tint (sb C4: off by default, so a tint
/// mapped by hand is still dropped while the setting is off; free text never tints).
pub fn allowed_tint(structured: Option<&StructuredOutput>, tint: Option<Tint>) -> Option<Tint> {
    tint.filter(|_| structured.is_some_and(|output| output.may_tint))
}

/// The answer's reactions, kept only when the agent may react (R8: off by default; free
/// text never reacts).
pub fn allowed_reactions(structured: Option<&StructuredOutput>, reactions: Vec<String>) -> Vec<String> {
    if structured.is_some_and(|output| output.may_react) { reactions } else { Vec::new() }
}

/// The answer's feeling, kept only when the agent may post feelings (M5: off by default;
/// free text never posts one).
pub fn allowed_feeling(structured: Option<&StructuredOutput>, feeling: Option<crate::domain::post::PostKind>) -> Option<crate::domain::post::PostKind> {
    feeling.filter(|_| structured.is_some_and(|output| output.may_feel))
}

/// The first request (tools, native search, structured-output wiring) and the strategy used.
pub fn build_request(plan: &EnginePlan, system: &str, user: &str, toolbox: Option<&ToolBox>) -> (ChatRequest, Option<SchemaStrategy>) {
    let mut request = ChatRequest { web_search: toolbox.is_some_and(|t| t.native_web_search), ..ChatRequest::default() };
    request.tools = toolbox.map(ToolBox::specs).unwrap_or_default();
    let strategy = plan.structured.as_ref().map(|_| plan.transport.schema_strategy(!request.tools.is_empty()));
    let mut system = system.to_string();
    if let (Some(structured), Some(strategy)) = (&plan.structured, strategy) {
        match strategy {
            SchemaStrategy::ResponseFormat => request.schema = structured.schema_value(),
            SchemaStrategy::Instruct => system = format!("{system}

{}", instruction_with_schema(JSON_INSTRUCTION, structured)),
            SchemaStrategy::EmitTool => {
                system = format!("{system}

{EMIT_INSTRUCTION}");
                let parameters = structured.schema_value().unwrap_or_default();
                let direct = request.tools.is_empty() && !request.web_search;
                request.tool_choice = if direct { ToolChoice::Force(STRUCTURED_OUTPUT_NAME.into()) } else { ToolChoice::Required };
                request.tools.push(ToolSpec { name: STRUCTURED_OUTPUT_NAME.into(), description: EMIT_DESCRIPTION.into(), parameters });
            }
        }
    }
    request.messages = vec![ChatMessage::system(system), ChatMessage::user(user)];
    (request, strategy)
}

/// Generates the post image when the plan's trigger fires (failures are recorded, not fatal).
async fn illustrate(plan: &EnginePlan, result: &mut EngineResult, roll: u8) {
    let Some(job) = &plan.image else { return };
    if !should_fire(job.config.trigger, result.draft.image_prompt.is_some(), roll) {
        return;
    }
    let fallback = || default_post_prompt(&result.draft.content, &plan.image_subject, job.config.engine.kind());
    let (prompt, negative) = match result.draft.image_prompt.clone() {
        Some(prompt) => (prompt, None),
        None => fallback().into_parts(),
    };
    match image::generate_with(job, &prompt, negative.as_deref()).await {
        Ok(path) => result.image_path = Some(path.to_string_lossy().into_owned()),
        Err(e) => result.image_error = Some(redact_secrets(&e, &plan.credentials.secrets())),
    }
}
