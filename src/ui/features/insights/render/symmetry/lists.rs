//! The Symmetry view's lists: threads to draw (by size, with their group order) and the
//! classes of structurally equivalent posts (excerpts open the post).

use super::super::common::{card, card_title, tr};
use super::group_order;
use crate::domain::insights::InsightsReport;
use crate::ui::features::forge::widgets::KeyboardStop;
use crate::ui::features::insights::chart::palette::series;
use crate::ui::features::insights::chart::ranked::{ranked_list, RankedRow};
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Classes and excerpts per class listed.
const CLASSES_SHOWN: usize = 6;
const EXCERPTS_PER_CLASS: usize = 3;

impl InsightsPanel {
    pub(super) fn thread_list(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows: Vec<RankedRow> = report
            .symmetry
            .threads
            .iter()
            .enumerate()
            .map(|(i, thread)| {
                let root = thread.posts.first().and_then(|&p| report.posts.get(p)).map(|p| p.excerpt.clone()).unwrap_or_default();
                RankedRow { label: root, value: thread.posts.len() as f64, value_text: group_order(thread), colour: series(i, theme), active: i == self.state.thread }
            })
            .collect();
        let entity = cx.entity().downgrade();
        let pick = Rc::new(move |i: usize, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.update_state(cx, |s| s.thread = i));
        });
        card(theme)
            .child(card_title(LucideIcon::ListTree, tr("insights.card.threads"), None, theme))
            .child(ranked_list("insights-threads", &rows, Some(pick), theme))
            .child(summary_line(tr("insights.summary.threads"), theme))
    }

    fn excerpt_link(&self, post: usize, report: &InsightsReport, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let text = report.posts.get(post).map(|p| p.excerpt.clone()).unwrap_or_default();
        let entity = cx.entity().downgrade();
        let open: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |_, cx| {
            let _ = entity.update(cx, |panel, cx| panel.open_post(post, cx));
        });
        let click = Rc::clone(&open);
        div()
            .id(ElementId::NamedInteger("insights-equivalent".into(), post as u64))
            .truncate()
            .px(theme.spacing_xs)
            .rounded(theme.radius_sm)
            .text_size(theme.font_size_xs)
            .text_color(theme.text_secondary)
            .cursor_pointer()
            .hover(|d| d.bg(theme.hover_bg))
            .on_click(move |_, window, cx| click(window, cx))
            .keyboard_stop(theme, open)
            .child(text)
    }

    pub(super) fn equivalence_card(&self, report: &Rc<InsightsReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let classes = report.symmetry.classes.iter().take(CLASSES_SHOWN).enumerate().map(|(i, class)| {
            let heading = div().flex().items_center().gap(theme.inline_gap).child(div().size(theme.insights_swatch_size).rounded(theme.radius_full).bg(series(i, theme))).child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(crate::i18n::t_count("insights.posts", class.len())));
            let links: Vec<Stateful<Div>> = class.iter().take(EXCERPTS_PER_CLASS).map(|&p| self.excerpt_link(p, report, cx)).collect();
            div().flex().flex_col().gap(theme.spacing_3xs).child(heading).children(links)
        });
        let summary = t!("insights.summary.equivalence", classes = report.symmetry.classes.len(), connected = report.symmetry.connected).to_string();
        card(theme)
            .child(card_title(LucideIcon::Puzzle, tr("insights.card.equivalence"), None, theme))
            .children(classes.collect::<Vec<_>>())
            .child(summary_line(summary, theme))
            .child(summary_line(tr("insights.explain.refinement"), theme))
    }
}
