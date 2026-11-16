//! The top of the Voice page: a post-sized live sample that restyles on every change (the
//! templated `voice_sample`, no AI), and "Hear them speak", which asks the AI for a real one.

use super::super::super::super::common::{card, heading, message, muted};
use super::super::super::stage_portrait;
use crate::state::forge::voice_sample;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView, HearStatus};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::Disableable;
use gpui_kit::*;

const LIVE_KEY: &str = "forge.ui.voice.live_title";
const LIVE_HINT_KEY: &str = "forge.ui.voice.live_hint";
const HEAR_KEY: &str = "forge.ui.voice.hear";
const HEARING_KEY: &str = "forge.ui.voice.hearing";
const HEARD_KEY: &str = "forge.ui.voice.heard";
const NO_KEY_KEY: &str = "forge.ui.voice.hear_no_key";
const FAILED_KEY: &str = "forge.ui.voice.hear_failed";
const ARG_ERROR: &str = "error";
const HEAR_ID: &str = "forge-hear-them";

impl ForgeView {
    pub(super) fn voice_sample_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let waiting = self.hear == HearStatus::Waiting;
        let hear = handle.act(|this, window, cx| this.hear_them_speak(window, cx));
        let label = tr(if waiting { HEARING_KEY } else { HEAR_KEY });
        let button = Button::new(HEAR_ID).outline().icon(LucideIcon::Volume2).label(label).disabled(waiting).on_click(move |_, w, cx| hear(w, cx));
        card(theme)
            .child(heading(tr(LIVE_KEY), theme))
            .child(self.post_bubble(voice_sample(&self.draft), theme.accent_brand))
            .child(muted(tr(LIVE_HINT_KEY), theme))
            .child(div().flex().child(button))
            .children(self.heard_sample())
    }

    /// The AI's sample post, or why there is none.
    fn heard_sample(&self) -> Option<Div> {
        let theme = &self.theme;
        match &self.hear {
            HearStatus::Idle | HearStatus::Waiting => None,
            HearStatus::Spoke(text) => Some(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_xs)
                    .child(message(tr(HEARD_KEY), theme.accent_dialectic, theme))
                    .child(self.post_bubble(text.clone(), theme.accent_dialectic)),
            ),
            HearStatus::NoKey => Some(message(tr(NO_KEY_KEY), theme.warning, theme)),
            HearStatus::Failed(error) => Some(message(tr_with(FAILED_KEY, &[(ARG_ERROR, error.clone())]), theme.accent_danger, theme)),
        }
    }

    /// `text` laid out like a post by the draft: portrait, name and handle, then the body
    /// in a callout with an `accent` threadline.
    fn post_bubble(&self, text: String, accent: Rgba) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let author = div()
            .flex()
            .items_baseline()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(self.display_name()))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(profile.display_handle()));
        let body = div()
            .p(theme.spacing_md)
            .rounded(theme.radius_lg)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(accent)
            .text_size(theme.font_size_md)
            .text_color(theme.text_primary)
            .line_height(relative(theme.line_height_body))
            .child(text);
        div()
            .flex()
            .items_start()
            .gap(theme.spacing_sm)
            .child(stage_portrait(profile, theme.avatar_size_md, theme))
            .child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_xs).child(author).child(body))
    }
}
