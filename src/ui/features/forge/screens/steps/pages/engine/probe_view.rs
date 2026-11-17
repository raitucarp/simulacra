//! The "Test call" and "Test image" cards: the button, a waiting line, then the reply (raw
//! and extracted), the picture, or what went wrong.

use super::super::super::super::common::{card, heading, muted};
use super::kit::engine_id;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{CallReply, ForgeHandle, ForgeView, Probe};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::Illustration;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use std::path::Path;

const CALL_TITLE_KEY: &str = "forge.ui.engine.call_title";
const CALL_HINT_KEY: &str = "forge.ui.engine.call_hint";
const CALL_BUTTON_KEY: &str = "forge.ui.engine.call_button";
const IMAGE_TITLE_KEY: &str = "forge.ui.engine.image_test_title";
const IMAGE_HINT_KEY: &str = "forge.ui.engine.image_test_hint";
const IMAGE_BUTTON_KEY: &str = "forge.ui.engine.image_test_button";
const WAITING_KEY: &str = "forge.ui.engine.probe_waiting";
const FAILED_KEY: &str = "forge.ui.engine.probe_failed";
const REPLY_KEY: &str = "forge.ui.engine.probe_reply";
const RAW_KEY: &str = "forge.ui.engine.probe_raw";
const UNREADABLE_KEY: &str = "forge.ui.engine.probe_unreadable";
const UNREADABLE_DETAIL_KEY: &str = "forge.ui.engine.probe_unreadable_detail";
const ARG_ERROR: &str = "error";
const RAW_ID: &str = "probe-raw";

/// A tinted message box in `color`.
fn callout(title: String, body: impl IntoElement, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .p(theme.spacing_sm)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .border_l(theme.border_width_threadline)
        .border_color(color)
        .text_size(theme.font_size_xs)
        .line_height(relative(theme.line_height_body))
        .child(div().font_weight(FontWeight::SEMIBOLD).text_color(color).child(title))
        .child(div().text_color(theme.text_primary).child(body))
}

/// A scrollable monospace block (the raw reply).
fn code_block(text: String, theme: &ThemeTokens) -> impl IntoElement {
    div()
        .id(engine_id(RAW_ID))
        .max_h(theme.prompt_preview_height)
        .overflow_y_scrollbar()
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_secondary)
        .child(text)
}

fn call_result(probe: &Probe<CallReply>, theme: &ThemeTokens) -> Vec<AnyElement> {
    match probe {
        Probe::Idle => Vec::new(),
        Probe::Waiting => vec![muted(tr(WAITING_KEY), theme).into_any_element()],
        Probe::Failed(message) => vec![callout(tr(FAILED_KEY), message.clone(), theme.accent_danger, theme).into_any_element()],
        Probe::Done(reply) => {
            let text = match &reply.text {
                Ok(text) => callout(tr(REPLY_KEY), text.clone(), theme.accent_success, theme),
                Err(error) => callout(tr(UNREADABLE_KEY), tr_with(UNREADABLE_DETAIL_KEY, &[(ARG_ERROR, error.clone())]), theme.warning, theme),
            };
            let raw = reply.raw.clone().map(|raw| {
                let caption = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(RAW_KEY));
                div().flex().flex_col().gap(theme.spacing_2xs).child(caption).child(code_block(raw, theme)).into_any_element()
            });
            std::iter::once(text.into_any_element()).chain(raw).collect()
        }
    }
}

/// A probe card: title, hint, the button (disabled while waiting) and the result.
fn probe_card(keys: (&str, &str, &str), id: &str, waiting: bool, theme: &ThemeTokens, run: impl Fn(&mut Window, &mut App) + 'static) -> Div {
    let (title, hint, label) = keys;
    let button = Button::new(engine_id(id)).primary().small().icon(LucideIcon::Zap).label(tr(label)).disabled(waiting).on_click(move |_, w, cx| run(w, cx));
    card(theme).child(heading(tr(title), theme)).child(muted(tr(hint), theme)).child(div().flex().child(button))
}

impl ForgeView {
    pub(super) fn call_probe_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let run = handle.act(ForgeView::test_engine_call);
        probe_card((CALL_TITLE_KEY, CALL_HINT_KEY, CALL_BUTTON_KEY), "call", self.engine.call == Probe::Waiting, theme, move |w, cx| run(w, cx))
            .children(call_result(&self.engine.call, theme))
    }

    pub(super) fn image_probe_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let run = handle.act(ForgeView::test_engine_image);
        let result: Option<AnyElement> = match &self.engine.image {
            Probe::Idle => None,
            Probe::Waiting => Some(muted(tr(WAITING_KEY), theme).into_any_element()),
            Probe::Failed(message) => Some(callout(tr(FAILED_KEY), message.clone(), theme.accent_danger, theme).into_any_element()),
            Probe::Done(path) => Some(drawn(path, theme).into_any_element()),
        };
        let art = Illustration::trait_card(self.engine().image.engine.kind()).width(theme.forge_option_card_width);
        probe_card((IMAGE_TITLE_KEY, IMAGE_HINT_KEY, IMAGE_BUTTON_KEY), "image", self.engine.image == Probe::Waiting, theme, move |w, cx| run(w, cx))
            .child(result.unwrap_or_else(|| div().flex().justify_center().child(art).into_any_element()))
    }
}

/// The test picture and where it was saved.
fn drawn(path: &Path, theme: &ThemeTokens) -> Div {
    let picture = img(path.to_path_buf()).size(theme.forge_probe_image_size).rounded(theme.radius_md).object_fit(ObjectFit::Cover);
    let caption = div().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(Icon::new(LucideIcon::Image).size(theme.font_size_xs)).child(path.display().to_string());
    div().flex().flex_col().items_center().gap(theme.spacing_xs).child(picture).child(caption)
}
