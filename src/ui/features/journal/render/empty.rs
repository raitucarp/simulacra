//! Illustrated empty states of the Journal tabs (`ArtState`), each with the way to begin.

use super::super::panel::JournalPanel;
use super::super::text::tr;
use super::common::{primary_button, surface};
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::*;

const JOURNAL_START_ID: &str = "journal-empty-start";
const CAPSULE_START_ID: &str = "journal-empty-capsule";

impl JournalPanel {
    fn empty_card(&self, key: EmptyKey, title: &str, hint: &str) -> Div {
        let theme = &self.theme;
        let art = Illustration::empty(key).width(theme.art_empty_width);
        surface(theme).items_center().py(theme.section_gap).child(art_state(ArtState::titled(art, tr(title), tr(hint)), theme))
    }

    pub(super) fn journal_empty(&self, cx: &mut Context<Self>) -> Div {
        let start = primary_button(JOURNAL_START_ID, LucideIcon::Feather, tr("journal.today.write")).on_click(cx.listener(|this, _, window, cx| this.new_entry(None, true, window, cx)));
        self.empty_card(EmptyKey::JournalEmpty, "journal.empty.title", "journal.empty.hint").child(start)
    }

    pub(super) fn mood_empty(&self) -> Div {
        self.empty_card(EmptyKey::MoodEmpty, "journal.mood.empty_title", "journal.mood.empty_hint")
    }

    pub(super) fn focus_empty(&self) -> Div {
        self.empty_card(EmptyKey::FocusEmpty, "journal.focus.empty_title", "journal.focus.empty_hint")
    }

    pub(super) fn capsules_empty(&self, cx: &mut Context<Self>) -> Div {
        let start = primary_button(CAPSULE_START_ID, LucideIcon::Mail, tr("journal.capsule.write")).on_click(cx.listener(|this, _, window, cx| this.write_capsule(window, cx)));
        self.empty_card(EmptyKey::CapsulesEmpty, "journal.capsule.empty_title", "journal.capsule.empty_hint").child(start)
    }
}
