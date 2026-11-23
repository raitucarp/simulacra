//! N20: bilingual posts in the shell. "Show in <language>" shows the post's kept translation
//! into the language the user is learning (Settings › Learning), or first asks the AI engine
//! for it (after the spend check, through the post actions runner) and keeps it; "Show
//! original" toggles back. A locked post is never translated.

use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post::translation::prompt::{can_translate, translation_prompt, TRANSLATION_TOKENS};
use crate::domain::spend::SpendEstimate;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, TranslateEvent};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// Routes one bilingual post event.
    pub(in crate::ui::features::shell) fn on_translate_event(&mut self, event: TranslateEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            TranslateEvent::Show(id) => self.show_translation(id, cx),
            TranslateEvent::Original(id) => self.data.ideas.thoughts.translated.retain(|shown| shown != &id),
            other => self.on_words_event(other, window, cx),
        }
    }

    /// Shows post `id` in the language the user is learning, asking for it the first time.
    fn show_translation(&mut self, id: PostId, cx: &mut Context<Self>) {
        let language = self.state.preferences().learning.study_language;
        let Some(post) = self.state.get_post(&id).filter(|post| can_translate(post)) else { return }; // X2: never a locked post
        if post.extras.translation.as_ref().is_some_and(|kept| kept.language == language.code()) {
            return self.data.ideas.thoughts.translated.push(id);
        }
        let prompt = translation_prompt(post, language.english_label());
        if self.data.ideas.thoughts.translating.is_some() {
            return;
        }
        let resume = id.clone();
        if self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.show_translation(resume, cx)), cx) {
            return;
        }
        match self.thinking_job(prompt, TRANSLATION_TOKENS, CancelToken::new()) {
            Ok(job) => {
                let thoughts = &mut self.data.ideas.thoughts;
                thoughts.translate_ticket += 1;
                thoughts.translating = Some(id);
                self.spawn_thinking_job(job, self.data.ideas.thoughts.translate_ticket, Self::finish_translation, cx);
            }
            Err(failure) => self.notify_soon(t!("posts.translation.failed", reason = failure_message(&failure)).to_string(), cx),
        }
        cx.notify();
    }

    /// Keeps a finished translation and shows it, or says why there is none.
    fn finish_translation(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let thoughts = &mut self.data.ideas.thoughts;
        let Some(id) = thoughts.translating.take().filter(|_| ticket == thoughts.translate_ticket) else { return };
        let language = self.state.preferences().learning.study_language;
        let kept = outcome.map_err(|failure| failure_message(&failure)).and_then(|answer| self.state.keep_translation(&id, language.code(), &answer, jiff::Timestamp::now()).map_err(|err| match err {
            crate::error::AppError::EmptyContent => tr("posts.distill.empty"),
            err => publish_error_message(&err),
        }));
        match kept {
            Ok(_) => self.data.ideas.thoughts.translated.push(id),
            Err(reason) => self.notify_soon(t!("posts.translation.failed", reason = reason).to_string(), cx),
        }
        cx.notify();
    }
}
