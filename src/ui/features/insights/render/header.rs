//! The page header: title, what is being analysed, an "updating" note while a new report is
//! computed, and the view tabs (icons only on narrow pages).

use super::super::panel::InsightsPanel;
use super::super::view::InsightsView;
use super::common::tr;
use super::Room;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::forge::widgets::KeyboardStop;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const TAB_ID_PREFIX: &str = "insights-tab-";

impl InsightsPanel {
    pub(super) fn render_header(&self, room: Room, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let subtitle = match &self.report {
            Some(report) => t!("insights.header.subtitle", posts = report.posts.len(), authors = report.authors.len(), documents = report.documents).to_string(),
            None => tr("insights.loading.title"),
        };
        let titles = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("insights.title")))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(subtitle));
        let updating = self.pending && self.report.is_some();
        let badge = div()
            .px(theme.spacing_sm)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_full)
            .bg(theme.accent_brand_subtle)
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_secondary)
            .child(tr("insights.updating"));
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().flex().flex_wrap().items_center().gap(theme.control_gap).child(titles).when(updating, |row| row.child(badge)))
            .child(self.render_tabs(room, cx))
    }

    fn render_tabs(&self, room: Room, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = InsightsView::ALL.into_iter().map(|view| {
            let label = if room == Room::Narrow { String::new() } else { tr(&format!("insights.view.{}", view.key())) };
            let segment = Segment {
                id: ElementId::Name(format!("{TAB_ID_PREFIX}{}", view.key()).into()),
                icon: view.icon(),
                label,
                active: self.state.view == view,
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            let open: Rc<dyn Fn(&mut Window, &mut App)> = Rc::new(move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.show(view, cx));
            });
            let click = Rc::clone(&open);
            let tip = tr(&format!("insights.view.{}", view.key()));
            segment_pill(segment, theme, move |window, cx| click(window, cx))
                .tooltip(move |window, cx| gpui_kit::component::tooltip::Tooltip::new(tip.clone()).build(window, cx))
                .keyboard_stop(theme, open)
        });
        segmented_track(theme).flex_wrap().children(pills) // sb F3: twelve views wrap onto a second row when they do not fit
    }
}
