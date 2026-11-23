//! The inspector header's view switch: the thread's views from the registry (epic W).

use super::inspector_outline::{InspectorCallback, InspectorCommand};
use crate::domain::views::{PostView, ViewScope};
use crate::theme::ThemeTokens;
use crate::ui::features::views::{render_view_switcher, PickView};
use gpui_kit::Div;
use std::rc::Rc;

/// Element id prefix of the inspector's view buttons.
const ID_PREFIX: &str = "inspector-mode";

/// The thread's view buttons, the active one in the accent.
pub(crate) fn render_mode_toggle(mode: PostView, on_command: &InspectorCallback, theme: &ThemeTokens) -> Div {
    let on_command = on_command.clone();
    let on_pick: PickView = Rc::new(move |view, window, cx| on_command(InspectorCommand::SetMode(view), window, cx));
    render_view_switcher(ViewScope::Thread, mode, ID_PREFIX, &on_pick, theme)
}
