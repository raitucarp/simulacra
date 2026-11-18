//! The filter bar: date range, audience, post kinds, authors and the network scope. Every
//! chip is a keyboard stop; a change recomputes the report in the background.

use super::super::panel::InsightsPanel;
use super::common::{card, option_chip, option_group, tr, Pick};
use super::Room;
use crate::domain::insights::filter::{Audience, DateRange};
use crate::domain::insights::{InsightsFilter, PostKind};
use gpui_kit::*;

/// Author chips shown at most.
pub const MAX_AUTHOR_CHIPS: usize = 10;

impl InsightsPanel {
    /// A callback applying `change` to the filter.
    fn filter_pick(&self, cx: &mut Context<Self>, change: impl Fn(&mut InsightsFilter) + 'static) -> Pick {
        self.pick(cx, move |state| change(&mut state.filter))
    }

    fn chip_id(group: &str, key: &str) -> ElementId {
        ElementId::Name(format!("insights-filter-{group}-{key}").into())
    }

    fn range_chips(&self, cx: &mut Context<Self>) -> Div {
        let chips = DateRange::ALL.into_iter().map(|range| {
            let pick = self.filter_pick(cx, move |f| f.range = range);
            option_chip(Self::chip_id("range", range.key()), tr(&format!("insights.range.{}", range.key())), self.state.filter.range == range, &self.theme, pick)
        });
        option_group(tr("insights.filter.range"), chips.collect(), &self.theme)
    }

    fn audience_chips(&self, cx: &mut Context<Self>) -> Div {
        let chips = Audience::ALL.into_iter().map(|audience| {
            let pick = self.filter_pick(cx, move |f| f.audience = audience);
            option_chip(Self::chip_id("audience", audience.key()), tr(&format!("insights.audience.{}", audience.key())), self.state.filter.audience == audience, &self.theme, pick)
        });
        option_group(tr("insights.filter.audience"), chips.collect(), &self.theme)
    }

    fn kind_chips(&self, cx: &mut Context<Self>) -> Div {
        let chips = PostKind::ALL.into_iter().map(|kind| {
            let pick = self.filter_pick(cx, move |f| f.toggle_kind(kind));
            let shown = !self.state.filter.hidden_kinds.contains(&kind);
            option_chip(Self::chip_id("kind", kind.key()), tr(&format!("insights.kind.{}", kind.key())), shown, &self.theme, pick)
        });
        option_group(tr("insights.filter.kinds"), chips.collect(), &self.theme)
    }

    fn author_chips(&self, cx: &mut Context<Self>) -> Div {
        let everyone = self.filter_pick(cx, |f| f.authors.clear());
        let mut chips = vec![option_chip(Self::chip_id("author", "all"), tr("insights.filter.all_authors"), self.state.filter.authors.is_empty(), &self.theme, everyone)];
        for (id, name) in self.authors.iter().take(MAX_AUTHOR_CHIPS) {
            let key = id.clone();
            let pick = self.filter_pick(cx, move |f| f.toggle_author(&key));
            chips.push(option_chip(Self::chip_id("author", id), name.clone(), self.state.filter.authors.contains(id), &self.theme, pick));
        }
        option_group(tr("insights.filter.authors"), chips, &self.theme)
    }

    fn network_chips(&self, cx: &mut Context<Self>) -> Div {
        let chips = [false, true].into_iter().map(|all| {
            let pick = self.filter_pick(cx, move |f| f.all_networks = all);
            let key = if all { "all" } else { "active" };
            option_chip(Self::chip_id("network", key), tr(&format!("insights.network_scope.{key}")), self.state.filter.all_networks == all, &self.theme, pick)
        });
        option_group(tr("insights.filter.network"), chips.collect(), &self.theme)
    }

    pub(super) fn render_filters(&self, room: Room, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let row = || div().w_full().flex().flex_wrap().gap_x(theme.section_gap).gap_y(theme.spacing_sm);
        let first = row().child(self.range_chips(cx)).child(self.audience_chips(cx)).child(self.network_chips(cx));
        let second = row().child(self.kind_chips(cx)).child(self.author_chips(cx));
        let gap = if room == Room::Narrow { theme.spacing_sm } else { theme.stack_gap };
        card(theme).gap(gap).child(first).child(second)
    }
}
