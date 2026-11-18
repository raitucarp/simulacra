//! Tests of the Arena's pure logic.

mod debate;
mod judge;
mod lab;
mod privacy;
mod publish;

use super::debate::{DebateFormat, DebateSetup, Debater, Judge, Side};
use crate::domain::ProfileId;

pub(super) fn agent(id: &str) -> ProfileId {
    ProfileId::new(id)
}

pub(super) fn name(id: &ProfileId) -> String {
    format!("Name of {id}")
}

/// An Oxford setup with the given `(agent, side)` debaters.
pub(super) fn setup(format: DebateFormat, debaters: &[(&str, Side)], rounds: u8) -> DebateSetup {
    DebateSetup {
        motion: "Cities should ban private cars".into(),
        source_post: None,
        debaters: debaters.iter().map(|(id, side)| Debater { agent: agent(id), side: *side }).collect(),
        format,
        rounds,
        moderator: None,
        judge: Judge::GlobalModel,
    }
}

pub(super) fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339("2026-11-18T10:00:00Z").expect("time").with_timezone(&chrono::Utc)
}
