//! Associations with what you already do (posts written, review sessions, focus minutes),
//! each sentence with its number of days, under a note that these are associations, not
//! causes, and not advice.

use super::super::panel::JournalPanel;
use super::super::text::{association_sentence, tr};
use super::common::{block_title, caption, surface};
use crate::domain::journal::mood::correlate::{associations, Strength};
use crate::domain::journal::mood::summary::daily_means;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;

impl JournalPanel {
    pub(super) fn render_associations(&self) -> Div {
        let theme = &self.theme;
        let links = associations(&daily_means(&self.data.book.checkins), &self.data.activity);
        let rows = links.iter().map(|link| {
            let colour = match link.strength {
                Strength::TooFewDays | Strength::NoVariation | Strength::Negligible => theme.text_muted,
                _ if link.is_positive() => theme.accent_success,
                _ => theme.accent_danger,
            };
            div()
                .flex()
                .items_start()
                .gap(theme.inline_gap)
                .child(div().flex_none().mt(theme.spacing_2xs).size(theme.dot_size_sm).rounded(theme.radius_full).bg(colour))
                .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(association_sentence(link)))
        });
        surface(theme)
            .child(block_title(LucideIcon::GitCompareArrows, theme.accent_dialectic, tr("journal.assoc.title"), theme))
            .child(caption(tr("journal.assoc.caveat"), theme))
            .children(rows)
    }
}
