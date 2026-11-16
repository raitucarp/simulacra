//! Keyboard reachability for the forge's custom controls: a tab stop with the theme's
//! focus ring (shown only while navigating by keyboard) that runs its action on Enter / Space.

use super::ActionCallback;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::ACTIVATE_KEYS;
use gpui_kit::*;

/// Tab order of the forge's custom controls: document order, like the kit's own widgets.
const TAB_ORDER: isize = 0;

/// The focus ring as an inset band along the element's edge (painted over its background, so
/// translucent fills never tint, and never clipped by a scrolling parent).
pub fn focus_ring_shadow(theme: &ThemeTokens) -> Vec<BoxShadow> {
    vec![BoxShadow::new(Pixels::ZERO, Pixels::ZERO, theme.focus_ring.into()).spread_radius(theme.focus_ring_width).inset()]
}

/// Whether `event` is a plain Enter / Space press (no modifier held).
pub fn is_activation(event: &KeyDownEvent) -> bool {
    let keystroke = &event.keystroke;
    !keystroke.modifiers.modified() && ACTIVATE_KEYS.contains(&keystroke.key.as_str())
}

/// Focus-ring styling and Enter / Space activation for any stateful element.
pub trait KeyboardStop: StatefulInteractiveElement + Styled + Sized {
    /// Shows the focus ring while this element (tracking its own or a given focus handle)
    /// has keyboard focus.
    fn focus_ring(self, theme: &ThemeTokens) -> Self {
        let (color, ring) = (theme.focus_ring, focus_ring_shadow(theme));
        self.focus_visible(move |style| style.border_color(color).shadow(ring))
    }

    /// Makes the element a tab stop (in document order) with the focus ring.
    fn tab_stop_ring(self, theme: &ThemeTokens) -> Self {
        self.tab_index(TAB_ORDER).focus_ring(theme)
    }

    /// Makes the element a tab stop with the focus ring; Enter / Space run `on_activate`.
    fn keyboard_stop(self, theme: &ThemeTokens, on_activate: ActionCallback) -> Self {
        self.tab_stop_ring(theme).on_key_down(move |event: &KeyDownEvent, window, cx| {
            if is_activation(event) {
                cx.stop_propagation();
                on_activate(window, cx);
            }
        })
    }
}

impl<E: StatefulInteractiveElement + Styled + Sized> KeyboardStop for E {}
