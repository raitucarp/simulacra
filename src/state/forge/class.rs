//! Applying an archetype class (a "Thinkers" palette) to a draft, and previewing it.

use super::change::FieldChange;
use super::draft::AgentDraft;
use super::palette::{apply_palette, clear_palette, preview_palette};
use crate::domain::archetype::ArchetypeClass;

/// Every field `class` would change on the draft, grouped by step in walking order.
pub fn preview_diff(draft: &AgentDraft, class: ArchetypeClass) -> Vec<FieldChange> {
    class.palette().map(|palette| preview_palette(draft, palette)).unwrap_or_default()
}

/// Applies `class` to the draft (and records it as the draft's class) as one undo step;
/// returns whether anything changed.
pub fn apply_class(draft: &mut AgentDraft, class: ArchetypeClass) -> bool {
    class.palette().is_some_and(|palette| apply_palette(draft, palette))
}

/// Records which class the draft started from (`None`: from scratch or a description).
pub fn set_class(draft: &mut AgentDraft, class: Option<ArchetypeClass>) -> bool {
    match class {
        Some(class) => draft.transform(|profile| {
            profile.config.archetype_class = Some(class);
            profile.config.palette = Some(class.key().to_string());
        }),
        None => clear_palette(draft),
    }
}
