//! The clap button (R7): the clap emoji and the post's clap count. A click claps once, an
//! Alt-click takes one of the acting persona's claps back, and holding it claps in a burst
//! (the shell times the burst, see [`crate::domain::reaction::gesture`]).

use super::chips::names_line;
use super::{ReactionAction, ReactionEvent};
use crate::domain::reaction::{PostReactions, CLAP};
use crate::domain::{PostId, ProfileId};
use crate::theme::ThemeTokens;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const CLAP_ID_PREFIX: &str = "reaction-clap-";

/// What the clap button shows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClapView {
    pub total: u32,
    /// The acting persona's own claps.
    pub mine: u32,
    pub tooltip: String,
}

/// The clap button's view of `set` (no reactions: no claps).
pub fn clap_view(set: Option<&PostReactions>, acting: &ProfileId, name_of: &dyn Fn(&ProfileId) -> String) -> ClapView {
    let how = t!("reactions.clap.how").to_string();
    let Some(set) = set.filter(|set| set.claps() > 0) else { return ClapView { tooltip: how, ..ClapView::default() } };
    let names: Vec<String> = set.clappers().into_iter().map(|(id, count)| {
        let name = if id == *acting { t!("reactions.chip.you", name = name_of(&id)).to_string() } else { name_of(&id) };
        t!("reactions.clap.count", name = name, count = count).to_string()
    }).collect();
    let tooltip = format!("{}\n{how}", t!("reactions.clap.tooltip", names = names_line(&names)));
    ClapView { total: set.claps(), mine: set.claps_by(acting), tooltip }
}

/// The button, pressed and released through `on_event`.
pub fn render_clap_button(post: &PostId, view: &ClapView, on_event: &ReactionAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (press, release, release_out) = (on_event.clone(), on_event.clone(), on_event.clone());
    let (press_id, release_id, out_id) = (post.clone(), post.clone(), post.clone());
    let colour = if view.mine > 0 { theme.accent_brand } else { theme.text_muted };
    let (hover_bg, tooltip) = (theme.hover_bg, view.tooltip.clone());
    div()
        .id(SharedString::from(format!("{CLAP_ID_PREFIX}{post}")))
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::MEDIUM)
        .text_color(colour)
        .hover(move |s| s.bg(hover_bg))
        .tooltip(move |window, cx| Tooltip::new(tooltip.clone()).build(window, cx))
        .on_mouse_down(MouseButton::Left, move |event: &MouseDownEvent, window, cx| {
            cx.stop_propagation();
            press(ReactionEvent::ClapPress(press_id.clone(), event.modifiers.alt), window, cx);
        })
        .on_mouse_up(MouseButton::Left, move |_, window, cx| release(ReactionEvent::ClapRelease(release_id.clone()), window, cx))
        .on_mouse_up_out(MouseButton::Left, move |_, window, cx| release_out(ReactionEvent::ClapRelease(out_id.clone()), window, cx))
        .child(CLAP)
        .when(view.total > 0, |button| button.child(view.total.to_string()))
}
