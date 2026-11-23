//! N6–N10 in the shell's data host: the open sheets of what-if branches (N6) and pinned
//! quotes (N7), and their text fields.

use super::view::AppView;
use crate::ui::features::data::{BranchDialog, QuoteDialog};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;

/// The open N6–N10 sheets.
#[derive(Default)]
pub struct IdeaSheets {
    /// N6: forking a what-if branch.
    pub branch: Option<BranchDialog>,
    /// N7: quoting a passage of a vault document.
    pub quote: Option<QuoteDialog>,
}

impl IdeaSheets {
    /// Whether one of them is open.
    pub fn any_open(&self) -> bool {
        self.branch.is_some() || self.quote.is_some()
    }
}

/// The text fields of the N6–N10 sheets.
pub struct IdeaInputs {
    /// N6: what a branch supposes, and its first post.
    pub branch_label: Entity<InputState>,
    pub branch_text: Entity<InputState>,
    /// N7: the passage search, and the post about the passage.
    pub quote_find: Entity<InputState>,
    pub quote_comment: Entity<InputState>,
}

fn field(placeholder: &str, window: &mut Window, cx: &mut Context<AppView>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

impl IdeaInputs {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        Self {
            branch_label: field("posts.branch.label_placeholder", window, cx),
            branch_text: field("posts.branch.first_post_placeholder", window, cx),
            quote_find: field("posts.pinned.find_placeholder", window, cx),
            quote_comment: field("posts.pinned.comment_placeholder", window, cx),
        }
    }

    /// Every field.
    pub fn all(&self) -> [&Entity<InputState>; 4] {
        [&self.branch_label, &self.branch_text, &self.quote_find, &self.quote_comment]
    }

    /// The quote sheet's list follows its search as it is typed.
    pub(super) fn subscribe(&self, window: &mut Window, cx: &mut Context<AppView>) -> Vec<Subscription> {
        let search = cx.subscribe_in(&self.quote_find, window, |this: &mut AppView, _, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.refresh_quote_filter(cx);
                cx.notify();
            }
        });
        vec![search]
    }
}
