//! The to-do syntax at a glance (D1): what the composer's help card lists, row by row. Each
//! row shows how a piece is typed and names the locale key of what it does; the boxes reuse
//! the entry state names.

use crate::domain::symbols::EntryState;

/// What a row of the help card explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxTopic {
    /// A box and the state it stands for.
    Box(EntryState),
    Nesting,
    Due,
    Repeat,
    Priority,
    Context,
    Estimate,
    Delegate,
}

/// One row of the help card: how it is typed, and what it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyntaxRow {
    pub sample: &'static str,
    pub topic: SyntaxTopic,
}

/// Every row of the help card, boxes first.
pub const SYNTAX_ROWS: [SyntaxRow; 13] = [
    row("[ ]", SyntaxTopic::Box(EntryState::Open)),
    row("[x]", SyntaxTopic::Box(EntryState::Done)),
    row("[>]", SyntaxTopic::Box(EntryState::Migrated)),
    row("[<]", SyntaxTopic::Box(EntryState::Scheduled)),
    row("[~]", SyntaxTopic::Box(EntryState::Dropped)),
    row("[/]", SyntaxTopic::Box(EntryState::Delegated)),
    row("  [ ]", SyntaxTopic::Nesting),
    row("^fri  ^2026-12-01  ^+3d", SyntaxTopic::Due),
    row("^^daily  ^^mon,wed  ^^15th", SyntaxTopic::Repeat),
    row("!  !!  !!!", SyntaxTopic::Priority),
    row("&home", SyntaxTopic::Context),
    row("~25m", SyntaxTopic::Estimate),
    row("@agent", SyntaxTopic::Delegate),
];

const fn row(sample: &'static str, topic: SyntaxTopic) -> SyntaxRow {
    SyntaxRow { sample, topic }
}

impl SyntaxTopic {
    /// The locale key of what the row does; `None` for a box, which is named by its state.
    pub fn label_key(self) -> Option<&'static str> {
        Some(match self {
            Self::Box(_) => return None,
            Self::Nesting => "todo.help.nesting",
            Self::Due => "todo.help.due",
            Self::Repeat => "todo.help.repeat",
            Self::Priority => "todo.help.priority",
            Self::Context => "todo.help.context",
            Self::Estimate => "todo.help.estimate",
            Self::Delegate => "todo.help.delegate",
        })
    }
}
