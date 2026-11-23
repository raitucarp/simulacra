//! The inspector's view commands (sb C2): switching cards / outline (saved as a setting) and
//! the outline's folding, cursor and "open" (remembered per thread for this session).

use super::super::inspector_state::InspectorState;
use super::super::view::AppView;
use crate::domain::PostId;
use crate::domain::views::PostView;
use crate::ui::features::feed::inspector_outline::{apply_command, thread_outline, InspectorCommand, OutlineCommand};
use gpui_kit::*;

impl AppView {
    /// Runs one command from the inspector's header or outline.
    pub(crate) fn run_inspector_command(&mut self, command: InspectorCommand, window: &mut Window, cx: &mut Context<Self>) {
        match command {
            InspectorCommand::SetMode(mode) => self.set_inspector_mode(mode, window, cx),
            InspectorCommand::Outline { root, command } => self.run_outline_command(root, command, window, cx),
        }
    }

    /// Shows the thread as `mode` and remembers it.
    pub(crate) fn set_inspector_mode(&mut self, mode: PostView, window: &mut Window, cx: &mut Context<Self>) {
        self.inspector.mode = mode;
        InspectorState::save_mode(self.state.db.as_ref(), mode);
        if mode == PostView::Outline {
            window.focus(&self.inspector.focus, cx);
        }
        cx.notify();
    }

    /// The palette's "Toggle the outline view".
    pub(crate) fn toggle_inspector_outline(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.set_inspector_mode(self.inspector.mode.toggled(), window, cx);
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
