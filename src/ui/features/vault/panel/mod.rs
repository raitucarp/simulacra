//! The Knowledge Vault view entity: owns navigation, the catalogue query and its results,
//! the facet and subject-tree state, the reader's scroll and the editor. Changes to the
//! store go out as [`VaultEvent`]s; the shell applies them and hands back fresh
//! [`VaultData`].

mod actions;
mod editing;
mod events;
mod search;

pub use events::{EditorSubmission, VaultEvent};

use super::data::VaultData;
use super::editor::fields::TextField;
use super::editor::EditorState;
use super::screen::{BrowseLayout, Navigation};
use crate::domain::document::citation::CitationStyle;
use crate::domain::document::facets::Facet;
use crate::domain::document::query::CatalogueQuery;
use crate::domain::document::SubjectId;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, EventEmitter, ScrollHandle, Subscription, Window};
use rust_i18n::t;
use std::collections::HashSet;

/// The Knowledge Vault screen.
pub struct VaultPanel {
    pub(super) theme: ThemeTokens,
    pub(super) data: VaultData,
    pub(super) nav: Navigation,
    pub(super) layout: BrowseLayout,
    pub(super) query: CatalogueQuery,
    /// Ids of the results, in order.
    pub(super) results: Vec<String>,
    /// Ids matching the search text alone (the base of the facet counts).
    pub(super) text_matches: Option<HashSet<String>>,
    pub(super) expanded_subjects: HashSet<SubjectId>,
    pub(super) collapsed_facets: HashSet<Facet>,
    pub(super) drawer_open: bool,
    pub(super) search_input: Entity<InputState>,
    pub(super) editor: EditorState,
    pub(super) reader_scroll: ScrollHandle,
    /// Highest reading progress seen in the open reader (percent).
    pub(super) reader_progress: u8,
    /// Text block the reader scrolls to on its next frame (from the contents).
    pub(super) pending_block: Option<usize>,
    pub(super) copied_style: Option<CitationStyle>,
    /// Replace motion with instant changes.
    pub(super) reduced_motion: bool,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<VaultEvent> for VaultPanel {}

impl VaultPanel {
    pub fn new(data: VaultData, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("vault.search_placeholder").to_string();
        let search_input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let subscription = cx.subscribe_in(&search_input, window, |this: &mut Self, input, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                let text = input.read(cx).value().to_string();
                this.set_search_text(text, cx);
            }
        });
        let editor = EditorState::new(window, cx);
        let list_inputs = [TextField::Subjects, TextField::Collections].map(|field| editor.input(field).clone());
        let mut subscriptions = vec![subscription];
        subscriptions.extend(list_inputs.iter().map(|input| cx.subscribe(input, |_: &mut Self, _, _: &InputEvent, cx| cx.notify())));
        let query = CatalogueQuery::new(data.network_id.clone());
        let mut panel = Self {
            theme,
            data,
            nav: Navigation::home(),
            layout: BrowseLayout::default(),
            query,
            results: Vec::new(),
            text_matches: None,
            expanded_subjects: HashSet::new(),
            collapsed_facets: HashSet::new(),
            drawer_open: false,
            search_input,
            editor,
            reader_scroll: ScrollHandle::new(),
            reader_progress: 0,
            pending_block: None,
            copied_style: None,
            reduced_motion: false,
            _subscriptions: subscriptions,
        };
        panel.refresh_results();
        panel
    }

    /// Swaps in fresh store data (and theme); a different network starts browsing afresh.
    pub fn set_data(&mut self, data: VaultData, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if data.network_id != self.data.network_id {
            self.query = CatalogueQuery::new(data.network_id.clone());
            self.nav = Navigation::home();
        }
        self.data = data;
        if &self.theme != theme {
            self.theme = theme.clone();
        }
        let missing: Vec<String> = self.open_documents().into_iter().filter(|id| self.data.document(id).is_none()).collect();
        for id in missing {
            self.nav = std::mem::take(&mut self.nav).without_document(&id);
        }
        self.refresh_results();
        cx.notify();
    }

    /// Adopts a changed theme.
    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Replaces motion with instant changes when `reduced` (the appearance preference).
    pub fn set_reduced_motion(&mut self, reduced: bool, cx: &mut Context<Self>) {
        if self.reduced_motion != reduced {
            self.reduced_motion = reduced;
            cx.notify();
        }
    }
}
