//! A persona's feeling post as a mood check-in (M6, opt-in: Settings › Reading & Writing ›
//! "Log my feelings in the journal", off by default). The check-in is written to the private
//! journal only; nothing flows back: like every check-in it never reaches the feed, search,
//! Insights or an agent (`journal::privacy`).
//!
//! The point on the affect grid is the mean of the emojis' points (a small table of common
//! feeling faces; others count as neutral), pushed out from the centre as the intensity rises;
//! energy follows the point's arousal.

use super::{Affect, CheckinDraft, ENERGY_MAX, ENERGY_MIN};
use crate::domain::post::{Intensity, PostKind};

/// (emoji, valence, arousal) of common feeling faces.
const FACES: [(&str, f32, f32); 28] = [
    ("😀", 0.8, 0.5), ("😄", 0.8, 0.6), ("😊", 0.8, 0.2), ("🙂", 0.5, 0.0), ("😂", 0.7, 0.8), ("🥰", 0.9, 0.4), ("😍", 0.9, 0.6),
    ("🤩", 0.9, 0.8), ("🥹", 0.6, 0.3), ("😌", 0.6, -0.5), ("🙃", 0.1, 0.2), ("😴", 0.1, -0.9), ("🥱", -0.1, -0.7), ("😐", 0.0, 0.0),
    ("😕", -0.4, 0.0), ("😟", -0.5, 0.3), ("😞", -0.6, -0.4), ("😔", -0.5, -0.5), ("😢", -0.7, -0.3), ("😭", -0.8, 0.2), ("😬", -0.3, 0.5),
    ("😰", -0.6, 0.7), ("😨", -0.6, 0.8), ("😱", -0.7, 0.9), ("😤", -0.5, 0.7), ("😠", -0.7, 0.7), ("😡", -0.8, 0.9), ("🤯", -0.2, 0.9),
];
/// How far towards its point the faintest feeling sits, and how much further each step goes
/// (the strongest sits on the point itself).
const NEAREST: f32 = 0.4;
const STEP: f32 = 0.15;

fn point_of(emoji: &str) -> (f32, f32) {
    FACES.iter().find(|(face, _, _)| *face == emoji).map_or((0.0, 0.0), |(_, valence, arousal)| (*valence, *arousal))
}

/// The point of `emojis` felt at `intensity`.
pub fn feeling_affect(emojis: &[String], intensity: Intensity) -> Affect {
    let count = emojis.len().max(1) as f32;
    let (valence, arousal) = emojis.iter().map(|emoji| point_of(emoji)).fold((0.0, 0.0), |(v, a), (ev, ea)| (v + ev, a + ea));
    let reach = NEAREST + STEP * f32::from(intensity.level() - Intensity::MIN.level());
    Affect::new(valence / count * reach, arousal / count * reach)
}

/// The check-in a feeling logs (its note, an energy from its arousal); `None` for any other kind.
pub fn checkin_from_feeling(kind: &PostKind) -> Option<CheckinDraft> {
    let PostKind::Feeling { emojis, intensity, note } = kind else { return None };
    let affect = feeling_affect(emojis, *intensity);
    let middle = f32::from(ENERGY_MIN + ENERGY_MAX) / 2.0;
    let energy = (middle + affect.arousal * (middle - f32::from(ENERGY_MIN))).round().clamp(f32::from(ENERGY_MIN), f32::from(ENERGY_MAX)) as u8;
    Some(CheckinDraft { affect: Some(affect), energy, note: note.clone(), ..CheckinDraft::default() })
}
