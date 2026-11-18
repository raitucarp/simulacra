//! The actions sheet: a scrim and an elevated card with the action rail on the left and the
//! chosen action (its choices, the answer and the answer's buttons) on the right.

use super::answer::render_answer_pane;
use super::model::{ActionSheet, SheetEvent};
use super::rail::render_rail;
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::InputState;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "post-actions-backdrop";
const CARD_ID: &str = "post-actions-card";
const CLOSE_ID: &str = "post-actions-close";
const ESCAPE_KEY: &str = "escape";

/// Sends one sheet event to the host.
pub type SheetAction = Rc<dyn Fn(SheetEvent, &mut Window, &mut App) + 'static>;

/// What the sheet renders from.
pub struct SheetInputs<'a> {
    pub sheet: &'a ActionSheet,
    /// The typed audience, language or concept.
    pub input: &'a Entity<InputState>,
    pub focus: &'a FocusHandle,
    pub theme: &'a ThemeTokens,
    pub on_event: SheetAction,
}

/// The overlay.
pub fn render_actions_sheet(inputs: SheetInputs<'_>) -> Stateful<Div> {
    let theme = inputs.theme;
    let (escape, dismiss) = (inputs.on_event.clone(), inputs.on_event.clone());
    let body = div()
        .flex_1()
        .min_h_0()
        .flex()
        .child(render_rail(inputs.sheet.active_kind(), &inputs.on_event, theme))
        .child(render_answer_pane(&inputs));
    let card = render_card(theme).child(render_header(&inputs.sheet.source, inputs.on_event.clone(), theme)).child(body);
    div()
        .id(BACKDROP_ID)
        .track_focus(inputs.focus)
        .on_key_down(move |event, window, cx| {
            if event.keystroke.key == ESCAPE_KEY {
                escape(SheetEvent::Close, window, cx);
            }
        })
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.overlay)
        .flex()
        .flex_col()
        .items_center()
        .pt(theme.palette_top_offset)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| dismiss(SheetEvent::Close, window, cx))
        .child(card)
}

/// The elevated card (clicks inside never reach the scrim).
fn render_card(theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(CARD_ID)
        .w(theme.post_actions_sheet_width)
        .max_w_full()
        .h(theme.post_actions_sheet_max_height)
        .flex()
        .flex_col()
        .overflow_hidden()
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .elevation_shadow(theme)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

fn render_header(source: &str, on_event: SheetAction, theme: &ThemeTokens) -> Div {
    let titles = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("post_actions.title").to_string()))
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("post_actions.subtitle", source = source).to_string()));
    let close = Button::new(CLOSE_ID)
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::X))
        .tooltip(t!("post_actions.close").to_string())
        .on_click(move |_, window, cx| on_event(SheetEvent::Close, window, cx));
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.card_padding)
        .py(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(Icon::new(LucideIcon::WandSparkles).size(theme.icon_size_sm).text_color(theme.accent_brand))
        .child(titles)
        .child(close)
}
