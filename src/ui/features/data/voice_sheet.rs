//! N11: the voice note sheet ("New voice note"): record from the microphone (where the
//! platform can) or choose an audio file, add a caption if you like, and post it.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::post::voice::{duration_label, VoiceNote};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SHEET_ID: &str = "data-voice";

/// One interaction with the voice note sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VoiceEvent {
    Open,
    Record,
    Stop,
    /// Chooses an audio file instead of recording.
    Choose,
    /// Drops the take (or file) to start again.
    Discard,
    Submit,
    Close,
}

/// The sheet's state.
#[derive(Clone)]
pub struct VoiceDialog {
    pub caption: Entity<InputState>,
    /// The take or chosen file, once stored in the media folder.
    pub note: Option<VoiceNote>,
    /// How long it has been recording (`None`: not recording).
    pub recording_ms: Option<u64>,
    /// A take or file is being stored.
    pub storing: bool,
    /// The platform can record.
    pub can_record: bool,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: VoiceEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Voice(event.clone()), window, cx)
}

fn button(id: &'static str, icon: LucideIcon, label: &str, on_event: &DataAction, event: VoiceEvent) -> Button {
    Button::new(id).small().icon(Icon::new(icon)).label(tr(label)).on_click(send(on_event, event))
}

/// The row that records, shows the take, or offers a file.
fn take_row(dialog: &VoiceDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let row = div().flex().flex_wrap().items_center().gap(theme.inline_gap).text_size(theme.font_size_sm).text_color(theme.text_primary);
    if let Some(ms) = dialog.recording_ms {
        let live = div().flex().items_center().gap(theme.inline_gap).text_color(theme.accent_danger).child(Icon::new(LucideIcon::Mic).small()).child(t!("posts.voice.recording", time = duration_label(ms)).to_string());
        return row.child(live).child(button("data-voice-stop", LucideIcon::CircleStop, "posts.voice.stop", on_event, VoiceEvent::Stop).primary());
    }
    if let Some(note) = &dialog.note {
        let length = note.duration_ms.map(|ms| format!(" · {}", duration_label(ms))).unwrap_or_default();
        let take = div().flex_1().min_w_0().truncate().child(format!("{}{length}", note.file_name()));
        return row.child(Icon::new(LucideIcon::Volume2).small().text_color(theme.text_muted)).child(take).child(button("data-voice-discard", LucideIcon::X, "posts.voice.discard", on_event, VoiceEvent::Discard).ghost());
    }
    if dialog.storing {
        return row.child(hint(tr("posts.voice.storing"), theme));
    }
    let record = dialog.can_record.then(|| button("data-voice-record", LucideIcon::Mic, "posts.voice.record", on_event, VoiceEvent::Record).primary());
    row.children(record).child(button("data-voice-choose", LucideIcon::FileUp, "posts.voice.choose", on_event, VoiceEvent::Choose).ghost())
}

/// The overlay.
pub fn render_voice_sheet(dialog: &VoiceDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let explain = if dialog.can_record { "posts.voice.explain" } else { "posts.voice.explain_no_mic" };
    let caption = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.voice.caption_label"), theme)).child(Input::new(&dialog.caption));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr(explain), theme)).child(take_row(dialog, &on_event, theme)).child(caption).children(problem);
    let ready = dialog.note.is_some() && dialog.recording_ms.is_none();
    let buttons = footer(theme)
        .child(Button::new("data-voice-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, VoiceEvent::Close)))
        .child(Button::new("data-voice-send").primary().small().icon(Icon::new(LucideIcon::Send)).label(tr("posts.voice.post")).disabled(!ready).on_click(send(&on_event, VoiceEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::Mic,
        title: tr("posts.voice.title"),
        subtitle: Some(tr("posts.voice.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Voice(VoiceEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
