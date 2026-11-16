//! Sub-tab "Live Preview": the exact system prompt and a sample @mention user turn.

use crate::theme::ThemeTokens;
use crate::ui::components::{card_stack, field_label, section_card};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const TITLE: &str = "LIVE COMPILED PROMPT INSPECTOR";
const SUBTITLE: &str = "Real-time preview of the exact System Prompt and @Mention User Turn assembled from all human parameters.";
const SYSTEM_LABEL: &str = "1. Assembled System Prompt (Sent to LLM)";
const SYSTEM_PREVIEW_ID: &str = "compiled-system-prompt-preview-scroll";
const USER_LABEL: &str = "2. Assembled @Mention User Turn Preview";
const USER_PREVIEW_ID: &str = "compiled-user-turn-preview-scroll";

/// Fixed sample @mention used to render the user-turn preview.
const SAMPLE_AUTHOR_NAME: &str = "Arya Wiratama";
const SAMPLE_AUTHOR_HANDLE: &str = "@architect";
const SAMPLE_POST_ID: &str = "post_01";
const SAMPLE_POST_TEXT: &str =
    "How does our linguistic framing shape what we consider self-evident in this network?";
const SAMPLE_IS_MENTION: bool = true;

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, profile) = (ctx.theme, ctx.profile);
    let system_prompt =
        profile.config.assemble_system_prompt(profile, ctx.network_name, ctx.grounding_doc_titles);
    let user_turn = profile.config.assemble_user_prompt(
        profile,
        SAMPLE_AUTHOR_NAME,
        SAMPLE_AUTHOR_HANDLE,
        SAMPLE_POST_ID,
        SAMPLE_POST_TEXT,
        SAMPLE_IS_MENTION,
    );
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(preview_block(SYSTEM_LABEL, SYSTEM_PREVIEW_ID, system_prompt, theme.border_strong, theme))
            .child(preview_block(USER_LABEL, USER_PREVIEW_ID, user_turn, theme.border_subtle, theme)),
    )
}

/// Labeled, height-capped, scrollable monospace preview of generated text.
fn preview_block(
    label: &str,
    id: &'static str,
    text: String,
    border: Rgba,
    theme: &ThemeTokens,
) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).child(field_label(label, theme)).child(
        div()
            .id(id)
            .w_full()
            .max_h(theme.prompt_preview_height)
            .overflow_y_scrollbar()
            .p(theme.spacing_md)
            .rounded(theme.radius_md)
            .bg(theme.bg_app)
            .border(theme.border_width_hairline)
            .border_color(border)
            .font_family(theme.font_family_mono.clone())
            .text_size(theme.font_size_xs)
            .text_color(theme.text_secondary)
            .child(text),
    )
}
