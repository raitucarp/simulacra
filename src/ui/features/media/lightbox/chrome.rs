//! The lightbox controls: the close button, the previous / next buttons and the caption
//! (position and the image's description).

use super::super::consts::{LIGHTBOX_CLOSE_ID, LIGHTBOX_NEXT_ID, LIGHTBOX_PREVIOUS_ID};
use crate::domain::media::Step;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// How much larger than the largest icon step the lightbox's control glyphs are drawn.
const CONTROL_ICON_SCALE: f32 = 1.5;

/// A round control on the scrim that runs `action` without closing the lightbox.
fn round_button(id: &'static str, icon: LucideIcon, label: String, theme: &ThemeTokens, action: impl Fn(&mut Window, &mut App) + 'static) -> Stateful<Div> {
    let hover = theme.hover_bg;
    div()
        .id(id)
        .absolute()
        .size(theme.lightbox_button_size)
        .rounded(theme.radius_full)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .hover(move |s| s.bg(hover))
        .text_color(theme.text_primary)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .tooltip(move |window, cx| Tooltip::new(label.clone()).build(window, cx))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, window, cx| action(window, cx))
        .child(Icon::new(icon).size(theme.icon_size_2xl * CONTROL_ICON_SCALE))
}

/// Top-right close button.
pub(super) fn close_button(on_close: Rc<dyn Fn(&mut Window, &mut App)>, theme: &ThemeTokens) -> Stateful<Div> {
    round_button(LIGHTBOX_CLOSE_ID, LucideIcon::X, t!("media.lightbox_close").to_string(), theme, move |window, cx| on_close(window, cx))
        .top(theme.spacing_lg)
        .right(theme.spacing_lg)
}

/// The previous (left edge) or next (right edge) button, vertically centred.
pub(super) fn step_button(step: Step, on_step: Rc<dyn Fn(Step, &mut Window, &mut App)>, theme: &ThemeTokens) -> Div {
    let (id, icon, label) = match step {
        Step::Previous => (LIGHTBOX_PREVIOUS_ID, LucideIcon::ChevronLeft, t!("media.lightbox_previous")),
        Step::Next => (LIGHTBOX_NEXT_ID, LucideIcon::ChevronRight, t!("media.lightbox_next")),
    };
    let button = round_button(id, icon, label.to_string(), theme, move |window, cx| on_step(step, window, cx));
    let rail = div().absolute().top_0().bottom_0().w(theme.lightbox_inset).flex().items_center().justify_center();
    match step {
        Step::Previous => rail.left_0(),
        Step::Next => rail.right_0(),
    }
    .child(button.relative())
}

/// "2 of 4" and the description of the shown image, at the bottom.
pub(super) fn caption(index: usize, count: usize, alt: &str, theme: &ThemeTokens) -> Div {
    let position = (count > 1).then(|| t!("media.lightbox_position", index = index + 1, count = count).to_string());
    let alt = alt.trim();
    div()
        .absolute()
        .left_0()
        .right_0()
        .bottom(theme.spacing_md)
        .flex()
        .justify_center()
        .when(position.is_some() || !alt.is_empty(), |this| {
            this.child(
                div()
                    .max_w(theme.lightbox_caption_width)
                    .px(theme.card_padding)
                    .py(theme.spacing_xs)
                    .rounded(theme.radius_lg)
                    .bg(theme.bg_elevated)
                    .border(theme.border_width_hairline)
                    .border_color(theme.border_subtle)
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(theme.spacing_3xs)
                    .children(position.map(|text| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)))
                    .when(!alt.is_empty(), |pill| pill.child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).text_center().child(alt.to_string()))),
            )
        })
}
