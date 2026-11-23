//! N18: open questions in the shell: making a post a question or a thought again, marking it
//! answered (from its own menu, or a reply's "This answers it"), reopening it, and the page of
//! every question still open in the network, where a click opens one.

use super::super::view::AppView;
use crate::domain::views::{PostView, ViewScope};
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{render_questions_sheet, tr, DataAction, DataEvent, QuestionsEvent, ThoughtEvent};
use crate::ui::features::views::{render_list_view, ViewInput};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// Routes one open question event.
    pub(in crate::ui::features::shell) fn on_questions_event(&mut self, event: QuestionsEvent, window: &mut Window, cx: &mut Context<Self>) {
        let (done, key) = match event {
            QuestionsEvent::SetQuestion(id, on) => (self.state.set_question(&id, on), if on { "posts.open_question.asked" } else { "posts.open_question.unasked" }),
            QuestionsEvent::Resolve(id, answer) => (self.state.resolve_question(&id, answer.as_ref(), jiff::Timestamp::now()), "posts.open_question.resolved"),
            QuestionsEvent::Reopen(id) => (self.state.reopen_question(&id), "posts.open_question.reopened"),
            QuestionsEvent::ShowAll => return self.data.ideas.thoughts.questions_open = true,
            QuestionsEvent::Open(id) => return self.open_question_post(id),
            QuestionsEvent::Close => return self.data.ideas.thoughts.questions_open = false,
        };
        let text = done.map_or_else(|err| publish_error_message(&err), |()| tr(key));
        self.notify_data(text, window, cx);
    }

    fn open_question_post(&mut self, id: PostId) {
        self.data.ideas.thoughts.questions_open = false;
        self.state.active_tab = crate::state::NavigationTab::Timeline;
        self.feed.selected_thread_post_id = Some(id);
    }

    /// The page of every open question, while it is open.
    pub(in crate::ui::features::shell) fn questions_page(&self, on_event: DataAction) -> Option<(Stateful<Div>, DataEvent)> {
        if !self.data.ideas.thoughts.questions_open {
            return None;
        }
        let opener = on_event.clone();
        let posts = self.state.open_questions();
        let count = posts.len();
        let on_open: crate::ui::features::views::OpenPost = Rc::new(move |id, window, cx| opener(DataEvent::Thought(ThoughtEvent::Questions(QuestionsEvent::Open(id))), window, cx));
        let input = ViewInput { state: &self.state, theme: &self.theme, posts, scheduled: Vec::new(), scope: ViewScope::Profile, on_open, month: None, on_month: Rc::new(|_, _, _| {}) };
        let list = render_list_view(PostView::OpenQuestions, &input)?;
        Some((render_questions_sheet(list, count, on_event, &self.theme), DataEvent::Thought(ThoughtEvent::Questions(QuestionsEvent::Close))))
    }
}
