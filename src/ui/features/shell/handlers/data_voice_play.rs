//! N11: playing a voice note from its card. A WAV note plays in the app (Stop on the card
//! while it does, and the card goes back to Play when the note ends); any other file opens in
//! the system's player. A locked post's note is never played.

use super::super::idea_host::VoicePlaying;
use super::super::view::AppView;
use crate::domain::PostId;
use crate::media::audio::play::{play, stop, Playback};
use rust_i18n::t;
use gpui_kit::*;
use std::time::Duration;

/// Time added to a note's length before the card goes back to Play.
const END_MARGIN: Duration = Duration::from_millis(300);

impl AppView {
    /// Plays the voice note of post `id`.
    pub(in crate::ui::features::shell) fn play_voice(&mut self, id: &PostId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(note) = self.state.get_post(id).filter(|post| !post.is_locked()).and_then(|post| post.extras.voice.as_deref().cloned()) else { return };
        self.data.ideas.playing = None;
        match play(std::path::Path::new(&note.path), note.is_wav()) {
            Ok(Playback::InApp) => {
                let length = note.duration_ms.map(Duration::from_millis);
                let end = length.map(|length| {
                    let ended = id.clone();
                    cx.spawn_in(window, async move |entity, cx| {
                        cx.background_executor().timer(length + END_MARGIN).await;
                        let _ = entity.update(cx, |this, cx| this.voice_ended(&ended, cx));
                    })
                });
                self.data.ideas.playing = Some(VoicePlaying { post: id.clone(), _end: end });
            }
            Ok(Playback::External) => {}
            Err(reason) => self.notify_data(t!("posts.voice.play_failed", reason = reason).to_string(), window, cx),
        }
    }

    /// Stops the note playing in the app.
    pub(in crate::ui::features::shell) fn stop_voice(&mut self) {
        stop();
        self.data.ideas.playing = None;
    }

    fn voice_ended(&mut self, id: &PostId, cx: &mut Context<Self>) {
        if self.data.ideas.playing.as_ref().is_some_and(|playing| &playing.post == id) {
            self.data.ideas.playing = None;
            cx.notify();
        }
    }
}
