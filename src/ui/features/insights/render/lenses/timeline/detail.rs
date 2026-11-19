//! The picked concept: when it first appeared, its peak and its last mention (or since when it
//! has been quiet), its mentions in order (posts and documents open where they live), and
//! per period the concepts mentioned alongside it (a chip picks that concept instead).

use super::super::super::common::{card, card_title, option_chip, tr};
use super::{period_label, phase_label};
use crate::domain::graph::NodeRef;
use crate::domain::insights::lenses::sources::SourceKind;
use crate::domain::insights::lenses::timeline::{ConceptTimeline, Mention, Phase};
use crate::ui::features::day_filter::format_day;
use crate::ui::features::insights::chart::lanes::PeriodRange;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Mentions listed at most, and concepts named alongside per period.
const MENTIONS_SHOWN: usize = 40;
const ALONGSIDE: usize = 3;

fn kind_icon(kind: SourceKind) -> LucideIcon {
    match kind {
        SourceKind::Post => LucideIcon::MessageSquare,
        SourceKind::Document => LucideIcon::BookOpen,
        SourceKind::Review => LucideIcon::GraduationCap,
    }
}

impl InsightsPanel {
    pub(super) fn concept_detail_card(&self, timeline: &ConceptTimeline, range: PeriodRange, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let Some(lane) = self.lenses.timeline.selected.and_then(|lane| timeline.lanes.get(lane).map(|l| (lane, l))) else {
            return card(theme).child(card_title(LucideIcon::Target, tr("insight.timeline.detail"), None, theme)).child(summary_line(tr("insight.timeline.pick_hint"), theme));
        };
        let (index, concept) = lane;
        let last = if concept.phase == Phase::Dormant { t!("insight.timeline.quiet_since", day = format_day(concept.last_day)) } else { t!("insight.timeline.last_seen", day = format_day(concept.last_day)) };
        let facts = t!("insight.timeline.facts", first = format_day(concept.first_day), peak = period_label(timeline.periods.start_of(concept.peak), timeline.periods.days), last = last).to_string();
        let mentions: Vec<&Mention> = timeline.mentions_of(index, range);
        let heading = card(theme).child(card_title(LucideIcon::Target, format!("{} · {}", concept.key, phase_label(concept.phase)), None, theme)).child(summary_line(facts, theme));
        if mentions.is_empty() {
            return heading.child(summary_line(tr("insight.timeline.none_in_range"), theme));
        }
        let list = div().id("insight-mentions").w_full().max_h(theme.insight_list_max_height).overflow_y_scroll().flex().flex_col().gap(theme.spacing_3xs).children(mentions.iter().take(MENTIONS_SHOWN).enumerate().map(|(i, m)| self.mention_row(i, m, cx)));
        heading
            .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(crate::i18n::t_count("insight.mentions", mentions.len())))
            .child(list)
            .child(self.alongside(timeline, index, range, cx))
    }

    fn mention_row(&self, i: usize, mention: &Mention, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let target = match mention.kind {
            SourceKind::Post => Some(InsightsEvent::OpenPost(mention.id.as_str().into())),
            SourceKind::Document => Some(InsightsEvent::OpenNode(NodeRef::Document(mention.id.clone()))),
            SourceKind::Review => None,
        };
        let row = div()
            .id(ElementId::NamedInteger("insight-mention".into(), i as u64))
            .flex()
            .items_start()
            .gap(theme.inline_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .child(Icon::new(kind_icon(mention.kind)).size(theme.icon_size_xs).text_color(theme.text_muted).flex_none())
            .child(div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(format_day(mention.day)))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(mention.excerpt.clone()));
        let entity = cx.entity().downgrade();
        row.when_some(target, |row, event| {
            row.cursor_pointer().hover(|s| s.bg(theme.hover_bg)).on_click(move |_, _, cx| {
                let event = event.clone();
                let _ = entity.update(cx, |_, cx| cx.emit(event));
            })
        })
    }

    /// Per period, the concepts most mentioned alongside the picked one.
    fn alongside(&self, timeline: &ConceptTimeline, lane: usize, range: PeriodRange, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let neighbours: Vec<_> = timeline.co_occurring(lane, range, ALONGSIDE).into_iter().filter(|p| !p.with.is_empty()).collect();
        if neighbours.is_empty() {
            return div();
        }
        let periods = neighbours.into_iter().map(|p| {
            let chips = p.with.iter().map(|&(other, count)| {
                let entity = cx.entity().downgrade();
                let label = format!("{} · {count}", timeline.lanes[other].key);
                option_chip(ElementId::NamedInteger("insight-alongside".into(), (p.period * timeline.lanes.len() + other) as u64), label, false, theme, Rc::new(move |_, cx| {
                    let _ = entity.update(cx, |panel, cx| panel.toggle_lane(other, cx));
                }))
            });
            let when = div().w(theme.insights_rank_value_width).flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(period_label(timeline.periods.start_of(p.period), timeline.periods.days));
            div().flex().flex_wrap().items_center().gap(theme.spacing_xs).child(when).children(chips)
        });
        div().w_full().flex().flex_col().gap(theme.spacing_xs).child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(tr("insight.timeline.alongside"))).children(periods)
    }
}
