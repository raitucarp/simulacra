//! The Focus tab: the setup (intention, length, link), the running timer, or the reflection
//! of the session that just ended; then the log and its statistics.

use super::super::panel::JournalPanel;
use super::super::text::tr;
use super::common::{block_title, caption, control_row, labelled, line_field, primary_button, surface};
use crate::domain::journal::focus::FocusLink;
use crate::ui::components::choice_chip;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::Disableable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const PRESET_ID: &str = "journal-focus-preset";
const CUSTOM_ID: &str = "journal-focus-custom";
const LINK_ID: &str = "journal-focus-link";
const START_ID: &str = "journal-focus-start";

impl JournalPanel {
    pub(super) fn render_focus(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let top = match (&self.focus_run.timer, &self.focus_run.finished) {
            (Some(_), _) => self.render_running(class, cx),
            (None, Some(_)) => self.render_reflection(cx),
            (None, None) => self.render_focus_setup(cx),
        };
        let below = if self.data.book.sessions.is_empty() { self.focus_empty() } else { self.render_focus_log(cx) };
        div().w_full().flex().flex_col().gap(theme.section_gap).child(top).child(below)
    }

    fn render_focus_setup(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let presets = Self::presets().into_iter().enumerate().map(|(i, minutes)| {
            let label = t!("journal.minutes", count = minutes).to_string();
            choice_chip((PRESET_ID, i), &label, self.focus_run.preset == Some(minutes), theme, self.listen(move |this, cx| this.choose_preset(Some(minutes), cx)))
        });
        let custom = choice_chip(CUSTOM_ID, &tr("journal.focus.custom"), self.focus_run.preset.is_none(), theme, self.listen(|this, cx| this.choose_preset(None, cx)));
        let lengths = control_row(theme).children(presets).child(custom).when(self.focus_run.preset.is_none(), |row| row.child(div().w(theme.journal_row_label_width).child(line_field(&self.fields.minutes, theme))));
        let start = primary_button(START_ID, LucideIcon::Play, tr("journal.focus.start")).disabled(self.chosen_minutes(cx).is_none()).on_click(cx.listener(|this, _, _, cx| this.start_focus(cx)));
        surface(theme)
            .child(block_title(LucideIcon::Target, theme.accent_success, tr("journal.focus.title"), theme))
            .child(labelled(tr("journal.focus.intention"), line_field(&self.fields.intention, theme), theme))
            .child(labelled(tr("journal.focus.length"), lengths, theme))
            .child(labelled(tr("journal.focus.link"), self.link_chips(), theme))
            .when(matches!(self.focus_run.link, Some(FocusLink::Goal(_))), |card| card.child(labelled(tr("journal.focus.goal"), line_field(&self.fields.goal, theme), theme)))
            .child(caption(tr("journal.focus.hint"), theme))
            .child(control_row(theme).child(start))
    }

    /// None, the review session, a goal, and what the shell offered (the open thought, documents).
    fn link_chips(&self) -> Div {
        let theme = &self.theme;
        let fixed = [(None, tr("journal.focus.link_none")), (Some(FocusLink::Review), tr("journal.focus.link_review")), (Some(FocusLink::Goal(String::new())), tr("journal.focus.link_goal"))];
        let offered = self.data.link_choices.iter().map(|choice| (Some(choice.link.clone()), choice.label.clone()));
        let chips = fixed.into_iter().chain(offered).enumerate().map(|(i, (link, label))| {
            let active = match (&link, &self.focus_run.link) {
                (Some(FocusLink::Goal(_)), Some(FocusLink::Goal(_))) => true,
                (a, b) => a == b,
            };
            choice_chip((LINK_ID, i), &label, active, theme, self.listen(move |this, cx| this.choose_link(link.clone(), cx)))
        });
        control_row(theme).children(chips)
    }

    /// A click handler that updates this panel.
    pub(super) fn listen(&self, f: impl Fn(&mut Self, &mut Context<Self>) + 'static) -> impl Fn(&mut Window, &mut App) + 'static {
        let entity = self.weak.clone();
        move |_, cx| {
            let _ = entity.update(cx, |this, cx| f(this, cx));
        }
    }
}
