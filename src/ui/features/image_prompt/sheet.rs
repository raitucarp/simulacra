//! The image ideas sheet: a scrim and an elevated card with the picks on the left and the
//! prompt, the AI's ideas and the results on the right.

use super::labels::{subtitle, tr};
use super::model::{IdeasEvent, IdeasSheet};
use super::picks::render_picks;
use super::prompt_pane::render_prompt_pane;
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::TextareaState;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const BACKDROP_ID: &str = "image-ideas-backdrop";
const CARD_ID: &str = "image-ideas-card";
const CLOSE_ID: &str = "image-ideas-close";
const ESCAPE_KEY: &str = "escape";

/// Sends one sheet event to the host.
pub type IdeasAction = Rc<dyn Fn(IdeasEvent, &mut Window, &mut App) + 'static>;

/// What the sheet renders from.
pub struct IdeasInputs<'a> {
    pub sheet: &'a IdeasSheet,
    /// The editable prompt.
    pub prompt: &'a Entity<TextareaState>,
    pub focus: &'a FocusHandle,
    pub theme: &'a ThemeTokens,
    pub on_event: IdeasAction,
}

/// The overlay.
pub fn render_ideas_sheet(inputs: IdeasInputs<'_>) -> Stateful<Div> {
    let theme = inputs.theme;
    let (escape, dismiss) = (inputs.on_event.clone(), inputs.on_event.clone());
    let body = div().flex_1().min_h_0().flex().child(render_picks(inputs.sheet, &inputs.on_event, theme)).child(render_prompt_pane(&inputs));
    let card = card(theme).child(header(inputs.sheet, inputs.on_event.clone(), theme)).child(body);
    div()
        .id(BACKDROP_ID)
        .track_focus(inputs.focus)
        .on_key_down(move |event, window, cx| {
            if event.keystroke.key == ESCAPE_KEY {
                escape(IdeasEvent::Close, window, cx);
            }
        })
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.overlay)
        .flex()
        .items_center()
        .justify_center()
        .on_mouse_down(MouseButton::Left, move |_, window, cx| dismiss(IdeasEvent::Close, window, cx))
        .child(card)
}

/// The elevated card (clicks inside never reach the scrim).
fn card(theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(CARD_ID)
        .w(theme.image_ideas_sheet_width)
        .max_w_full()
        .h(theme.image_ideas_sheet_height)
        .max_h_full()
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

fn header(sheet: &IdeasSheet, on_event: IdeasAction, theme: &ThemeTokens) -> Div {
    let titles = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("title")))
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(subtitle(sheet.context.target, &sheet.about)));
    let close = Button::new(CLOSE_ID)
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::X))
        .tooltip(tr("close"))
        .on_click(move |_, window, cx| on_event(IdeasEvent::Close, window, cx));
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.card_padding)
        .py(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(Icon::new(LucideIcon::Sparkles).size(theme.icon_size_sm).text_color(theme.accent_brand))
        .child(titles)
        .child(close)
}
