//! Walking a palace: the route visits rooms in order and each room's loci in order; a walk
//! can cover every placed locus or only those whose item is due (or new). At each stop the
//! learner recalls what is placed, reveals it and grades it; the grade of a stop with a
//! review item is an FSRS review in mode `palace`. A forgotten stop is not re-queued: the
//! route stays the route.

use super::{Palace, PalaceId, PlacedTarget};
use crate::domain::review::{Answer, Grade, ItemId, ReviewItem, ReviewMode};
use chrono::NaiveDate;

/// Which loci a walk visits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum WalkScope {
    /// Every placed locus.
    #[default]
    All,
    /// Only loci whose review item is due today or new.
    Due,
}

/// One stop of a walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WalkStop {
    /// Index of the room in the palace.
    pub room: usize,
    /// Index of the locus in its room.
    pub locus: usize,
    pub target: PlacedTarget,
    /// The review item graded at this stop (a placed item, or the active item on a placed post).
    pub item: Option<ItemId>,
}

/// Every placed locus in walking order, as (room index, locus index, target).
pub fn route(palace: &Palace) -> Vec<(usize, usize, PlacedTarget)> {
    palace
        .rooms
        .iter()
        .enumerate()
        .flat_map(|(r, room)| room.loci.iter().enumerate().filter_map(move |(l, locus)| locus.placement.as_ref().map(|p| (r, l, p.target.clone()))))
        .collect()
}

/// The stops of a walk over `palace`; `item_of` finds the active review item of a target.
pub fn plan_walk(palace: &Palace, scope: WalkScope, item_of: &dyn Fn(&PlacedTarget) -> Option<ReviewItem>, today: NaiveDate) -> Vec<WalkStop> {
    route(palace)
        .into_iter()
        .filter_map(|(room, locus, target)| {
            let item = item_of(&target);
            let due = item.as_ref().is_some_and(|item| item.schedule.is_new() || item.schedule.is_due(today));
            (scope == WalkScope::All || due).then(|| WalkStop { room, locus, item: item.map(|item| item.id), target })
        })
        .collect()
}

/// A walk in progress.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Walk {
    pub palace_id: PalaceId,
    pub stops: Vec<WalkStop>,
    /// Index of the current stop.
    pub position: usize,
    pub revealed: bool,
    /// The grade given at each stop passed.
    pub grades: Vec<Grade>,
}

impl Walk {
    pub fn new(palace_id: PalaceId, stops: Vec<WalkStop>) -> Self {
        Self { palace_id, stops, position: 0, revealed: false, grades: Vec::new() }
    }

    pub fn current(&self) -> Option<&WalkStop> {
        self.stops.get(self.position)
    }

    pub fn is_finished(&self) -> bool {
        self.position >= self.stops.len()
    }

    /// Shows what is placed at the current stop.
    pub fn reveal(self) -> Self {
        Self { revealed: !self.is_finished(), ..self }
    }

    /// Grades the revealed stop and walks on; returns the review answer of its item, if any.
    pub fn grade(self, grade: Grade) -> (Self, Option<Answer>) {
        let Some(stop) = self.current().filter(|_| self.revealed).cloned() else { return (self, None) };
        let answer = stop.item.map(|item_id| Answer { item_id, grade, mode: ReviewMode::Palace, confidence: None, correct: None });
        let mut grades = self.grades.clone();
        grades.push(grade);
        (Self { position: self.position + 1, revealed: false, grades, ..self }, answer)
    }

    /// Share of graded stops that were recalled, once any was graded.
    pub fn recalled_share(&self) -> Option<f64> {
        (!self.grades.is_empty()).then(|| self.grades.iter().filter(|g| g.is_recall()).count() as f64 / self.grades.len() as f64)
    }
}
