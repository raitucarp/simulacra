//! Quick capture: a small box at the top of the window that saves a thought, a link or a
//! pasted text to the Inbox on Enter. Escape or a click outside closes it.

use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Escape, Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "capture-backdrop";
const BOX_ID: &str = "capture-box";
const SAVE_ID: &str = "capture-save";
const INBOX_ID: &str = "capture-inbox";
const KEY_CONTEXT: &str = "QuickCapture";

/// A callback without arguments.
pub type CaptureAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The quick-capture box.
#[derive(IntoElement)]
pub struct CaptureBox {
    pub theme: ThemeTokens,
    pub input: Entity<InputState>,
    /// Captures waiting in the Inbox.
    pub inbox_count: usize,
    pub on_save: CaptureAction,
    pub on_close: CaptureAction,
    pub on_open_inbox: CaptureAction,
}

impl RenderOnce for CaptureBox {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        let (close_outside, close_key) = (self.on_close.clone(), self.on_close.clone());
        div()
            .id(BACKDROP_ID)
            .absolute()
            .inset_0()
            .occlude()
            .flex()
            .justify_center()
            .items_start()
            .pt(theme.capture_box_top)
            .on_mouse_down(MouseButton::Left, move |_, window, cx| close_outside(window, cx))
            .child(
                div()
                    .id(BOX_ID)
                    .key_context(KEY_CONTEXT)
                    .on_action(move |_: &Escape, window, cx| close_key(window, cx))
                    .w(theme.capture_box_width)
                    .max_w_full()
                    .p(theme.card_padding)
                    .rounded(theme.radius_lg)
                    .border(theme.border_width_hairline)
                    .border_color(theme.accent_brand_border)
                    .bg(theme.bg_elevated)
                    .elevation_shadow(theme)
                    .flex()
                    .flex_col()
                    .gap(theme.stack_gap)
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(self.entry_row())
                    .child(self.hint_row()),
            )
    }
}

impl CaptureBox {
    fn entry_row(&self) -> Div {
        let theme = &self.theme;
        let save = self.on_save.clone();
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(Icon::new(LucideIcon::Feather).size(theme.icon_size_md).text_color(theme.accent_brand))
            .child(div().flex_1().min_w_0().child(Input::new(&self.input).appearance(false).text_size(theme.font_size_base)))
            .child(Button::new(SAVE_ID).primary().small().label(t!("capture.save").to_string()).on_click(move |_, w, cx| save(w, cx)))
    }

    fn hint_row(&self) -> Div {
        let theme = &self.theme;
        let open = self.on_open_inbox.clone();
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.control_gap)
            .child(div().flex_1().min_w_0().text_size(theme.font_size_hint).text_color(theme.text_muted).child(t!("capture.hint").to_string()))
            .child(
                Button::new(INBOX_ID)
                    .ghost()
                    .xsmall()
                    .icon(Icon::new(LucideIcon::Inbox).size(theme.icon_size_sm))
                    .label(t!("capture.open_inbox", count = self.inbox_count).to_string())
                    .on_click(move |_, w, cx| open(w, cx)),
            )
    }
}
