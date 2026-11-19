//! The page header: the debate art, the title, one line on what happens here (and that every
//! request counts toward today's AI budget), and the tabs.

use super::super::kit::caption;
use super::super::panel::ArenaPanel;
use super::super::tab::ArenaTab;
use super::super::text::tr;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::illustration::{Illustration, SpotKey};
use crate::ui::responsive::LayoutClass;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const TAB_ID_PREFIX: &str = "arena-tab-";

impl ArenaPanel {
    pub(super) fn render_header(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let titles = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("arena.title")))
            .child(caption(tr("arena.subtitle"), theme));
        let art = Illustration::spot(SpotKey::Debate).width(theme.art_spot_width);
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().flex().items_center().gap(theme.section_gap).when(!class.is_compact(), |row| row.child(art)).child(titles))
            .child(self.render_tabs(cx))
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = ArenaTab::ALL.into_iter().map(|tab| {
            let segment = Segment {
                id: ElementId::Name(format!("{TAB_ID_PREFIX}{}", tab.key()).into()),
                icon: tab.icon(),
                label: tr(&tab.label_key()),
                active: self.tab == tab,
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.show(tab, cx));
            })
        });
        segmented_track(theme).children(pills)
    }
}
