//! Journal requests (sb E3): opening the page or one of its quick starts, the data the page
//! is built from (the records, activity per day, wikilink previews, focus links), and the
//! palette's journal entries (only when Settings › Privacy & Data lets search include them).

use super::super::view::AppView;
use crate::domain::journal::activity::{activity_by_day, review_sessions_by_day, ActivityByDay};
use crate::domain::journal::focus::stats::minutes_by_day;
use crate::domain::journal::focus::FocusLink;
use crate::domain::journal::journal_day;
use crate::domain::journal::privacy::{palette_entries, JournalPrivacy};
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::state::{NavigationTab, WikilinkTarget};
use crate::ui::features::command_palette::model::{PaletteGroup, PaletteItem};
use crate::ui::features::command_palette::{excerpt, CommandId, PaletteTarget};
use crate::ui::features::journal::data::LinkChoice;
use crate::ui::features::journal::{JournalData, JournalTab, WikilinkPreviews};
use crate::ui::icons::LucideIcon;
use chrono::{Local, NaiveDate};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::BTreeMap;
use std::rc::Rc;

/// Vault documents offered as focus links, and characters of a link or palette label.
const LINKED_DOCUMENTS: usize = 3;
const LABEL_CHARS: usize = 48;
const KEYWORD_CHARS: usize = 280;

impl AppView {
    /// Opens the page, then the quick start a command asks for.
    pub(in crate::ui::features::shell) fn open_journal(&mut self, command: CommandId, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Journal, window, cx);
        self.journal.panel.update(cx, |panel, cx| match command {
            CommandId::NewJournalEntry => panel.new_entry(None, true, window, cx),
            CommandId::LogMood => panel.show(JournalTab::Mood, cx),
            CommandId::StartFocus => panel.show(JournalTab::Focus, cx),
            CommandId::NewTimeCapsule => panel.write_capsule(window, cx),
            _ => {}
        });
    }

    /// Opens a journal entry the palette found.
    pub(in crate::ui::features::shell) fn open_journal_entry(&mut self, id: String, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Journal, window, cx);
        let id = crate::domain::journal::RecordId::new(id);
        self.journal.panel.update(cx, |panel, cx| panel.open_entry(&id, window, cx));
    }

    pub(in crate::ui::features::shell) fn journal_data(&self, today: NaiveDate) -> JournalData {
        JournalData {
            book: Rc::new(self.journal.book.clone()),
            today,
            activity: Rc::new(self.journal_activity()),
            wikilinks: Rc::new(self.journal_wikilinks()),
            link_choices: self.journal_link_choices(),
            reduced_motion: self.state.preferences().appearance.reduced_motion,
            media_dir: self.state.media_dir(),
        }
    }

    /// Posts the user wrote (any persona), review sessions and focus minutes per journal day.
    fn journal_activity(&self) -> ActivityByDay {
        let state = &self.state;
        let posts = state.posts.iter().filter(|p| state.author_of(p).is_human_persona()).fold(BTreeMap::new(), |mut days: BTreeMap<NaiveDate, usize>, post| {
            *days.entry(journal_day(&post.created_at.with_timezone(&Local))).or_default() += 1;
            days
        });
        let answers: Vec<_> = self.review.deck.log.iter().map(|entry| (entry.day, entry.reviewed_at)).collect();
        activity_by_day(&posts, &review_sessions_by_day(&answers), &minutes_by_day(&self.journal.book.sessions))
    }

    /// What every `[[target]]` in the entries names: a post's excerpt or a document's title.
    fn journal_wikilinks(&self) -> WikilinkPreviews {
        let targets = self.journal.book.entries.iter().flat_map(|e| tokenize_post_body(&e.body)).filter_map(|token| match token {
            BodyToken::Wikilink { target, .. } => Some(target),
            _ => None,
        });
        let preview = |target: &str| match self.state.resolve_wikilink_target(target) {
            Some(WikilinkTarget::Post(id)) => self.state.resolve_wikilink_fast(id.as_str()),
            Some(WikilinkTarget::Document(id)) => self.state.documents.iter().find(|d| d.id == id).map(|d| d.title.clone()).unwrap_or(id),
            None => target.to_string(),
        };
        WikilinkPreviews(targets.map(|t| (t.trim().to_string(), preview(&t))).collect())
    }

    /// The open thought and a few vault documents, as focus links.
    fn journal_link_choices(&self) -> Vec<LinkChoice> {
        let thought = self.feed.selected_thread_post_id.as_ref().map(|id| LinkChoice {
            link: FocusLink::Post(id.as_str().to_string()),
            label: t!("journal.focus.link_thought", excerpt = excerpt(&self.state.resolve_wikilink_fast(id.as_str()), LABEL_CHARS)).to_string(),
        });
        let documents = self.state.current_documents().take(LINKED_DOCUMENTS).map(|d| LinkChoice { link: FocusLink::Document(d.id.clone()), label: excerpt(&d.title, LABEL_CHARS) });
        thought.into_iter().chain(documents).collect()
    }

    /// Journal entries for the palette's word search, none unless the user opted in.
    pub(in crate::ui::features::shell) fn journal_palette_items(&self) -> Vec<PaletteItem> {
        let privacy = JournalPrivacy { include_in_search: self.state.preferences().privacy.journal_in_search };
        palette_entries(&self.journal.book.entries, privacy)
            .into_iter()
            .map(|entry| PaletteItem {
                target: PaletteTarget::JournalEntry(entry.id.as_str().to_string()),
                group: PaletteGroup::Journal,
                title: excerpt(&entry.title(), LABEL_CHARS),
                detail: crate::ui::features::journal::text::day_label(entry.day),
                keywords: excerpt(&entry.body, KEYWORD_CHARS),
                icon: LucideIcon::NotebookPen,
                shortcut: None,
            })
            .collect()
    }
}
