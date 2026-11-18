//! The page header: the notebook art, the title, a gentle line about today, the privacy
//! note (everything here stays on this page), and the tabs.

use super::super::panel::JournalPanel;
use super::super::tab::JournalTab;
use super::super::text::{day_label, tr};
use super::common::caption;
use crate::domain::journal::streak::streaks;
use crate::i18n::t_count;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, SpotKey};
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const TAB_ID_PREFIX: &str = "journal-tab-";

impl JournalPanel {
    pub(super) fn render_header(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let streak = streaks(self.data.book.written_days(), self.data.today);
        let subtitle = match streak.current {
            0 => day_label(self.data.today),
            days => format!("{}{}{}", day_label(self.data.today), crate::i18n::COUNT_SEPARATOR, t_count("journal.streak.current", days)),
        };
        let titles = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("journal.title")))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(subtitle))
            .child(self.privacy_note());
        let art = Illustration::spot(SpotKey::Journal).width(theme.art_spot_width);
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().flex().items_center().gap(theme.section_gap).when(!class.is_compact(), |row| row.child(art)).child(titles))
            .child(self.render_tabs(class, cx))
    }

    /// "Private: only you see this page…", with a lock.
    fn privacy_note(&self) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .flex()
            .items_start()
            .gap(theme.inline_gap)
            .child(Icon::new(LucideIcon::Lock).size(theme.icon_size_xs).text_color(theme.accent_success))
            .child(caption(tr("journal.privacy_note"), theme).flex_1().min_w_0())
    }

    fn render_tabs(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = JournalTab::ALL.into_iter().map(|tab| {
            let label = if class.is_compact() { String::new() } else { tr(&tab.label_key()) };
            let segment = Segment {
                id: ElementId::Name(format!("{TAB_ID_PREFIX}{}", tab.key()).into()),
                icon: tab.icon(),
                label,
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
