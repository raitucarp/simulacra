//! "Lately": what the last two weeks of analysed posts mostly came from, the stances heard
//! less in that time with agents who hold them (a chip opens the agent), and earlier posts
//! furthest in meaning from them (a row opens the post). Plain statements, no advice.

use super::super::super::common::{card, card_title, option_chip, option_group, tr};
use crate::domain::insights::lenses::LensReport;
use crate::ui::features::insights::chart::summary::summary_line;
use crate::ui::features::insights::chart::ticks::percent;
use crate::ui::features::insights::events::InsightsEvent;
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

impl InsightsPanel {
    pub(super) fn lately_card(&self, report: &Rc<LensReport>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let title = card_title(LucideIcon::Clock, tr("insight.lately.title"), None, theme);
        let Some(lately) = report.diversity.lately.as_ref() else {
            return card(theme).child(title).child(summary_line(tr("insight.lately.none"), theme));
        };
        let mostly = lately.top_author.as_ref().map(|(name, share)| t!("insight.lately.mostly", posts = super::super::posts_label(lately.posts), days = lately.days, share = percent(*share), name = name).to_string());
        let stance = lately.top_stance.as_ref().map(|(key, share)| t!("insight.lately.stance", share = percent(*share), stance = tr(key)).to_string());
        let quieter = (!lately.quieter.is_empty()).then(|| t!("insight.lately.quieter", stances = lately.quieter.iter().map(|k| tr(k)).collect::<Vec<_>>().join(", ")).to_string());
        let chips = lately.agents.iter().enumerate().map(|(i, agent)| {
            let (id, entity) = (agent.id.clone(), cx.entity().downgrade());
            option_chip(ElementId::NamedInteger("insight-lately-agent".into(), i as u64), format!("{} · {}", agent.name, tr(&agent.stance)), false, theme, Rc::new(move |_, cx| {
                let id = id.clone();
                let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenAuthor(id)));
            }))
        }).collect::<Vec<_>>();
        let further = lately.further.iter().filter_map(|&i| report.posts.get(i).map(|p| (i, p))).map(|(i, post)| self.further_row(i, post.id.clone(), post.excerpt.clone(), cx));
        let lines = [mostly, stance, quieter].into_iter().flatten().map(|line| summary_line(line, theme));
        let mut body = card(theme).child(title).children(lines);
        if !chips.is_empty() {
            body = body.child(option_group(tr("insight.lately.agents"), chips, theme));
        }
        if !lately.further.is_empty() {
            body = body.child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(tr("insight.lately.further"))).children(further);
        }
        body
    }

    fn further_row(&self, i: usize, id: crate::domain::PostId, excerpt: String, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let entity = cx.entity().downgrade();
        div()
            .id(ElementId::NamedInteger("insight-further".into(), i as u64))
            .flex()
            .items_start()
            .gap(theme.inline_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .cursor_pointer()
            .hover(|s| s.bg(theme.hover_bg))
            .on_click(move |_, _, cx| {
                let id = id.clone();
                let _ = entity.update(cx, |_, cx| cx.emit(InsightsEvent::OpenPost(id)));
            })
            .child(Icon::new(LucideIcon::Compass).size(theme.icon_size_xs).text_color(theme.text_muted).flex_none())
            .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(excerpt))
    }
}
