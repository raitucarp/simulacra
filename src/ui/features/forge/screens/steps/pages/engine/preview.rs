//! The live preview of a structured answer: an editable sample answer (generated from the
//! schema until edited) and the post it maps to, drawn like a feed card.

use super::super::super::super::common::{card, heading, muted};
use super::kit::{engine_id, json_line, CODE_ROWS};
use crate::agent::runtime::PostDraft;
use crate::domain::engine::StructuredOutput;
use crate::state::forge::engine::{object_problem, preview, sample_answer, PreviewError, SampleTexts};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ForgeTextInput;
use crate::ui::features::post::mood_chip;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::render_avatar_mode;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const TITLE_KEY: &str = "forge.ui.engine.preview_title";
const HINT_KEY: &str = "forge.ui.engine.preview_hint";
const SAMPLE_KEY: &str = "forge.ui.engine.preview_sample";
const RESET_KEY: &str = "forge.ui.engine.preview_reset";
const NOTHING_KEY: &str = "forge.ui.engine.preview_nothing";
const IMAGE_KEY: &str = "forge.ui.engine.preview_image";
const SAMPLE_ROOT: &str = "forge.ui.engine.sample";
const HASHTAG_SAMPLES: [&str; 2] = ["hashtag_a", "hashtag_b"];
const PART_SAMPLES: [&str; 2] = ["part_a", "part_b"];

/// The localized sample values of the well-known answer fields.
fn sample_texts() -> SampleTexts {
    let text = |key: &str| tr(&format!("{SAMPLE_ROOT}.{key}"));
    SampleTexts {
        content: text("content"),
        image_prompt: text("image_prompt"),
        mood: text("mood"),
        hashtags: HASHTAG_SAMPLES.iter().map(|key| text(key)).collect(),
        thread_parts: PART_SAMPLES.iter().map(|key| text(key)).collect(),
        other: text("other"),
    }
}

impl ForgeView {
    pub(super) fn preview_card(&self, structured: &StructuredOutput, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let sample = self.engine.sample.clone().unwrap_or_else(|| sample_answer(structured, &sample_texts()));
        let edit = handle.with(|this: &mut ForgeView, text: String, _, cx| {
            this.engine.sample = Some(text);
            cx.notify();
        });
        let input = ForgeTextInput::new(engine_id("sample"), sample.clone(), theme, move |t, w, cx| edit(t, w, cx)).multiline(CODE_ROWS.0, CODE_ROWS.1);
        let reset = handle.act(|this, _, cx| {
            this.engine.sample = None;
            cx.notify();
        });
        let reset = Button::new(engine_id("sample-reset")).ghost().xsmall().icon(LucideIcon::RefreshCw).label(tr(RESET_KEY)).on_click(move |_, w, cx| reset(w, cx));
        let rendered = match preview(structured, &sample) {
            Ok(draft) => Some(self.sample_post(&draft)),
            Err(PreviewError::NothingToPost) => Some(muted(tr(NOTHING_KEY), theme).text_color(theme.warning)),
            Err(PreviewError::InvalidJson(_)) => None,
        };
        let caption = div().flex().items_center().justify_between().child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(tr(SAMPLE_KEY))).child(reset);
        card(theme)
            .child(heading(tr(TITLE_KEY), theme))
            .child(muted(tr(HINT_KEY), theme))
            .child(caption)
            .child(input)
            .child(json_line(object_problem(&sample), theme))
            .children(rendered)
    }

    /// The draft's post as a small feed card: author, body, mood, image prompt and thread.
    fn sample_post(&self, draft: &PostDraft) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let avatar = profile.config.avatar.as_ref().and_then(|mode| render_avatar_mode(mode, theme.avatar_size_md, theme));
        let author = div()
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .children(avatar)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(profile.name.clone()))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(profile.handle.clone()));
        let image = draft.image_prompt.clone().map(|prompt| image_line(prompt, theme));
        let parts = draft.thread_parts.iter().map(|part| thread_part(part.clone(), theme));
        div()
            .p(theme.card_padding)
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .rounded(theme.radius_lg)
            .bg(theme.bg_feed_post)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(author)
            .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).line_height(relative(theme.line_height_body)).child(draft.body()))
            .children(draft.mood.as_deref().map(|mood| div().flex().child(mood_chip(mood, theme))))
            .children(image)
            .children(parts)
    }
}

/// "This post gets an image of …".
fn image_line(prompt: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_start()
        .gap(theme.spacing_xs)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_elevated)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_secondary)
        .child(div().text_color(theme.accent_brand).child(Icon::new(LucideIcon::Image).size(theme.font_size_sm)))
        .child(div().flex_1().min_w_0().child(format!("{} {prompt}", tr(IMAGE_KEY))))
}

/// A chained follow-up post, joined by a threadline.
fn thread_part(text: String, theme: &ThemeTokens) -> Div {
    div()
        .ml(theme.spacing_md)
        .pl(theme.spacing_md)
        .border_l(theme.border_width_threadline)
        .border_color(theme.border)
        .text_size(theme.font_size_sm)
        .text_color(theme.text_primary)
        .line_height(relative(theme.line_height_body))
        .child(text)
}
