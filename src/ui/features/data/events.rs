//! Everything the data views ask the shell to do.

use crate::domain::data::saved::ItemRef;
use crate::domain::document::CollectionId;
use crate::domain::post::auto_delete::DeleteAfter;
use crate::domain::preferences::ArchiveAfter;
use crate::domain::{PostId, ProfileId};
use gpui_kit::{App, Window};
use std::rc::Rc;

/// Which shelf the Saved view shows.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum SavedTarget {
    #[default]
    Bookmarks,
    Collection(CollectionId),
}

/// The export dialog's scope choice (the details come with their own events).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    Everything,
    Collection,
    DateRange,
    Persona,
}

impl ScopeKind {
    pub const ALL: [Self; 4] = [Self::Everything, Self::Collection, Self::DateRange, Self::Persona];

    /// Stable key (locale keys `data.export.scope.<key>`, element ids).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Everything => "everything",
            Self::Collection => "collection",
            Self::DateRange => "date_range",
            Self::Persona => "persona",
        }
    }
}

/// One interaction with a data view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataEvent {
    /// Post "…" menu.
    EditPost(PostId),
    /// Archives the post (`true`) or brings it back to the feed (Y5).
    SetArchived(PostId, bool),
    /// Archives the post this long after it went out (Y6).
    SetAutoArchive(PostId, ArchiveAfter),
    /// Deletes the post this long from now (`None` keeps it, Y7).
    SetAutoDelete(PostId, Option<DeleteAfter>),
    OpenPostHistory(PostId),
    ToggleBookmark(PostId),
    AddToCollection(PostId, CollectionId),
    RemoveFromCollection(PostId, CollectionId),
    /// "New collection…": the Saved view, ready to name it, with this post to add.
    NewCollectionFor(PostId),
    /// History sheet.
    OpenDocumentHistory(String),
    SelectVersion(usize),
    RestoreVersion(usize),
    CloseHistory,
    /// Export dialog.
    OpenExport,
    SetScope(ScopeKind),
    PickCollection(CollectionId),
    PickPersona(ProfileId),
    /// Fills the date range with the last this many days.
    LastDays(i64),
    ToggleOriginals,
    ToggleJournal,
    RunExport,
    CloseExport,
    /// Snapshot and restore sheets.
    OpenSnapshot,
    CreateSnapshot,
    CloseSnapshot,
    OpenRestore,
    PickSnapshotFile,
    VerifySnapshot,
    ConfirmRestore,
    CloseRestore,
    /// Knowledge page: the Library / Saved switch and the Saved view.
    ShowLibrary,
    ShowSaved,
    SelectShelf(SavedTarget),
    CreateCollection,
    MoveItem { from: usize, to: usize },
    RemoveItem(ItemRef),
    EditNote(ItemRef),
    SaveNote,
    CancelNote,
    OpenItem(ItemRef),
    LearnShelf,
    /// Scheduled queue sheet (Y4).
    OpenScheduled,
    EditScheduled(PostId),
    PublishScheduledNow(PostId),
    CancelScheduled(PostId),
    CloseScheduled,
}

/// Sends a data event to the shell.
pub type DataAction = Rc<dyn Fn(DataEvent, &mut Window, &mut App) + 'static>;
