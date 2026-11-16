//! Sub-tab "Jungian & MBTI": grouped 4x4 type matrix, identity variant and cognitive stack.

use crate::domain::{IdentityVariant, MbtiType};
use crate::theme::ThemeTokens;
use crate::ui::components::{card_stack, section_card, EnumChips};
use crate::ui::features::agent_editor::steps::StepCtx;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const TITLE: &str = "16-TYPE JUNGIAN & MBTI MATRIX";
const SUBTITLE: &str =
    "Select an archetype grouped by temperament. Automatically updates the 4-function cognitive stack.";
const MBTI_CARD_ID: &str = "mbti-card";
const IDENTITY_VARIANT_ID: &str = "id-var";

/// Temperament groups of the 16 types, in display order.
const GROUPS: [(&str, [MbtiType; 4]); 4] = [
    ("ANALYSTS (NT)", [MbtiType::Intj, MbtiType::Intp, MbtiType::Entj, MbtiType::Entp]),
    ("DIPLOMATS (NF)", [MbtiType::Infj, MbtiType::Infp, MbtiType::Enfj, MbtiType::Enfp]),
    ("SENTINELS (SJ)", [MbtiType::Istj, MbtiType::Isfj, MbtiType::Estj, MbtiType::Esfj]),
    ("EXPLORERS (SP)", [MbtiType::Istp, MbtiType::Isfp, MbtiType::Estp, MbtiType::Esfp]),
];

pub(super) fn render(ctx: &StepCtx<'_>) -> Div {
    card_stack(ctx.theme).child(
        section_card(TITLE, SUBTITLE, ctx.theme)
            .children(GROUPS.iter().map(|(title, types)| type_group(title, types, ctx)))
            .child(variant_row(ctx)),
    )
}

fn type_group(title: &str, types: &[MbtiType], ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .child(title.to_string()),
        )
        .child(
            div()
                .w_full()
                .flex()
                .gap(theme.spacing_xs)
                .children(types.iter().map(|&mbti| type_card(mbti, ctx))),
        )
}

fn type_card(mbti: MbtiType, ctx: &StepCtx<'_>) -> Stateful<Div> {
    let theme = ctx.theme;
    let selected = ctx.profile.config.mbti == mbti;
    let hover_bg = theme.hover_bg;
    let on_pick = ctx.edit(|p, m: MbtiType| p.config.mbti = m);
    div()
        .id(format!("{}-{}", MBTI_CARD_ID, mbti.code()))
        .flex_1()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border(theme.border_width_hairline)
        .when(selected, |s| s.bg(theme.accent_brand_subtle).border_color(theme.accent_brand))
        .when(!selected, |s| {
            s.bg(theme.bg_app).border_color(theme.border_subtle).hover(move |h| h.bg(hover_bg))
        })
        .on_click(move |_, w, cx| on_pick(mbti, w, cx))
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_sm)
                .font_weight(FontWeight::BOLD)
                .text_color(if selected { theme.accent_brand } else { theme.text_primary })
                .child(mbti.code()),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .truncate()
                .child(mbti.archetype_name()),
        )
}

/// Identity variant chips (left) and the active cognitive-function stack (right).
fn variant_row(ctx: &StepCtx<'_>) -> Div {
    let theme = ctx.theme;
    let chips = EnumChips::new(
        &IdentityVariant::ALL,
        ctx.profile.config.identity_variant,
        IDENTITY_VARIANT_ID,
        IdentityVariant::label,
    )
    .key(|v| v.suffix().to_string())
    .chips(theme, ctx.edit(|p, v: IdentityVariant| p.config.identity_variant = v));
    div()
        .w_full()
        .pt(theme.spacing_xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_md)
        .child(div().flex().items_center().gap(theme.spacing_xs).children(chips))
        .child(cognitive_stack(ctx.profile.config.mbti.cognitive_functions(), theme))
}

fn cognitive_stack(functions: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_md)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(Icon::new(LucideIcon::Brain).size(theme.font_size_xs).text_color(theme.accent_brand))
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_primary)
                .child(functions.into()),
        )
}
