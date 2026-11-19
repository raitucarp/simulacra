//! The credence chip (sb E1): a compact "70%" on the user's own posts that carry a credence,
//! opening a small popover to set the percentage and the "resolves by" date, or to resolve
//! the prediction (true, false, ambiguous, with a note), reopen or remove it.

mod chip;
mod popover;

pub use chip::{render_credence_chip, with_credence_item};

use crate::domain::thinking::credence::{Credence, DuePreset, Outcome};
use crate::domain::PostId;
use jiff::civil::Date;
use gpui_kit::component::input::InputState;
use gpui_kit::{App, Entity, Window};
use std::collections::HashMap;
use std::rc::Rc;

/// What the chip shows of a post's credence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CredenceChip {
    pub percent: u8,
    pub outcome: Option<Outcome>,
    pub due: Option<Date>,
    /// Open and past its date.
    pub is_due: bool,
    pub sentence: Option<String>,
    pub note: String,
}

impl CredenceChip {
    pub fn of(credence: &Credence, today: Date) -> Self {
        Self {
            percent: credence.percent,
            outcome: credence.outcome,
            due: credence.resolves_by,
            is_due: credence.is_due(today),
            sentence: credence.sentence.clone(),
            note: credence.note.clone(),
        }
    }
}

/// Something done with a post's credence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CredenceEvent {
    /// Open the popover (the "…" menu's "Set a credence").
    Open(PostId),
    Close,
    SetPercent(PostId, u8),
    /// Move the percentage by this many points.
    Nudge(PostId, i8),
    SetDue(PostId, Option<DuePreset>),
    /// Resolve with the note typed in the popover.
    Resolve(PostId, Outcome),
    Reopen(PostId),
    Remove(PostId),
}

/// Sends a credence event to the shell.
pub type CredenceAction = Rc<dyn Fn(CredenceEvent, &mut Window, &mut App) + 'static>;

/// The credence controls of one post card (only on the user's own posts).
#[derive(Clone)]
pub struct CredenceControls {
    pub post_id: PostId,
    pub chip: Option<CredenceChip>,
    /// The popover is open on this card.
    pub open: bool,
    /// The resolution note field (one, shared: only one popover is open at a time).
    pub note: Entity<InputState>,
    pub on_event: CredenceAction,
}

/// What the feed knows to build every card's controls.
#[derive(Clone)]
pub struct CredenceSource {
    /// Chips of the posts that carry a credence, by post id.
    pub chips: Rc<HashMap<String, CredenceChip>>,
    /// The post whose popover is open.
    pub open: Option<PostId>,
    pub note: Entity<InputState>,
    pub on_event: CredenceAction,
}

impl CredenceSource {
    /// The controls of `post`'s card.
    pub fn controls(&self, post: &PostId) -> CredenceControls {
        CredenceControls {
            post_id: post.clone(),
            chip: self.chips.get(post.as_str()).cloned(),
            open: self.open.as_ref() == Some(post),
            note: self.note.clone(),
            on_event: self.on_event.clone(),
        }
    }
}

/// Percentages offered as one-click choices.
pub const PERCENT_PRESETS: [u8; 7] = [5, 20, 35, 50, 65, 80, 95];
/// Points one nudge moves.
pub const NUDGE_POINTS: i8 = 5;
