//! The default image prompt of an autonomous agent's post that came without one: the post's
//! gist, seen the agent's way (its body, mood and palette), assembled like the helper does.
//! The engine adds the agent's house style itself, so it is left out here.

use super::assemble::{assemble_prompt, ImageContext, ImagePrompt};
use super::choices::{ImageChoices, ImageTarget};
use super::engine::PromptEngine;
use super::subject::{subject_from_post, ImageSubject};
use crate::domain::engine::ImageEngineKind;

const SEEN_BY_LEAD: &str = "as imagined by";

/// The prompt for a post whose text is `post_text`, by an agent described by `agent`.
pub fn default_post_prompt(post_text: &str, agent: &ImageSubject, kind: ImageEngineKind) -> ImagePrompt {
    let gist = subject_from_post(post_text);
    let seen_by = (!agent.is_empty()).then(|| format!("{SEEN_BY_LEAD} {}", agent.noun));
    let details = seen_by.into_iter().chain(agent.details.iter().cloned()).collect();
    let subject = ImageSubject { details, palette_hint: agent.palette_hint.clone(), ..gist };
    let context = ImageContext { target: ImageTarget::Post, subject, theme_colours: Vec::new() };
    let engine = PromptEngine { kind, style_prefix: String::new(), size: String::new() };
    assemble_prompt(&ImageChoices::for_target(ImageTarget::Post), &context, &engine)
}
