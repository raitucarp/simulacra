//! Running a code block (P5): which blocks get a Run button, what the button hands the shell,
//! and when the user must see the code and confirm before it runs.

use super::layout::Item;
use super::BodyLinks;
use crate::domain::code::{runtime_for, Runtime};
use crate::domain::profile::Profile;
use crate::domain::Post;
use gpui_kit::{App, Window};
use std::rc::Rc;

/// One press of Run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunRequest {
    /// The post the block is in.
    pub post: String,
    /// The block's place among the post's code blocks (from 0).
    pub block: usize,
    pub runtime: Runtime,
    pub code: String,
}

pub type RunCodeAction = Rc<dyn Fn(RunRequest, &mut Window, &mut App) + 'static>;

/// What code blocks may do: only blocks in a runtime that is switched on and found get Run.
#[derive(Clone)]
pub struct CodeActions {
    /// Runtimes a block may run in now (Settings › Code allows them and they were found).
    pub runnable: Rc<[Runtime]>,
    pub on_run: RunCodeAction,
}

impl CodeActions {
    /// The runtime a block tagged `lang` runs in, when it may run.
    pub fn runtime_of(&self, lang: Option<&str>) -> Option<Runtime> {
        lang.and_then(runtime_for).filter(|runtime| self.runnable.contains(runtime))
    }
}

/// The Run button of one block: the request it sends.
#[derive(Clone)]
pub(super) struct RunButton {
    pub request: RunRequest,
    pub on_run: RunCodeAction,
}

/// The Run button of the block at `index` of `items`, when it may run.
pub(super) fn run_button(items: &[Item<'_>], index: usize, lang: Option<&str>, code: &str, links: &BodyLinks) -> Option<RunButton> {
    let actions = links.code.as_ref()?;
    let runtime = actions.runtime_of(lang)?;
    let block = items[..index].iter().filter(|item| matches!(item, Item::Block { .. })).count();
    let request = RunRequest { post: links.scope.clone(), block, runtime, code: code.to_string() };
    Some(RunButton { request, on_run: actions.on_run.clone() })
}

/// Whether code from `post` (written by `author`) is shown to the user to confirm before it
/// runs: always, unless one of the user's own personas typed it (agents' posts, imported and
/// captured passages all ask).
pub fn needs_confirm(post: &Post, author: &Profile) -> bool {
    !author.is_human_persona() || post.extras.origin.is_some()
}
