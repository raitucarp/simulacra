//! Data ownership in the shell (sb F2): bookmarks and collection items and the edit counts
//! loaded from the vault, the open sheets (history, export, snapshot, restore), the
//! Knowledge page's Library / Saved switch and shelf, and the text fields those use.

use super::view::AppView;
use crate::domain::data::saved::{ItemRef, SavedShelf};
use crate::domain::PostId;
use crate::state::AppState;
use crate::storage::data::StagedRestore;
use crate::ui::features::data::{ExportDialog, HistorySheet, RestoreDialog, SavedTarget, SnapshotDialog};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

/// The text fields of the data views.
pub struct DataInputs {
    pub passphrase: Entity<InputState>,
    pub repeat: Entity<InputState>,
    pub restore_passphrase: Entity<InputState>,
    pub from: Entity<InputState>,
    pub to: Entity<InputState>,
    pub collection_name: Entity<InputState>,
    pub note: Entity<InputState>,
    /// The meaning of a custom symbol on the legend page (S9).
    pub meaning: Entity<InputState>,
    /// N2: the reply in the margin sheet.
    pub margin_reply: Entity<InputState>,
    /// N6: what a what-if branch supposes, and its first post.
    pub branch_label: Entity<InputState>,
    pub branch_text: Entity<InputState>,
}

/// Everything data ownership keeps in the shell.
pub struct DataHost {
    pub shelf: SavedShelf,
    /// Earlier versions per post id (the "edited" marks).
    pub revision_counts: HashMap<String, usize>,
    pub history: Option<HistorySheet>,
    pub export: Option<ExportDialog>,
    pub snapshot: Option<SnapshotDialog>,
    pub restore: Option<RestoreDialog>,
    /// The scheduled queue sheet is open (Y4).
    pub scheduled_open: bool,
    /// The legend page is open, and the custom symbol whose meaning is being edited (S9).
    pub legend_open: bool,
    pub editing_meaning: Option<crate::domain::symbols::SymbolRef>,
    /// The monthly migration review under way (S16).
    pub review: Option<crate::domain::symbols::MigrationReview<PostId>>,
    /// The verified snapshot waiting for the go-ahead.
    pub staged: Option<StagedRestore>,
    /// The Knowledge page shows the Saved view instead of the library.
    pub showing_saved: bool,
    pub target: SavedTarget,
    pub editing_note: Option<ItemRef>,
    /// The post "New collection…" was asked from, added once the collection exists.
    pub pending_add: Option<PostId>,
    pub inputs: DataInputs,
    /// The open sheet's keyboard focus (Escape closes it), sb P16.
    pub focus: FocusHandle,
    /// Locked posts: the session's unlocked texts and the unlock sheet (X4).
    pub locks: super::lock_host::LockHost,
    /// N2: the open margin sheet.
    pub margin: Option<crate::ui::features::data::MarginDialog>,
    /// N6: the open branch sheet.
    pub branch: Option<crate::ui::features::data::BranchDialog>,
    /// N5: the depth dial, and the ticket of the newest rendering request.
    pub depth: crate::ui::features::data::DepthView,
    pub depth_ticket: u64,
    _subscriptions: Vec<Subscription>,
}

fn field(placeholder: String, masked: bool, window: &mut Window, cx: &mut Context<AppView>) -> Entity<InputState> {
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder).masked(masked))
}

impl DataInputs {
    fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        Self {
            passphrase: field(t!("data.snapshot.passphrase_placeholder").to_string(), true, window, cx),
            repeat: field(t!("data.snapshot.repeat_placeholder").to_string(), true, window, cx),
            restore_passphrase: field(t!("data.snapshot.passphrase_placeholder").to_string(), true, window, cx),
            from: field(t!("data.export.date_placeholder").to_string(), false, window, cx),
            to: field(t!("data.export.date_placeholder").to_string(), false, window, cx),
            collection_name: field(t!("data.saved.name_placeholder").to_string(), false, window, cx),
            note: field(t!("data.saved.note_placeholder").to_string(), false, window, cx),
            meaning: field(t!("symbols.legend.meaning_placeholder").to_string(), false, window, cx),
            margin_reply: field(t!("post.margin.reply_placeholder").to_string(), false, window, cx),
            branch_label: field(t!("posts.branch.label_placeholder").to_string(), false, window, cx),
            branch_text: field(t!("posts.branch.first_post_placeholder").to_string(), false, window, cx),
        }
    }

    /// Whether one of the fields has the keyboard.
    pub fn any_focused(&self, window: &Window, cx: &App) -> bool {
        [&self.passphrase, &self.repeat, &self.restore_passphrase, &self.from, &self.to, &self.collection_name, &self.note, &self.meaning, &self.margin_reply, &self.branch_label, &self.branch_text].into_iter().any(|input| input.read(cx).focus_handle(cx).is_focused(window))
    }

    /// Re-renders on edits of the live fields; Enter in the name and note fields submits.
    fn subscribe(&self, window: &mut Window, cx: &mut Context<AppView>) -> Vec<Subscription> {
        let changes = [&self.passphrase, &self.repeat, &self.from, &self.to].map(|input| cx.subscribe_in(input, window, AppView::on_data_input_change));
        let enter = [(&self.collection_name, Some(true)), (&self.note, Some(false)), (&self.margin_reply, None)].map(|(input, is_name)| {
            cx.subscribe_in(input, window, move |this: &mut AppView, _, event: &InputEvent, window, cx| {
                if matches!(event, InputEvent::PressEnter { .. }) {
                    let next = match is_name { Some(true) => crate::ui::features::data::DataEvent::CreateCollection, Some(false) => crate::ui::features::data::DataEvent::SaveNote, None => crate::ui::features::data::DataEvent::SubmitMargin };
                    this.on_data_event(next, window, cx);
                }
            })
        });
        changes.into_iter().chain(enter).collect()
    }
}

impl DataHost {
    /// Loads bookmarks, collection items and edit counts, and creates the fields.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let vault = state.db.as_ref();
        let shelf = vault.and_then(|db| db.load_saved_shelf().map_err(|err| eprintln!("simulacra: saved items not loaded: {err}")).ok()).unwrap_or_default();
        let counts = vault.and_then(|db| db.post_revision_counts().ok()).unwrap_or_default();
        let inputs = DataInputs::new(window, cx);
        let subscriptions = inputs.subscribe(window, cx);
        Self {
            shelf,
            revision_counts: counts.into_iter().collect(),
            history: None,
            export: None,
            snapshot: None,
            restore: None,
            scheduled_open: false, legend_open: false, editing_meaning: None, review: None,
            staged: None,
            showing_saved: false,
            target: SavedTarget::Bookmarks,
            editing_note: None,
            pending_add: None,
            inputs,
            focus: cx.focus_handle(),
            locks: super::lock_host::LockHost::new(window, cx),
            margin: None,
            branch: None,
            depth: Default::default(),
            depth_ticket: 0,
            _subscriptions: subscriptions,
        }
    }
}
