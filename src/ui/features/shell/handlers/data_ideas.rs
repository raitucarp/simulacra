//! N6–N10: the open idea sheet over the shell (what-if branch, pinned quote), with the event
//! that closes it.

use super::super::view::AppView;
use crate::ui::features::data::{render_branch_sheet, render_quote_sheet, BranchEvent, DataAction, DataEvent, QuoteEvent};
use gpui_kit::*;

impl AppView {
    /// The open N6–N10 sheet and the event closing it, if one is open.
    pub(in crate::ui::features::shell) fn idea_overlay(&self, on_event: DataAction) -> Option<(Stateful<Div>, DataEvent)> {
        let (sheets, theme) = (&self.data.ideas, &self.theme);
        if let Some(dialog) = &sheets.branch {
            return Some((render_branch_sheet(dialog, on_event, theme), DataEvent::Branch(BranchEvent::Close)));
        }
        let dialog = sheets.quote.as_ref()?;
        Some((render_quote_sheet(dialog, on_event, theme), DataEvent::Quote(QuoteEvent::Close)))
    }
}
