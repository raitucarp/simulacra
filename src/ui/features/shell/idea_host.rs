//! N6–N11 in the shell's data host: the open sheets of what-if branches (N6), pinned quotes
//! (N7), series (N10) and voice notes (N11, with the recording under way), and their text fields.

use super::view::AppView;
use crate::ui::features::data::{BranchDialog, QuoteDialog, CorrectionDialog, InkDialog, SeriesDialog, VoiceDialog};
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
    /// N10: starting a series.
    pub series: Option<SeriesDialog>,
    /// N11: a new voice note, and the recording under way (dropping it discards the take).
    pub voice: Option<VoiceDialog>,
    pub recording: Option<VoiceRecording>,
    /// N11: the voice note playing in the app (the card shows Stop), until it ends.
    pub playing: Option<VoicePlaying>,
    /// N11: the post whose voice note is being transcribed.
    pub transcribing: Option<crate::domain::PostId>,
    /// N12: a new drawing.
    pub ink: Option<InkDialog>,
    /// N14: retracting or correcting a post.
    pub correction: Option<CorrectionDialog>,
    /// N13: the claim the AI is asked about, and the answers so far; the ticket of the newest AI request (N13, N15).
    pub rechecking: Option<crate::domain::PostId>,
    pub rechecks: Vec<(crate::domain::PostId, String)>,
    pub ticket: u64,
    /// N15: the thread whose summary the AI is writing.
    pub distilling: Option<crate::domain::PostId>,
    /// N16–N20: their sheets.
    pub thoughts: super::thought_host::ThoughtSheets,
}

impl IdeaSheets {
    /// Whether one of them is open.
    pub fn any_open(&self) -> bool {
        self.branch.is_some() || self.quote.is_some() || self.series.is_some() || self.voice.is_some() || self.ink.is_some() || self.correction.is_some() || self.thoughts.any_open()
    }
}

/// N11: a note playing in the app, and the task that marks its end.
pub struct VoicePlaying {
    pub post: crate::domain::PostId,
    pub _end: Option<Task<()>>,
}

/// N11: a recording under way and the task that ticks its clock.
pub struct VoiceRecording {
    pub recorder: crate::media::audio::record::Recorder,
    pub _tick: Task<()>,
}

/// The text fields of the N6–N10 sheets.
pub struct IdeaInputs {
    /// N6: what a branch supposes, and its first post.
    pub branch_label: Entity<InputState>,
    pub branch_text: Entity<InputState>,
    /// N7: the passage search, and the post about the passage.
    pub quote_find: Entity<InputState>,
    pub quote_comment: Entity<InputState>,
    /// N10: a new series' title and planned parts.
    pub series_title: Entity<InputState>,
    pub series_goal: Entity<InputState>,
    /// N11: a voice note's caption.
    pub voice_caption: Entity<InputState>,
    /// N12: a drawing's caption.
    pub ink_caption: Entity<InputState>,
    /// N14: why a post is retracted, or what is right instead.
    pub correction_reason: Entity<InputState>,
    /// N16–N20: their fields.
    pub thoughts: super::thought_host::ThoughtInputs,
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
            series_title: field("posts.series.title_placeholder", window, cx),
            series_goal: field("posts.series.goal_placeholder", window, cx),
            voice_caption: field("posts.voice.caption_placeholder", window, cx),
            ink_caption: field("posts.ink.caption_placeholder", window, cx),
            correction_reason: field("posts.correction.reason_placeholder", window, cx),
            thoughts: super::thought_host::ThoughtInputs::new(window, cx),
        }
    }

    /// Every field.
    pub fn all(&self) -> [&Entity<InputState>; 9] {
        [&self.branch_label, &self.branch_text, &self.quote_find, &self.quote_comment, &self.series_title, &self.series_goal, &self.voice_caption, &self.ink_caption, &self.correction_reason]
    }

    /// The quote sheet's list follows its search as it is typed.
    pub(super) fn subscribe(&self, window: &mut Window, cx: &mut Context<AppView>) -> Vec<Subscription> {
        let search = cx.subscribe_in(&self.quote_find, window, |this: &mut AppView, _, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.refresh_quote_filter(cx);
                cx.notify();
            }
        });
        let mut subscriptions = vec![search];
        subscriptions.extend(self.thoughts.subscribe(window, cx)); // N16–N20
        subscriptions
    }
}
