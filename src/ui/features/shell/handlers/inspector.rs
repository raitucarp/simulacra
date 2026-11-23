//! The inspector's view commands: switching views (W8: remembered per thread, and the last
//! choice as the default of threads never switched, both in the settings table) and the
//! outline's folding, cursor and "open" (remembered per thread for this session).

use super::super::inspector_state::InspectorState;
use super::super::view::AppView;
use crate::domain::views::memory::ViewOwner;
use crate::domain::views::PostView;
use crate::domain::PostId;
use crate::ui::features::feed::inspector_outline::{apply_command, thread_outline, InspectorCommand, OutlineCommand};
use gpui_kit::*;

impl AppView {
    /// Runs one command from the inspector's header or outline.
    pub(crate) fn run_inspector_command(&mut self, command: InspectorCommand, window: &mut Window, cx: &mut Context<Self>) {
        match command {
            InspectorCommand::SetMode(mode) => self.set_inspector_mode(mode, window, cx),
            InspectorCommand::Outline { root, command } => self.run_outline_command(root, command, window, cx),
            InspectorCommand::Month { root, month } => {
                self.inspector.month = Some((root, month));
                cx.notify();
            }
        }
    }

    /// The root of the thread the inspector shows (the selected post's, else the feed's top).
    pub(crate) fn inspected_root(&self) -> Option<PostId> {
        let inspected = self.feed.selected_thread_post_id.clone().or_else(|| self.feed.stream.rows().post(&self.state, 0).map(|post| post.id.clone()));
        inspected.and_then(|id| self.root_of(&id))
    }

    /// W8: the view of the inspected thread: the one chosen for it, else the last one chosen.
    pub(crate) fn inspector_view_mode(&self) -> PostView {
        let chosen = self.inspected_root().and_then(|root| self.inspector.memory.chosen(&ViewOwner::Thread(root)));
        chosen.unwrap_or(self.inspector.mode)
    }

    /// Shows the inspected thread as `mode` and remembers it, for the thread and as the default.
    pub(crate) fn set_inspector_mode(&mut self, mode: PostView, window: &mut Window, cx: &mut Context<Self>) {
        self.inspector.mode = mode;
        InspectorState::save_mode(self.state.db.as_ref(), mode);
        if let Some(root) = self.inspected_root() {
            self.inspector.remember(self.state.db.as_ref(), &ViewOwner::Thread(root), mode);
        }
        if mode == PostView::Outline {
            window.focus(&self.inspector.focus, cx);
        }
        cx.notify();
    }

    /// The palette's "Toggle the outline view".
    pub(crate) fn toggle_inspector_outline(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_inspector_mode(self.inspector_view_mode().toggled(), window, cx);
    }

    fn run_outline_command(&mut self, root: PostId, command: OutlineCommand, window: &mut Window, cx: &mut Context<Self>) {
        let Some(root_post) = self.state.get_post(&root) else { return };
        let tree = thread_outline(&self.state, root_post);
        let (thread, open) = apply_command(self.inspector.outlines.thread(&root), &tree, &command);
        self.inspector.outlines = std::mem::take(&mut self.inspector.outlines).with_thread(root, thread);
        match open {
            Some(post_id) => {
                self.reveal_post(post_id, window, cx);
                window.focus(&self.inspector.focus, cx);
            }
            None => cx.notify(),
        }
    }
}
