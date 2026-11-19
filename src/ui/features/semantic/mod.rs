//! Semantic search surfaces (sb B3): the scored result row with its similarity bar, the
//! "find similar" sheet and the indexing indicator. The shell owns their state and wiring.

mod indicator;
mod row;
mod sheet;

pub use indicator::{render_indexing_indicator, BadgeAction, IndexingBadge};
pub use row::{render_scored_row, render_similarity};
pub use sheet::{hit_icon, render_similar_sheet, HitAction, SheetActions, SimilarView};
