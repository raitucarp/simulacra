//! Surfaces, borders and interaction overlays.
//!
//! Hover, pressed and active-tab colors are the palette foreground at a low alpha, so they
//! lighten dark themes and darken light themes alike, on whatever surface they sit.

use crate::theme::color::{mix, with_alpha, TRANSPARENT};
use crate::theme::palette::ThemePalette;
use crate::theme::ThemeTokens;

/// Foreground alpha of the ghost hover overlay.
const HOVER_ALPHA: f32 = 0.04;
/// Foreground alpha of the active tab fill.
const TAB_ACTIVE_ALPHA: f32 = 0.05;
/// Foreground alpha of a pressed control.
const PRESSED_ALPHA: f32 = 0.08;
/// Foreground alpha of the inline code and code block fill.
const CODE_ALPHA: f32 = 0.07;
/// Foreground alpha of a hovered window caption button.
const WINDOW_CONTROL_HOVER_ALPHA: f32 = 0.12;
/// Foreground alpha of a resting / hovered scrollbar thumb.
const SCROLLBAR_ALPHA: f32 = 0.18;
const SCROLLBAR_HOVER_ALPHA: f32 = 0.3;
/// How far the subtle border fades back into the background.
const BORDER_SUBTLE_MIX: f32 = 0.2;
/// How far the strong border moves toward the foreground.
const BORDER_STRONG_MIX: f32 = 0.06;
/// How far input fields move from the card surface toward the app background.
const INPUT_BG_MIX: f32 = 0.5;
/// How far graph edges move from the border toward the muted text color.
const GRAPH_EDGE_MIX: f32 = 0.3;

/// Background layers, borders, hover/pressed overlays, inputs and scrollbars.
pub(super) fn apply_surfaces(t: &mut ThemeTokens, palette: &ThemePalette) {
    let c = &palette.colors;
    set_tokens!(t {
        bg_app: c.bg, bg_card: c.surface, bg_elevated: c.elevated, bg_sidebar: c.bg,
        bg_feed_post: c.bg, bg_transparent: TRANSPARENT,
        input_bg: mix(c.surface, c.bg, INPUT_BG_MIX),

        border: c.border, border_subtle: mix(c.border, c.bg, BORDER_SUBTLE_MIX),
        border_strong: mix(c.border, c.fg, BORDER_STRONG_MIX), border_focus: c.accent, focus_ring: c.info,
        graph_edge: mix(c.border, c.fg_muted, GRAPH_EDGE_MIX),

        hover_bg: with_alpha(c.fg, HOVER_ALPHA), bg_tab_active: with_alpha(c.fg, TAB_ACTIVE_ALPHA),
        pressed_bg: with_alpha(c.fg, PRESSED_ALPHA), bg_code: with_alpha(c.fg, CODE_ALPHA),
        bg_window_control_hover: with_alpha(c.fg, WINDOW_CONTROL_HOVER_ALPHA),
        scrollbar: with_alpha(c.fg, SCROLLBAR_ALPHA),
        scrollbar_hover: with_alpha(c.fg, SCROLLBAR_HOVER_ALPHA),
    });
}
