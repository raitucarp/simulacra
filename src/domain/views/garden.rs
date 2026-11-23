//! N3: the garden view of a list of posts: its garden notes in beds by maturity, evergreen
//! first, each bed most recently tended first. A note shows its text on one line, the
//! thread it heads and when it was planted; a locked note shows only its placeholder.

use super::rapid_log::one_line;
use crate::domain::post::garden::Maturity;
use crate::domain::{Post, PostId};
use jiff::Timestamp;
use std::cmp::Reverse;

/// One garden note.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GardenEntry {
    pub post_id: PostId,
    pub text: String,
    pub locked: bool,
    pub planted: Timestamp,
    /// When its stage last changed, else when it was planted.
    pub tended: Timestamp,
    /// How many replies it heads (its thread).
    pub replies: usize,
}

/// The notes at one stage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GardenBed {
    pub maturity: Maturity,
    pub notes: Vec<GardenEntry>,
}

/// The beds of `posts`' garden notes, evergreen first; a stage without notes has no bed.
pub fn garden<'a>(posts: impl IntoIterator<Item = &'a Post>) -> Vec<GardenBed> {
    let mut beds: Vec<GardenBed> = Maturity::ALL.iter().rev().map(|maturity| GardenBed { maturity: *maturity, notes: Vec::new() }).collect();
    for post in posts {
        let Some(note) = post.extras.garden else { continue };
        let entry = GardenEntry { post_id: post.id.clone(), text: one_line(post), locked: post.is_locked(), planted: note.planted_at, tended: note.tended_at.unwrap_or(note.planted_at), replies: post.replies_count };
        if let Some(bed) = beds.iter_mut().find(|bed| bed.maturity == note.maturity) {
            bed.notes.push(entry);
        }
    }
    beds.iter_mut().for_each(|bed| bed.notes.sort_by_key(|note| Reverse(note.tended)));
    beds.retain(|bed| !bed.notes.is_empty());
    beds
}

/// How many notes the beds hold.
pub fn note_count(beds: &[GardenBed]) -> usize {
    beds.iter().map(|bed| bed.notes.len()).sum()
}
