//! Reactions on post cards (epic R): the emoji picker (R5), the reaction bar (R6) and the
//! clap button (R7).
//!
//! The shell owns what the picker remembers and which card it is open on; every card gets
//! [`ReactionControls`] built from one [`ReactionSource`], and sends [`ReactionEvent`]s back.

mod chips;
mod clap;
mod grid;
mod picker;
mod trigger;
#[cfg(test)]
mod tests;

pub use clap::{clap_view, render_clap_button, ClapView};
pub use chips::{chip_views, names_line, render_reaction_bar, ChipView, TOOLTIP_NAMES};
pub use picker::{picker_emoji, tab_label};
pub use trigger::render_add_reaction;

use crate::domain::emoji::{EmojiGroup, PickerMemory, SkinTone};
use crate::domain::{PostId, ProfileId};
use crate::state::AppState;
use gpui_kit::component::input::InputState;
use gpui_kit::{App, Entity, Window};
use std::rc::Rc;

/// The picker's tabs: the recently used emoji, then one tab per group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PickerTab {
    #[default]
    Recent,
    Group(EmojiGroup),
}

impl PickerTab {
    /// Every tab, in order.
    pub fn all() -> impl Iterator<Item = PickerTab> {
        std::iter::once(Self::Recent).chain(EmojiGroup::ALL.into_iter().map(Self::Group))
    }
}

/// Something done with a card's reactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReactionEvent {
    OpenPicker(PostId),
    ClosePicker,
    SelectTab(PickerTab),
    SetTone(Option<SkinTone>),
    /// An emoji picked for the post: toggles the acting persona's reaction with it.
    Pick(PostId, String),
    /// A chip clicked: toggles the acting persona's reaction with its emoji.
    Toggle(PostId, String),
    /// The clap button went down (with Alt held: `true`).
    ClapPress(PostId, bool),
    /// The clap button came up, or the pointer left it while down.
    ClapRelease(PostId),
}

/// Sends a reaction event to the shell.
pub type ReactionAction = Rc<dyn Fn(ReactionEvent, &mut Window, &mut App) + 'static>;

/// The open picker (only one card has it at a time).
#[derive(Clone)]
pub struct PickerView {
    pub search: Entity<InputState>,
    pub tab: PickerTab,
    pub memory: Rc<PickerMemory>,
}

/// The reaction controls of one post card.
#[derive(Clone)]
pub struct ReactionControls {
    pub post_id: PostId,
    /// `Some` while the picker is open on this card.
    pub picker: Option<PickerView>,
    /// The reaction bar's chips (empty without reactions).
    pub chips: Vec<ChipView>,
    pub clap: ClapView,
    pub on_event: ReactionAction,
}

/// What the feed knows to build every card's controls.
#[derive(Clone)]
pub struct ReactionSource {
    /// The post whose picker is open.
    pub open: Option<PostId>,
    pub picker: PickerView,
    pub on_event: ReactionAction,
}

impl ReactionSource {
    /// The controls of `post`'s card, its chips read from `state` (as the acting persona).
    pub fn controls(&self, post: &PostId, state: &AppState) -> ReactionControls {
        let open = self.open.as_ref() == Some(post);
        let name_of = |id: &ProfileId| state.find_author(id).map_or_else(|| id.to_string(), |profile| profile.name.clone());
        let (set, acting) = (state.reactions_of(post), &state.active_profile_id);
        let chips = set.map(|set| chip_views(set, acting, &name_of)).unwrap_or_default();
        let clap = clap_view(set, acting, &name_of);
        ReactionControls { post_id: post.clone(), picker: open.then(|| self.picker.clone()), chips, clap, on_event: self.on_event.clone() }
    }
}

/// Wraps `event` as a click handler.
pub(crate) fn send(on_event: &ReactionAction, event: ReactionEvent) -> impl Fn(&mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |window, cx| on_event(event.clone(), window, cx)
}
