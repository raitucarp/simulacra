//! Where the vault is (browse, entry, reader, subject index, editor) and the history the
//! back button walks.

/// How the browse view lays results out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BrowseLayout {
    /// Covers on shelves (recognition by look).
    #[default]
    Shelf,
    /// A sortable table (comparison by field).
    Table,
}

/// One screen of the vault.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Browse,
    /// The encyclopedic entry of a document.
    Entry(String),
    /// The reading view of a document, scrolled to a block when given.
    Reader(String),
    SubjectIndex,
    /// The add (none) or edit (document id) form.
    Editor(Option<String>),
}

/// The current screen and the screens before it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Navigation {
    pub current: Screen,
    history: Vec<Screen>,
}

/// Deepest history kept (older screens are dropped).
const MAX_HISTORY: usize = 32;

impl Navigation {
    /// Goes to `screen`, remembering the current one (going to the same screen is a no-op).
    pub fn go(mut self, screen: Screen) -> Self {
        if screen != self.current {
            let previous = std::mem::replace(&mut self.current, screen);
            self.history.push(previous);
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }
        self
    }

    /// Returns to the previous screen (browse when there is none).
    pub fn back(mut self) -> Self {
        self.current = self.history.pop().unwrap_or_default();
        self
    }

    /// Back to browsing with no history.
    pub fn home() -> Self {
        Self::default()
    }

    /// Drops screens of documents that no longer exist (after a delete).
    pub fn without_document(mut self, id: &str) -> Self {
        let mentions = |screen: &Screen| match screen {
            Screen::Entry(doc) | Screen::Reader(doc) => doc == id,
            Screen::Editor(doc) => doc.as_deref() == Some(id),
            Screen::Browse | Screen::SubjectIndex => false,
        };
        self.history.retain(|screen| !mentions(screen));
        if mentions(&self.current) {
            self.current = self.history.pop().unwrap_or_default();
        }
        self
    }
}
