//! The page header: title, today's counts, the tabs, and the ways to add material: pick
//! posts to learn, or a new connection map.

use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::{ReviewPanel, Screen};
use super::super::text::tr;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const TAB_ID_PREFIX: &str = "review-tab-";
const PICK_POSTS_ID: &str = "review-pick-posts";
const NEW_MAP_ID: &str = "review-new-map";

fn tab_icon(screen: Screen) -> LucideIcon {
    match screen {
        Screen::Stats => LucideIcon::ChartColumn,
        Screen::Items | Screen::Picker | Screen::MapEditor => LucideIcon::Layers,
        _ => LucideIcon::Calendar,
    }
}

impl ReviewPanel {
    pub(super) fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let plan = self.data.plan();
        let subtitle = t!("review.header.subtitle", due = plan.queued_due, new = plan.queued_new, total = self.data.live_items().len()).to_string();
        let titles = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("review.title")))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(subtitle));
        let pick = Button::new(PICK_POSTS_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::GraduationCap))
            .label(tr("review.pick_posts"))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(ReviewEvent::OpenPicker(PickerMode::Learn))));
        let map = Button::new(NEW_MAP_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Share2))
            .label(tr("review.new_map"))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(ReviewEvent::OpenPicker(PickerMode::Map))));
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().flex().flex_wrap().items_center().gap(theme.control_gap).child(titles).child(pick).child(map))
            .child(div().max_w(theme.review_card_max_width).child(self.render_tabs(cx)))
    }

    fn render_tabs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let active = self.screen.key();
        let pills = Screen::TABS.into_iter().map(|screen| {
            let segment = Segment {
                id: ElementId::Name(format!("{TAB_ID_PREFIX}{}", screen.key()).into()),
                icon: tab_icon(screen),
                label: tr(&format!("review.tab.{}", screen.key())),
                active: active == screen.key(),
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.open_tab(screen, cx));
            })
        });
        segmented_track(theme).children(pills)
    }

    /// A tab click: "Today" returns to a running session rather than abandoning it (its
    /// "End" button ends it).
    fn open_tab(&mut self, screen: Screen, cx: &mut Context<Self>) {
        let resume = if screen == Screen::Today && !self.session.is_finished() { Screen::Session } else { screen };
        self.show(resume, cx);
    }
}
