//! A belief's revision log: each change with its old and new credence, why, from where and
//! when, newest first.

use super::super::data::BeliefRow;
use super::super::BeliefsPanel;
use super::text::{change_text, reason_text, source_text, stamp, tr};
use crate::ui::features::arena::kit::{caption, section_label};
use gpui_kit::*;

impl BeliefsPanel {
    pub(super) fn render_history(&self, row: &BeliefRow) -> Div {
        let theme = &self.theme;
        let lines = row.history.iter().map(|revision| {
            let change = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(change_text(revision));
            let meta = format!("{} · {}", source_text(&revision.source), stamp(revision.at));
            div()
                .w_full()
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .pl(theme.spacing_sm)
                .border_l(theme.border_width_threadline)
                .border_color(theme.border_subtle)
                .child(change)
                .child(caption(reason_text(revision), theme))
                .child(caption(meta, theme))
        });
        div().w_full().flex().flex_col().gap(theme.spacing_xs).child(section_label(tr("beliefs.history"), theme)).children(lines)
    }
}
