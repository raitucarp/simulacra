//! Renders the list views of the registry (every view but the thread inspector's own cards,
//! outline and argument map) from a [`ViewInput`].

use super::input::ViewInput;
use super::rapid_log::render_rapid_log;
use super::raw::render_raw;
use crate::domain::views::PostView;
use gpui_kit::AnyElement;

/// `view` over the input's posts; `None` for the views their scope draws itself.
pub fn render_list_view(view: PostView, input: &ViewInput<'_>) -> Option<AnyElement> {
    match view {
        PostView::RapidLog => Some(render_rapid_log(input)),
        PostView::Raw => Some(render_raw(input)),
        PostView::Cards | PostView::Outline | PostView::Argument => None,
    }
}
