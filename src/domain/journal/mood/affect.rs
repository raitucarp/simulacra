//! The affect grid (Russell's circumplex of affect): a point of valence (unpleasant to
//! pleasant) and arousal (calm to activated), each in -1..=1, read as one of eight labelled
//! regions around the circle, or neutral near the centre.

use serde::{Deserialize, Serialize};

/// Distance from the centre under which a feeling reads as neutral.
pub const NEUTRAL_RADIUS: f32 = 0.2;
/// The axis range.
const AXIS: f32 = 1.0;
/// Degrees per region (eight around the circle).
const REGION_DEGREES: f32 = 45.0;
const FULL_TURN: f32 = 360.0;

/// A point on the grid.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Affect {
    /// -1 unpleasant to 1 pleasant.
    pub valence: f32,
    /// -1 deactivated (calm, tired) to 1 activated (alert, tense).
    pub arousal: f32,
}

impl Affect {
    /// A point clamped onto the grid.
    pub fn new(valence: f32, arousal: f32) -> Self {
        Self { valence: valence.clamp(-AXIS, AXIS), arousal: arousal.clamp(-AXIS, AXIS) }
    }

    /// The point of a click at (`x`, `y`) shares of the grid's width and height (top left
    /// is (0, 0): unpleasant and activated).
    pub fn from_grid(x: f32, y: f32) -> Self {
        Self::new(x.clamp(0.0, 1.0) * 2.0 - AXIS, AXIS - y.clamp(0.0, 1.0) * 2.0)
    }

    /// Its position as shares of the grid (the inverse of [`Self::from_grid`]).
    pub fn grid_position(self) -> (f32, f32) {
        ((self.valence + AXIS) / 2.0, (AXIS - self.arousal) / 2.0)
    }

    pub fn region(self) -> AffectRegion {
        AffectRegion::of(self)
    }
}

/// A labelled region of the circumplex, counter-clockwise from pleasant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AffectRegion {
    Pleasant,
    Excited,
    Alert,
    Tense,
    Unpleasant,
    Sad,
    Tired,
    Calm,
    Neutral,
}

impl AffectRegion {
    /// The eight regions around the circle, starting at pleasant (0°).
    pub const AROUND: [Self; 8] = [Self::Pleasant, Self::Excited, Self::Alert, Self::Tense, Self::Unpleasant, Self::Sad, Self::Tired, Self::Calm];

    pub fn of(affect: Affect) -> Self {
        if affect.valence.hypot(affect.arousal) < NEUTRAL_RADIUS {
            return Self::Neutral;
        }
        let degrees = affect.arousal.atan2(affect.valence).to_degrees().rem_euclid(FULL_TURN);
        let index = ((degrees + REGION_DEGREES / 2.0) / REGION_DEGREES).floor() as usize % Self::AROUND.len();
        Self::AROUND[index]
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Pleasant => "pleasant",
            Self::Excited => "excited",
            Self::Alert => "alert",
            Self::Tense => "tense",
            Self::Unpleasant => "unpleasant",
            Self::Sad => "sad",
            Self::Tired => "tired",
            Self::Calm => "calm",
            Self::Neutral => "neutral",
        }
    }

    /// Where its label sits on the grid, as shares (the centre of its arc at 0.8 of the
    /// radius; the centre for neutral).
    pub fn label_position(self) -> (f32, f32) {
        let Some(index) = Self::AROUND.iter().position(|r| *r == self) else { return (0.5, 0.5) };
        let radians = (index as f32 * REGION_DEGREES).to_radians();
        Affect::new(radians.cos() * LABEL_RADIUS, radians.sin() * LABEL_RADIUS).grid_position()
    }
}

/// Share of the radius at which region labels sit.
const LABEL_RADIUS: f32 = 0.78;
