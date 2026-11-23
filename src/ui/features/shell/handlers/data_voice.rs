//! N11: the voice note sheet in the shell. Recording runs on the recorder's own thread while
//! a task ticks the sheet's clock; stopping stores the take in the media folder off the UI
//! thread, as does a chosen audio file. Posting publishes it with the caption. Closing the
//! sheet discards a take still being recorded.

use super::super::idea_host::VoiceRecording;
use super::super::view::AppView;
use crate::domain::post::voice::{audio_extensions, VoiceNote, MAX_VOICE_BYTES};
use crate::media::audio::record::{can_record, RecordError, Recorder};
use crate::media::audio::{store_recording, store_voice_file};
use crate::ui::features::composer::publish_error_message;
use crate::ui::features::data::{tr, VoiceDialog, VoiceEvent};
use gpui_kit::*;
use rust_i18n::t;
use std::time::Duration;

/// How often the recording clock moves.
const TICK: Duration = Duration::from_millis(250);

/// A recording problem as the user reads it.
fn record_problem(err: &RecordError) -> String {
    match err {
        RecordError::Unavailable => tr("posts.voice.no_recorder"),
        RecordError::NoMicrophone(code) => t!("posts.voice.no_microphone", code = code).to_string(),
        RecordError::Failed(detail) => t!("posts.voice.failed", detail = detail.clone()).to_string(),
    }
}

impl AppView {
    /// Routes one voice note event.
    pub(in crate::ui::features::shell) fn on_voice_event(&mut self, event: VoiceEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            VoiceEvent::Open => self.open_voice(window, cx),
            VoiceEvent::Record => self.start_recording(window, cx),
            VoiceEvent::Stop => self.stop_recording(window, cx),
            VoiceEvent::Choose => self.choose_voice_file(window, cx),
            VoiceEvent::Discard => self.edit_voice(|dialog| dialog.note = None),
            VoiceEvent::Play(id) => self.play_voice(&id, window, cx),
            VoiceEvent::StopPlaying => self.stop_voice(),
            VoiceEvent::Submit => self.submit_voice(window, cx),
            VoiceEvent::Close => self.close_voice(window, cx),
        }
        cx.notify();
    }

    fn edit_voice(&mut self, change: impl FnOnce(&mut VoiceDialog)) {
        if let Some(dialog) = self.data.ideas.voice.as_mut() {
            change(dialog);
        }
    }

    pub(in crate::ui::features::shell) fn open_voice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let caption = self.data.inputs.ideas.voice_caption.clone();
        caption.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.voice = Some(VoiceDialog { caption, note: None, recording_ms: None, storing: false, can_record: can_record(), problem: None });
    }

    fn start_recording(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.data.ideas.recording.is_some() {
            return;
        }
        match Recorder::start() {
            Ok(recorder) => {
                let tick = cx.spawn_in(window, async move |entity, cx| loop {
                    cx.background_executor().timer(TICK).await;
                    if entity.update(cx, |this, cx| this.tick_recording(cx)).is_err() {
                        break;
                    }
                });
                self.data.ideas.recording = Some(VoiceRecording { recorder, _tick: tick });
                self.edit_voice(|dialog| (dialog.recording_ms, dialog.note, dialog.problem) = (Some(0), None, None));
            }
            Err(err) => self.edit_voice(|dialog| dialog.problem = Some(record_problem(&err))),
        }
    }

    fn tick_recording(&mut self, cx: &mut Context<Self>) {
        let Some(ms) = self.data.ideas.recording.as_ref().map(|r| r.recorder.elapsed_ms()) else { return };
        self.edit_voice(|dialog| dialog.recording_ms = Some(ms));
        cx.notify();
    }

    fn stop_recording(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(recording) = self.data.ideas.recording.take() else { return };
        self.edit_voice(|dialog| (dialog.recording_ms, dialog.storing) = (None, true));
        let media_dir = self.state.media_dir();
        let VoiceRecording { recorder, .. } = recording;
        let task = cx.background_executor().spawn(async move { recorder.finish().map_err(|err| record_problem(&err)).and_then(|wav| store_recording(&wav, &media_dir)) });
        self.finish_voice_store(task, window, cx);
    }

    fn choose_voice_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (title, filter, media_dir) = (tr("posts.voice.choose_title"), tr("posts.voice.audio_filter"), self.state.media_dir());
        cx.spawn_in(window, async move |entity, cx| {
            let Some(file) = rfd::AsyncFileDialog::new().set_title(&title).add_filter(filter, &audio_extensions()).pick_file().await else { return };
            let path = file.path().to_path_buf();
            let _ = entity.update_in(cx, |this, window, cx| {
                this.edit_voice(|dialog| dialog.storing = true);
                let task = cx.background_executor().spawn(async move { store_voice_file(&path, &media_dir) });
                this.finish_voice_store(task, window, cx);
            });
        })
        .detach();
    }

    /// Puts the stored note in the sheet once `task` is done, or says why it is not.
    fn finish_voice_store(&mut self, task: Task<Result<VoiceNote, String>>, window: &mut Window, cx: &mut Context<Self>) {
        cx.spawn_in(window, async move |entity, cx| {
            let stored = task.await;
            let _ = entity.update(cx, |this, cx| {
                this.edit_voice(|dialog| {
                    dialog.storing = false;
                    match stored {
                        Ok(note) => (dialog.note, dialog.problem) = (Some(note), None),
                        Err(reason) => dialog.problem = Some(t!("posts.voice.not_stored", reason = reason, limit = MAX_VOICE_BYTES / (1024 * 1024)).to_string()),
                    }
                });
                cx.notify();
            });
        })
        .detach();
    }

    fn submit_voice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(dialog) = self.data.ideas.voice.as_ref() else { return };
        let Some(note) = dialog.note.clone().filter(|_| dialog.recording_ms.is_none()) else { return };
        let caption = dialog.caption.read(cx).value().to_string();
        match self.state.submit_voice_post(&caption, note) {
            Ok(_) => {
                self.notify_data(tr("posts.voice.posted"), window, cx);
                self.close_voice(window, cx);
            }
            Err(err) => self.edit_voice(|dialog| dialog.problem = Some(publish_error_message(&err))),
        }
    }

    fn close_voice(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.data.ideas.recording = None;
        self.data.inputs.ideas.voice_caption.update(cx, |field, cx| field.set_value("", window, cx));
        self.data.ideas.voice = None;
    }
}
