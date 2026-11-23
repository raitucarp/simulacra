//! Renders the list views of the registry (every view but the thread inspector's own cards,
//! outline and argument map) from a [`ViewInput`].

use super::input::ViewInput;
use super::rapid_log::render_rapid_log;
use super::board::render_board;
use super::calendar::render_calendar;
use super::future::render_future;
use super::garden::render_garden;
use super::branches::render_branches;
use super::series::render_series;
use super::still_true::render_still_true;
use super::lens_fan::render_lens_fan;
use super::glossary::render_glossary;
use super::open_questions::render_open_questions;
use super::mind_changes::render_mind_changes;
use super::raw::render_raw;
use crate::domain::views::PostView;
use gpui_kit::AnyElement;

/// `view` over the input's posts; `None` for the views their scope draws itself.
pub fn render_list_view(view: PostView, input: &ViewInput<'_>) -> Option<AnyElement> {
    match view {
        PostView::RapidLog => Some(render_rapid_log(input)),
        PostView::Raw => Some(render_raw(input)),
        PostView::Calendar => Some(render_calendar(input)),
        PostView::Future => Some(render_future(input)),
        PostView::Board => Some(render_board(input)),
        PostView::Garden => Some(render_garden(input)), // N3
        PostView::Branches => Some(render_branches(input)), // N6
        PostView::Series => Some(render_series(input)), // N10
        PostView::StillTrue => Some(render_still_true(input)), // N13
        PostView::LensFan => Some(render_lens_fan(input)), // N16
        PostView::Glossary => Some(render_glossary(input)), // N17
        PostView::OpenQuestions => Some(render_open_questions(input)), // N18
        PostView::MindChanges => Some(render_mind_changes(input)), // N19
        PostView::Cards | PostView::Outline | PostView::Argument => None,
    }
}
