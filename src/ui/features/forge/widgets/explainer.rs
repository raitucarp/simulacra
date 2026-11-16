//! [`ExplainerButton`]: a small "?" next to a setting that opens a hover card saying what it
//! is, how it changes posts and an example, in plain language. The "?" is a tab stop:
//! Enter / Space pin the card open while it keeps focus; Escape closes every card.

use super::explainer_card::{card, Explainer, EXPLAIN_ROOT};
use super::explainer_state::{generation, set_open, PinState};
use super::focus_ring::{is_activation, KeyboardStop};
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::hover_card::HoverCard;
use gpui_kit::component::Icon;
use gpui_kit::*;
use std::rc::Rc;

const TRIGGER_SUFFIX: &str = "trigger";
const PIN_SUFFIX: &str = "pin";

pub use super::explainer_state::close_explainers;

/// Builds the body of a hover card.
pub type CardBody = Rc<dyn Fn(&ThemeTokens) -> Div>;

/// A "?" icon showing an [`Explainer`] (or any card body) on hover.
#[derive(IntoElement)]
pub struct ExplainerButton {
    id: SharedString,
    icon: LucideIcon,
    body: CardBody,
    theme: ThemeTokens,
}

impl ExplainerButton {
    pub fn new(id: impl Into<SharedString>, explainer: Explainer, theme: &ThemeTokens) -> Self {
        Self::custom(id, LucideIcon::HelpCircle, Rc::new(move |theme: &ThemeTokens| card(&explainer, theme)), theme)
    }

    /// An `icon` trigger for any card `body` (sized and padded by the body itself).
    pub fn custom(id: impl Into<SharedString>, icon: LucideIcon, body: CardBody, theme: &ThemeTokens) -> Self {
        Self { id: id.into(), icon, body, theme: theme.clone() }
    }

    /// A button for field `path`, or `None` when the field has no explainer texts.
    pub fn for_field(path: &str, theme: &ThemeTokens) -> Option<Self> {
        Explainer::for_field(path).map(|explainer| Self::new(SharedString::from(format!("{EXPLAIN_ROOT}-{path}")), explainer, theme))
    }
}


/// The card as a floating surface, for the keyboard-pinned copy.
fn floating(body: &CardBody, theme: &ThemeTokens) -> impl IntoElement {
    let surface = body(theme)
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .elevation_shadow(theme);
    deferred(anchored().snap_to_window_with_margin(theme.spacing_md).child(surface))
}

/// Toggles the pinned card of `key` on Enter / Space.
fn on_trigger_key(pin: &Entity<PinState>, key: &SharedString, event: &KeyDownEvent, cx: &mut App) {
    if !is_activation(event) {
        return;
    }
    cx.stop_propagation();
    let pinned = pin.update(cx, |state, cx| {
        state.pinned = !state.pinned;
        cx.notify();
        state.pinned
    });
    set_open(key, pinned, cx);
}

impl RenderOnce for ExplainerButton {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.clone();
        let key = SharedString::from(format!("{}-{}", self.id, generation(cx)));
        let pin = window.use_keyed_state(SharedString::from(format!("{key}-{PIN_SUFFIX}")), cx, |_, cx| PinState::new(cx));
        let handle = pin.read(cx).handle.clone();
        let pinned = pin.read(cx).pinned && handle.is_focused(window);
        if pin.read(cx).pinned && !pinned {
            pin.update(cx, |state, _| state.pinned = false);
            set_open(&key, false, cx);
        }
        let (key_pin, pin_key) = (pin.clone(), key.clone());
        let trigger = div()
            .id(SharedString::from(format!("{key}-{TRIGGER_SUFFIX}")))
            .track_focus(&handle)
            .focus_ring(&theme)
            .flex_shrink_0()
            .rounded(theme.radius_full)
            .cursor_pointer()
            .text_color(theme.text_muted)
            .on_key_down(move |event: &KeyDownEvent, _, cx| on_trigger_key(&key_pin, &pin_key, event, cx))
            .child(Icon::new(self.icon).size(theme.font_size_sm));
        let (body, card_theme, open_key) = (self.body.clone(), theme.clone(), key.clone());
        let hover = HoverCard::new(key)
            .trigger(trigger)
            .content(move |_, _, _| body(&card_theme))
            .on_open_change(move |open, _, cx| set_open(&open_key, *open, cx));
        div().flex_shrink_0().child(hover).children(pinned.then(|| floating(&self.body, &theme)))
    }
}
