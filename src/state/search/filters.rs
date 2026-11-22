//! Whether a post passes a query's filters and exclusions.
//!
//! - `from:` — the author's handle or name contains the value (ignoring case).
//! - `kind:` — the post kind's key (`standard`, `feeling`, `question`).
//! - `symbol:` — an entry (`task`), an entry in a state (`task_done`), a signifier
//!   (`priority`) or a custom symbol's name.
//! - `tint:` — the tint's key.
//! - `has:image|file|code|todo` — attached images or files, inline or fenced code, a to-do.
//! - `is:open|done|scheduled` — the entry's state or its to-dos' (scheduled is the journal
//!   state; posts scheduled for publishing are never searched before they are due).
//! - `is:archived|locked` — archived or password-locked.
//! - `before:` is the days before the bound, `after:` the bound and the days after it; a bound
//!   that names no day is left out.

use super::dates::resolve_date;
use super::{text, SearchHit};
use crate::domain::activity::local_date;
use crate::domain::post::BodyToken;
use crate::domain::profile::Profile;
use crate::domain::search_query::{HasFilter, IsFilter, PostQuery, QueryFilter};
use crate::domain::symbols::{entry_key, signifier_key, state_key, EntryState};
use crate::domain::todo::{entries::todo_entries, parse_todos};
use crate::domain::Post;
use crate::state::AppState;
use jiff::civil::Date;
use jiff::tz::TimeZone;

/// When the search runs: the local day and zone.
pub(super) struct When {
    pub today: Date,
    pub tz: TimeZone,
}

/// Whether `post` (found as `hit`) belongs in the results of `query`.
pub(super) fn admits(state: &AppState, post: &Post, hit: &SearchHit, query: &PostQuery, when: &When) -> bool {
    if post.network_id != state.active_network_id || post.is_locked() && (hit.by_text || hit.by_meaning) || text::excluded(post, query) {
        return false;
    }
    query.filters.iter().all(|filter| holds(state, post, filter, when) != Some(false))
        && query.excluded_filters.iter().all(|filter| holds(state, post, filter, when) != Some(true))
}

/// Whether `filter` holds for `post`; `None` when it cannot be applied (a bound naming no day).
fn holds(state: &AppState, post: &Post, filter: &QueryFilter, when: &When) -> Option<bool> {
    Some(match filter {
        QueryFilter::From(name) => written_by(state.author_of(post), name),
        QueryFilter::Kind(kind) => post.kind.key().eq_ignore_ascii_case(kind),
        QueryFilter::Symbol(symbol) => carries_symbol(post, &symbol.to_lowercase()),
        QueryFilter::Tint(tint) => post.extras.tint.is_some_and(|t| t.key().eq_ignore_ascii_case(tint)),
        QueryFilter::Has(what) => has(state, post, *what),
        QueryFilter::Is(state_filter) => is(post, *state_filter),
        QueryFilter::Before(bound) => local_date(post, &when.tz) < resolve_date(bound, when.today)?,
        QueryFilter::After(bound) => local_date(post, &when.tz) >= resolve_date(bound, when.today)?,
    })
}

fn written_by(author: &Profile, name: &str) -> bool {
    let name = name.to_lowercase();
    author.handle.to_lowercase().contains(&name) || author.name.to_lowercase().contains(&name)
}

fn carries_symbol(post: &Post, symbol: &str) -> bool {
    let set = &post.extras.symbols;
    let entry = set.entry.map(entry_key);
    let stated = set.entry.zip(set.state).map(|(entry, state)| format!("{}_{}", entry_key(entry), state_key(state)));
    entry == Some(symbol)
        || stated.as_deref() == Some(symbol)
        || set.signifiers.iter().any(|s| signifier_key(*s) == symbol)
        || set.custom.iter().any(|custom| custom.name.eq_ignore_ascii_case(symbol))
}

fn has(state: &AppState, post: &Post, what: HasFilter) -> bool {
    match what {
        HasFilter::Image => post.image_path.is_some() || !post.extras.images.is_empty(),
        HasFilter::File => !post.extras.files.is_empty(),
        HasFilter::Code => match state.index.meta(post.id.as_str()) {
            Some(meta) => meta.tokens.iter().any(is_code),
            None => crate::domain::post::tokenize_post_body(&post.content).iter().any(is_code),
        },
        HasFilter::Todo => !parse_todos(&post.content).is_empty(),
    }
}

fn is_code(token: &BodyToken) -> bool {
    match token {
        BodyToken::Code(_) | BodyToken::CodeBlock { .. } => true,
        other => other.as_mark().is_some_and(|(_, children)| children.iter().any(is_code)),
    }
}

fn is(post: &Post, what: IsFilter) -> bool {
    let entry = post.extras.symbols.state;
    let todos = || todo_entries(&post.content, &post.extras.todo_state);
    match what {
        IsFilter::Open => entry == Some(EntryState::Open) || todos().iter().any(|t| t.state == EntryState::Open),
        IsFilter::Done => entry == Some(EntryState::Done) || all_settled(&todos()),
        IsFilter::Scheduled => entry == Some(EntryState::Scheduled) || todos().iter().any(|t| t.state == EntryState::Scheduled),
        IsFilter::Archived => post.lifecycle.is_archived(),
        IsFilter::Locked => post.is_locked(),
    }
}

/// Whether there are to-dos and every one is done or dropped.
fn all_settled(todos: &[crate::domain::todo::entries::TodoEntry]) -> bool {
    !todos.is_empty() && todos.iter().all(|todo| todo.state.is_closed())
}
