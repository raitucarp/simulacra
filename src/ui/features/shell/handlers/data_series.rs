//! N10: series in the shell: adding a post to one of the user's series, the "New series…"
//! sheet, and leaving a series.

use super::super::view::AppView;
use crate::domain::post::series::{parse_goal, series_title};
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, SeriesDialog, SeriesEvent};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one series event.
    pub(in crate::ui::features::shell) fn on_series_event(&mut self, event: SeriesEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            SeriesEvent::Add(id, title) => {
                self.add_to_series(&id, &title, None, window, cx);
            }
            SeriesEvent::New(id) => self.open_series(id, window, cx),
            SeriesEvent::Leave(id) => match self.state.leave_series(&id) {
                Ok(()) => self.notify_data(tr("posts.series.left"), window, cx),
                Err(err) => self.notify_data(publish_error_message(&err), window, cx),
            },
            SeriesEvent::Submit => self.submit_series(window, cx),
            SeriesEvent::Close => self.close_series(window, cx),
        }
    }

    fn add_to_series(&mut self, id: &PostId, title: &str, goal: Option<u32>, window: &mut Window, cx: &mut Context<Self>) -> bool {
        match self.state.add_to_series(id, title, goal) {
            Ok(entry) => {
                self.notify_data(t!("posts.series.added", title = entry.title, number = entry.number).to_string(), window, cx);
                true
            }
            Err(err) => {
                self.notify_data(publish_error_message(&err), window, cx);
                false
            }
        }
    }

    fn open_series(&mut self, id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        let (title, goal) = (self.data.inputs.ideas.series_title.clone(), self.data.inputs.ideas.series_goal.clone());
        for input in [&title, &goal] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.series = Some(SeriesDialog { post_id: id, title: title.clone(), goal, problem: None });
        title.update(cx, |field, cx| field.focus(window, cx));
    }

    fn close_series(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        for input in [&self.data.inputs.ideas.series_title, &self.data.inputs.ideas.series_goal] {
            input.update(cx, |field, cx| field.set_value("", window, cx));
        }
        self.data.ideas.series = None;
    }

    fn submit_series(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.series.as_ref() else { return };
        let (post, title, goal) = (dialog.post_id.clone(), dialog.title.read(cx).value().to_string(), dialog.goal.read(cx).value().trim().to_string());
        let problem = match (series_title(&title), goal.is_empty() || parse_goal(&goal).is_some()) {
            (None, _) => Some(tr("posts.series.name_first")),
            (_, false) => Some(tr("posts.series.goal_invalid")),
            _ => None,
        };
        if problem.is_none() && self.add_to_series(&post, &title, parse_goal(&goal), window, cx) {
            return self.close_series(window, cx);
        }
        if let Some(dialog) = self.data.ideas.series.as_mut() {
            dialog.problem = problem.or_else(|| Some(tr("posts.series.failed")));
        }
    }
}
