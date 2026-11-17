//! The units: the chapter / paragraph / sentence switch, select all or none or one chapter,
//! and the virtualized preview list with a checkbox per unit.

use super::super::labels::granularity_label;
use super::ImportSheet;
use crate::import::Granularity;
use crate::ui::components::{choice_chip, segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const SPLIT_ID: &str = "import-split";
const CHAPTER_CHIP_ID: &str = "import-chapter";
const CHAPTERS_ROW_ID: &str = "import-chapters";
const ALL_ID: &str = "import-select-all";
const NONE_ID: &str = "import-select-none";

fn granularity_icon(granularity: Granularity) -> LucideIcon {
    match granularity {
        Granularity::Chapter => LucideIcon::BookOpen,
        Granularity::Paragraph => LucideIcon::Pilcrow,
        Granularity::Sentence => LucideIcon::MessageSquare,
    }
}

impl ImportSheet {
    fn split_switch(&self) -> Div {
        let theme = &self.theme;
        let pills = Granularity::ALL.into_iter().enumerate().map(|(index, granularity)| {
            let on_pick = self.actions.on_granularity.clone();
            let segment = Segment {
                id: (SPLIT_ID, index).into(),
                icon: granularity_icon(granularity),
                label: granularity_label(granularity),
                active: granularity == self.session.granularity,
                active_icon_color: theme.accent_brand,
            };
            segment_pill(segment, theme, move |window, cx| on_pick(granularity, window, cx))
        });
        segmented_track(theme).children(pills)
    }

    fn selection_bar(&self) -> Div {
        let theme = &self.theme;
        let (all, none) = (self.actions.on_select_all.clone(), self.actions.on_select_all.clone());
        let count = t!("import.selected", selected = self.session.selected_count(), total = self.session.units.len());
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.control_gap)
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(count.to_string()))
            .child(
                div()
                    .flex()
                    .gap(theme.control_gap)
                    .child(Button::new(ALL_ID).ghost().xsmall().label(t!("import.select_all").to_string()).on_click(move |_, w, cx| all(true, w, cx)))
                    .child(Button::new(NONE_ID).ghost().xsmall().label(t!("import.select_none").to_string()).on_click(move |_, w, cx| none(false, w, cx))),
            )
    }

    fn chapter_chips(&self) -> Option<Stateful<Div>> {
        let chapters = self.session.chapters_with_units();
        (chapters.len() > 1).then(|| {
            let titles = self.session.chapter_titles();
            let chips = chapters.into_iter().map(|chapter| {
                let label = titles.get(chapter).cloned().flatten().unwrap_or_else(|| t!("import.chapter_number", number = chapter + 1).to_string());
                let pick = self.actions.on_select_chapter.clone();
                choice_chip((CHAPTER_CHIP_ID, chapter), &label, false, &self.theme, move |w, cx| pick(chapter, w, cx)).flex_none()
            });
            div().id(CHAPTERS_ROW_ID).flex().gap(self.theme.spacing_xs).overflow_x_scroll().pb(self.theme.spacing_xs).children(chips)
        })
    }

    pub(super) fn units(&self) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.section_title("import.split_title"))
            .child(self.split_switch())
            .child(self.selection_bar())
            .children(self.chapter_chips())
            .child(self.preview())
    }
}
