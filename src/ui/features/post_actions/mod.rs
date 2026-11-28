//! Post actions sheet (sb C2): the rail of thinking actions, the chosen action's choices, the
//! AI answer and its buttons (copy, post as reply, save to vault, re-run). The shell owns the
//! state and runs the requests; this module only renders and reports [`SheetEvent`]s.

mod answer;
mod blocks;
mod choices;
mod footer;
mod hits;
mod labels;
mod model;
mod rail;
mod sheet;

pub use labels::{failure_text, group_icon, group_title, kind_icon, kind_title};
pub use model::{ActionChoice, ActionSheet, ActionStatus, ConceptHits, SheetEvent};
pub use sheet::{render_actions_sheet, SheetAction, SheetInputs};
