//! N11: a voice note on its card: Play (or Stop while it plays in the app), "Voice note" and its
//! length, and its transcript once there is one. A locked post shows nothing of it: only the placeholder.

use super::controls::PostCardHandlers;
use crate::domain::post::voice::{duration_label, VoiceNote};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::data::{DataEvent, VoiceEvent};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The voice note `post` shows (`None`: none, or the post is locked).
pub(crate) fn shown_voice(post: &Post) -> Option<&VoiceNote> {
    post.extras.voice.as_deref().filter(|_| !post.is_locked())
}

/// The line beside the button: "Voice note · 0:42". The stored file's name is a content
/// hash (recorded or chosen alike), which tells a reader nothing.
pub(crate) fn voice_label(note: &VoiceNote) -> String {
    let name = t!("posts.voice.label").to_string();
    match note.duration_ms {
        Some(ms) => format!("{name} · {}", duration_label(ms)),
        None => name,
    }
}

fn play_button(post: &Post, playing: bool, handlers: &PostCardHandlers) -> Option<Button> {
    let on_event = handlers.data.as_ref()?.on_event.clone();
    let (icon, label, event) = match playing {
        true => (LucideIcon::CircleStop, t!("posts.voice.stop_playing"), VoiceEvent::StopPlaying),
        false => (LucideIcon::Play, t!("posts.voice.play"), VoiceEvent::Play(post.id.clone())),
    };
    let button = Button::new(SharedString::from(format!("voice-play-{}", post.id))).small().icon(Icon::new(icon)).label(label.to_string());
    Some(button.on_click(move |_, window, cx| on_event(DataEvent::Voice(event.clone()), window, cx)))
}

/// The card's voice note; `None` without one (or on a locked post).
pub(super) fn render_voice_card(post: &Post, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let note = shown_voice(post)?;
    let playing = handlers.data.as_ref().is_some_and(|data| data.ideas.playing.as_ref() == Some(&post.id));
    let label = div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_muted).child(voice_label(note));
    let row = div().flex().items_center().gap(theme.inline_gap).child(Icon::new(LucideIcon::Mic).small().text_color(theme.accent_brand_text)).children(play_button(post, playing, handlers)).child(label);
    let transcribing = handlers.data.as_ref().is_some_and(|data| data.ideas.transcribing.as_ref() == Some(&post.id));
    let note_line = |text: String| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text);
    let transcript = note.transcript.clone().map(|text| div().flex().flex_col().gap(theme.spacing_3xs).child(note_line(t!("posts.voice.transcript_label").to_string())).child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(text)));
    let pending = transcribing.then(|| note_line(t!("posts.voice.transcribing").to_string()));
    Some(div().w_full().flex().flex_col().gap(theme.spacing_xs).p(theme.spacing_sm).rounded(theme.radius_md).bg(theme.bg_callout).child(row).children(transcript).children(pending))
}
