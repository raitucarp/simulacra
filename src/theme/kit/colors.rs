//! Field-by-field mapping of [`ThemeTokens`] onto gpui-component's theme colors.

use crate::theme::color::{mix, readable_on, to_hex, AA_UI_CONTRAST, BLACK, TRANSPARENT, WHITE};
use crate::theme::ThemeTokens;
use gpui_kit::component::ThemeConfigColors;
use gpui_kit::{Rgba, SharedString};

/// How far a hovered / pressed fill moves toward the foreground.
const HOVER_MIX: f32 = 0.12;
const ACTIVE_MIX: f32 = 0.2;
/// How far a menu / list highlight moves from the elevated surface toward the foreground.
const HIGHLIGHT_MIX: f32 = 0.08;
/// How far a selected list row is tinted with the accent.
const LIST_ACTIVE_MIX: f32 = 0.16;

fn hex(color: Rgba) -> Option<SharedString> {
    Some(to_hex(color).into())
}

/// A filled control color with its readable foreground, hover and pressed variants.
struct Fill {
    base: Option<SharedString>,
    foreground: Option<SharedString>,
    hover: Option<SharedString>,
    active: Option<SharedString>,
}

fn fill(color: Rgba, foreground: Rgba, t: &ThemeTokens) -> Fill {
    Fill {
        base: hex(color),
        foreground: hex(foreground),
        hover: hex(mix(color, t.text_primary, HOVER_MIX)),
        active: hex(mix(color, t.text_primary, ACTIVE_MIX)),
    }
}

fn status_fill(color: Rgba, t: &ThemeTokens) -> Fill {
    fill(color, readable_on(color, WHITE, BLACK, AA_UI_CONTRAST), t)
}

/// App background, text, borders, focus ring, caret, links, selection and window chrome.
pub(super) fn apply_base(c: &mut ThemeConfigColors, t: &ThemeTokens) {
    c.background = hex(t.bg_app);
    c.foreground = hex(t.text_primary);
    c.border = hex(t.border);
    c.input = hex(t.border_strong);
    c.ring = hex(t.border_focus);
    c.muted = hex(t.bg_elevated);
    c.muted_foreground = hex(t.text_muted);
    c.caret = hex(t.accent_brand);
    c.link = hex(t.accent_user);
    c.selection = hex(t.selection);
    c.overlay = hex(t.overlay);
    c.window_border = hex(t.border);
    c.drag_border = hex(t.accent_brand);
    c.title_bar = hex(t.bg_app);
    c.title_bar_border = hex(t.border);
    c.status_bar = hex(t.bg_app);
    c.status_bar_border = hex(t.border);
}

/// Primary (accent), secondary and ghost/default button colors plus menu highlights.
pub(super) fn apply_buttons(c: &mut ThemeConfigColors, t: &ThemeTokens) {
    let primary = fill(t.accent_brand, t.on_accent, t);
    (c.primary, c.primary_foreground, c.primary_hover, c.primary_active) =
        (primary.base, primary.foreground, primary.hover, primary.active);
    let secondary = fill(t.bg_elevated, t.text_primary, t);
    (c.secondary, c.secondary_foreground, c.secondary_hover, c.secondary_active) =
        (secondary.base, secondary.foreground, secondary.hover, secondary.active);
    c.button = hex(t.bg_elevated);
    c.button_foreground = hex(t.text_primary);
    c.button_hover = hex(mix(t.bg_elevated, t.text_primary, HIGHLIGHT_MIX));
    c.button_active = hex(mix(t.bg_elevated, t.text_primary, HOVER_MIX));
    c.accent = hex(mix(t.bg_elevated, t.text_primary, HIGHLIGHT_MIX));
    c.accent_foreground = hex(t.text_primary);
}

/// Danger, success, warning and info fills with readable foregrounds.
pub(super) fn apply_statuses(c: &mut ThemeConfigColors, t: &ThemeTokens) {
    let danger = fill(t.accent_danger, t.on_danger, t);
    (c.danger, c.danger_foreground, c.danger_hover, c.danger_active) =
        (danger.base, danger.foreground, danger.hover, danger.active);
    let success = status_fill(t.accent_success, t);
    (c.success, c.success_foreground, c.success_hover, c.success_active) =
        (success.base, success.foreground, success.hover, success.active);
    let warning = status_fill(t.warning, t);
    (c.warning, c.warning_foreground, c.warning_hover, c.warning_active) =
        (warning.base, warning.foreground, warning.hover, warning.active);
    let info = status_fill(t.info, t);
    (c.info, c.info_foreground, c.info_hover, c.info_active) = (info.base, info.foreground, info.hover, info.active);
}

/// Popovers, lists, tables, sidebars, tabs and scrollbars.
pub(super) fn apply_surfaces(c: &mut ThemeConfigColors, t: &ThemeTokens) {
    let highlight = hex(mix(t.bg_elevated, t.text_primary, HIGHLIGHT_MIX));
    let row_active = hex(mix(t.bg_card, t.accent_brand, LIST_ACTIVE_MIX));
    c.popover = hex(t.bg_elevated);
    c.popover_foreground = hex(t.text_primary);
    (c.list, c.list_even, c.list_head) = (hex(t.bg_card), hex(t.bg_card), hex(t.bg_elevated));
    (c.list_hover, c.list_active, c.list_active_border) = (highlight.clone(), row_active.clone(), hex(t.accent_brand));
    (c.table, c.table_even, c.table_head) = (hex(t.bg_card), hex(t.bg_card), hex(t.bg_elevated));
    (c.table_hover, c.table_active, c.table_active_border) = (highlight.clone(), row_active, hex(t.accent_brand));
    (c.table_head_foreground, c.table_row_border) = (hex(t.text_muted), hex(t.border_subtle));
    (c.sidebar, c.sidebar_foreground, c.sidebar_border) = (hex(t.bg_sidebar), hex(t.text_primary), hex(t.border));
    (c.sidebar_accent, c.sidebar_accent_foreground) = (highlight, hex(t.text_primary));
    (c.sidebar_primary, c.sidebar_primary_foreground) = (hex(t.accent_brand), hex(t.on_accent));
    (c.tab_bar, c.tab, c.tab_active) = (hex(t.bg_card), hex(TRANSPARENT), hex(t.bg_elevated));
    (c.tab_foreground, c.tab_active_foreground) = (hex(t.text_muted), hex(t.text_primary));
    (c.scrollbar, c.scrollbar_thumb, c.scrollbar_thumb_hover) =
        (hex(TRANSPARENT), hex(t.scrollbar), hex(t.scrollbar_hover));
    (c.switch, c.slider_bar, c.progress_bar) = (hex(t.border_strong), hex(t.accent_brand), hex(t.accent_brand));
    (c.skeleton, c.accordion, c.group_box) = (hex(t.bg_elevated), hex(t.bg_card), hex(t.bg_card));
}

/// Chart series follow the palette accents; candles use success / danger.
pub(super) fn apply_charts(c: &mut ThemeConfigColors, t: &ThemeTokens) {
    c.chart_1 = hex(t.accent_brand);
    c.chart_2 = hex(t.accent_user);
    c.chart_3 = hex(t.accent_agent);
    c.chart_4 = hex(t.accent_success);
    c.chart_5 = hex(t.warning);
    c.chart_bullish = hex(t.accent_success);
    c.chart_bearish = hex(t.accent_danger);
    c.chart_grid = hex(t.border_subtle);
}
