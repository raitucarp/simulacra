//! The reaction bar (R6): one chip per emoji with its count, the acting persona's own chips
//! highlighted, and a tooltip naming the personas and agents who reacted. Clicking a chip
//! adds or takes back the acting persona's reaction with that emoji.

use super::{send, ReactionAction, ReactionEvent};
use crate::domain::reaction::PostReactions;
use crate::domain::{PostId, ProfileId};
use crate::theme::ThemeTokens;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const CHIP_ID_PREFIX: &str = "reaction-chip-";
/// Names listed in a chip's tooltip before "and N more".
pub const TOOLTIP_NAMES: usize = 8;

/// What one chip shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChipView {
    pub emoji: String,
    pub total: u32,
    /// The acting persona reacted with this emoji.
    pub mine: bool,
    pub tooltip: String,
}

/// "Ada, Socrates and 3 more": the first [`TOOLTIP_NAMES`] names, then how many are left.
pub fn names_line(names: &[String]) -> String {
    let shown = names[..names.len().min(TOOLTIP_NAMES)].join(", ");
    match names.len().saturating_sub(TOOLTIP_NAMES) {
        0 => shown,
        more => t!("reactions.chip.more", names = shown, count = more).to_string(),
    }
}

/// The chips of `set`, in the order emoji were first given; `name_of` names a profile (the
/// acting persona is marked as "you").
pub fn chip_views(set: &PostReactions, acting: &ProfileId, name_of: &dyn Fn(&ProfileId) -> String) -> Vec<ChipView> {
    set.chips()
        .into_iter()
        .map(|chip| {
            let names: Vec<String> = chip.profiles.iter().map(|id| match id == acting {
                true => t!("reactions.chip.you", name = name_of(id)).to_string(),
                false => name_of(id),
            }).collect();
            let tooltip = t!("reactions.chip.tooltip", emoji = chip.emoji.as_str(), names = names_line(&names)).to_string();
            ChipView { mine: chip.includes(acting), emoji: chip.emoji, total: chip.total, tooltip }
        })
        .collect()
}

/// One chip: the emoji and its count.
fn chip(index: usize, post: &PostId, view: &ChipView, on_event: &ReactionAction) -> Button {
    let toggle = send(on_event, ReactionEvent::Toggle(post.clone(), view.emoji.clone()));
    Button::new(SharedString::from(format!("{CHIP_ID_PREFIX}{post}-{index}")))
        .outline()
        .xsmall()
        .selected(view.mine)
        .label(format!("{} {}", view.emoji, view.total))
        .tooltip(view.tooltip.clone())
        .on_click(move |_, window, cx| toggle(window, cx))
}

/// The bar: every chip, wrapping; `None` without reactions.
pub fn render_reaction_bar(post: &PostId, chips: &[ChipView], on_event: &ReactionAction, theme: &ThemeTokens) -> Option<Div> {
    if chips.is_empty() {
        return None;
    }
    let chips = chips.iter().enumerate().map(|(index, view)| chip(index, post, view, on_event));
    Some(div().w_full().flex().flex_wrap().items_center().gap(theme.spacing_xs).children(chips))
}
