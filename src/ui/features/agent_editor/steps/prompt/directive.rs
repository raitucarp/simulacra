//! Sub-tab "Directive & Template": core directive, custom template and placeholder legend.

use crate::theme::ThemeTokens;
use crate::ui::components::{card_stack, labeled_textarea, mono_tag, section_card};
use crate::ui::features::agent_editor::steps::StepCtx;
use gpui_kit::*;

const TITLE: &str = "CORE DIRECTIVE & CUSTOM PROMPT TEMPLATE";
const SUBTITLE: &str = "Write the core personal directive and optional {{placeholder}} template.";
const DIRECTIVE_LABEL: &str = "Core Personal Directive & Intellectual Mission";
const TEMPLATE_LABEL: &str = "Custom Prompt Assembly Template";
const PLACEHOLDERS_LABEL: &str = "Available Placeholders:";
const PLACEHOLDERS: [&str; 10] = [
    "{{name}}",
    "{{handle}}",
    "{{network_context}}",
    "{{all_agent_info}}",
    "{{identity}}",
    "{{psychology}}",
    "{{intelligence}}",
    "{{linguistics}}",
    "{{schedule}}",
    "{{directive}}",
];
const SAVE_BUTTON_ID: &str = "save-prompt-sub1-btn";

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    let (theme, inputs) = (ctx.theme, ctx.inputs);
    card_stack(theme).child(
        section_card(TITLE, SUBTITLE, theme)
            .child(labeled_textarea(DIRECTIVE_LABEL, inputs.system_prompt_input, theme))
            .child(labeled_textarea(TEMPLATE_LABEL, inputs.prompt_template_input, theme))
            .child(placeholder_legend(theme))
            .child(ctx.save_footer(SAVE_BUTTON_ID)),
    )
}

fn placeholder_legend(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.accent_brand)
                .child(PLACEHOLDERS_LABEL),
        )
        .children(PLACEHOLDERS.into_iter().map(|token| mono_tag(token, theme.text_primary, theme)))
}
