//! Mnemo the guide owl: which pose to draw and which tip to say, by context.

use crate::state::forge::ForgeStep;
use crate::ui::illustration::GuidePose;

const GUIDE_ROOT: &str = "forge.guide";
const START_TIP: &str = "start_tip";
const WAITING_TIP: &str = "waiting_tip";
const CHEER_TIP: &str = "cheer_tip";

/// What the forge is doing right now, as far as Mnemo cares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuideMoment {
    /// The "choose your class" screen.
    Start,
    /// On `step`; `fresh` until the first edit after entering it.
    Step { step: ForgeStep, fresh: bool },
    /// Waiting for the AI to read a description.
    Waiting,
    /// The new thinker was just brought to life.
    Celebrating,
}

/// Steps that are mostly reading rather than choosing.
const READING_STEPS: [ForgeStep; 2] = [ForgeStep::Grimoire, ForgeStep::Review];

/// Think while waiting, Cheer on success, Read on reading steps, Point on step entry,
/// Idle otherwise.
pub fn guide_pose(moment: GuideMoment) -> GuidePose {
    match moment {
        GuideMoment::Waiting => GuidePose::Think,
        GuideMoment::Celebrating => GuidePose::Cheer,
        GuideMoment::Step { step, .. } if READING_STEPS.contains(&step) => GuidePose::Read,
        GuideMoment::Step { fresh: true, .. } | GuideMoment::Start => GuidePose::Point,
        GuideMoment::Step { .. } => GuidePose::Idle,
    }
}

/// i18n key of what Mnemo says.
pub fn guide_tip_key(moment: GuideMoment) -> String {
    match moment {
        GuideMoment::Step { step, .. } => step.guide_tip_key(),
        GuideMoment::Start => format!("{GUIDE_ROOT}.{START_TIP}"),
        GuideMoment::Waiting => format!("{GUIDE_ROOT}.{WAITING_TIP}"),
        GuideMoment::Celebrating => format!("{GUIDE_ROOT}.{CHEER_TIP}"),
    }
}
