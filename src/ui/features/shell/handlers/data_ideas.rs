//! N6–N10: the open idea sheet over the shell (what-if branch, pinned quote, series), with the event
//! that closes it; what every card's menu shares; and setting who may read a post (N8).

use super::super::view::AppView;
use crate::domain::post::audience::Audience;
use crate::domain::PostId;
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{render_branch_sheet, render_quote_sheet, render_series_sheet, render_correction_sheet, render_ink_sheet, render_voice_sheet, tr, BranchEvent, DataAction, DataEvent, IdeaSource, CorrectionEvent, InkEvent, QuoteEvent, SeriesEvent, VoiceEvent};
use gpui_kit::*;

impl AppView {
    /// The open N6–N10 sheet and the event closing it, if one is open.
    pub(in crate::ui::features::shell) fn idea_overlay(&self, on_event: DataAction) -> Option<(Stateful<Div>, DataEvent)> {
        let (sheets, theme) = (&self.data.ideas, &self.theme);
        if let Some(dialog) = &sheets.branch {
            return Some((render_branch_sheet(dialog, on_event, theme), DataEvent::Branch(BranchEvent::Close)));
        }
        if let Some(dialog) = &sheets.series {
            return Some((render_series_sheet(dialog, on_event, theme), DataEvent::Series(SeriesEvent::Close)));
        }
        if let Some(dialog) = &sheets.voice {
            return Some((render_voice_sheet(dialog, on_event, theme), DataEvent::Voice(VoiceEvent::Close))); // N11
        }
        if let Some(dialog) = &sheets.ink {
            return Some((render_ink_sheet(dialog, on_event, theme), DataEvent::Ink(InkEvent::Close))); // N12
        }
        if let Some(dialog) = &sheets.correction {
            return Some((render_correction_sheet(dialog, on_event, theme), DataEvent::Correction(CorrectionEvent::Close))); // N14
        }
        let dialog = sheets.quote.as_ref()?;
        Some((render_quote_sheet(dialog, on_event, theme), DataEvent::Quote(QuoteEvent::Close)))
    }

    /// What every card's menu shares this render.
    pub(in crate::ui::features::shell) fn idea_source(&self) -> IdeaSource {
        IdeaSource { agents: self.state.audience_agents(), series: self.state.series_listings().into_iter().map(|listing| listing.title).collect(), playing: self.data.ideas.playing.as_ref().map(|playing| playing.post.clone()), transcribing: self.data.ideas.transcribing.clone(), rechecking: self.data.ideas.rechecking.clone(), rechecks: self.data.ideas.rechecks.clone() }
    }

    /// Sets who may read the user's post `id` (`None`: everyone) and says what changed.
    pub(in crate::ui::features::shell) fn set_audience(&mut self, id: &PostId, audience: Option<Audience>, window: &mut Window, cx: &mut Context<Self>) {
        let key = match &audience {
            None => "posts.audience.set_everyone",
            Some(Audience::Agents { agents }) if !agents.is_empty() => "posts.audience.set_agents",
            Some(_) => "posts.audience.set_only_me",
        };
        match self.state.set_audience(id, audience) {
            Ok(true) => self.notify_data(tr(key), window, cx),
            Ok(false) => {}
            Err(err) => self.notify_data(publish_error_message(&err), window, cx),
        }
    }
}

impl AppView {
    /// An info toast from where no window is at hand (a request that finished in the background).
    pub(in crate::ui::features::shell) fn notify_soon(&self, text: String, cx: &mut Context<Self>) {
        let handle = self.spend.window;
        cx.defer(move |cx| {
            let _ = handle.update(cx, |_, window, cx| {
                use gpui_kit::component::WindowExt;
                window.push_notification(gpui_kit::component::notification::Notification::info(text), cx);
            });
        });
    }
}
