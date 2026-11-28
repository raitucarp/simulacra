//! The indexing indicator: a small pill floating at the bottom-right of the window while the
//! background queue embeds ("Indexing 120 of 800" with a thin progress bar and a stop
//! button), or after it paused on a failure (click to open the Embeddings settings).

use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const INDICATOR_ID: &str = "semantic-indexing-indicator";
const STOP_ID: &str = "semantic-indexing-stop";

/// What the pill shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexingBadge {
    Progress { done: usize, total: usize },
    /// Paused with this short, localized reason.
    Paused { reason: String },
}

/// A window-level action of the pill.
pub type BadgeAction = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// The floating pill; `on_stop` cancels (or dismisses a paused badge), `on_open` shows the
/// Embeddings settings.
pub fn render_indexing_indicator(badge: &IndexingBadge, on_stop: BadgeAction, on_open: BadgeAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (icon, color) = match badge {
        IndexingBadge::Progress { .. } => (LucideIcon::RefreshCw, theme.accent_brand),
        IndexingBadge::Paused { .. } => (LucideIcon::CircleAlert, theme.warning),
    };
    div()
        .id(INDICATOR_ID)
        .absolute()
        .bottom(theme.panel_padding)
        .right(theme.panel_padding)
        .w(theme.indexing_indicator_width)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.card_padding)
        .py(theme.spacing_sm)
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .elevation_shadow(theme)
        .cursor_pointer()
        .on_click(move |_, window, cx| on_open(window, cx))
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
        .child(render_text(badge, theme))
        .child(stop_button(on_stop))
}

/// The title, the pause reason and the progress bar.
fn render_text(badge: &IndexingBadge, theme: &ThemeTokens) -> Div {
    let (title, fraction, reason) = match badge {
        IndexingBadge::Progress { done, total } => {
            let fraction = if *total == 0 { 0.0 } else { *done as f32 / *total as f32 };
            (t!("semantic.indexing.progress", done = done, total = total).to_string(), Some(fraction), None)
        }
        IndexingBadge::Paused { reason } => (t!("semantic.indexing.paused").to_string(), None, Some(reason.clone())),
    };
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_primary).child(title))
        .children(reason.map(|reason| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(reason))) // the whole reason: cut at two lines it lost its advice mid-word
        .children(fraction.map(|fraction| progress_bar(fraction, theme)))
}

fn stop_button(on_stop: BadgeAction) -> Button {
    Button::new(STOP_ID)
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::X))
        .tooltip(t!("semantic.indexing.cancel").to_string())
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_stop(window, cx);
        })
}

fn progress_bar(fraction: f32, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .h(theme.similarity_bar_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .overflow_hidden()
        .child(div().h_full().w(relative(fraction.clamp(0.0, 1.0))).bg(theme.accent_brand))
}
