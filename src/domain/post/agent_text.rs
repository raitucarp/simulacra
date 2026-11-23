//! How an agent reads a post: its text, then a note of what else it carries: its files (L13:
//! names and facts, never contents) and its voice note with the transcript (N11). A locked
//! post is its placeholder alone: nothing it carries is described.

use super::voice::duration_label;
use super::Post;
use crate::domain::media::files::with_files_note;

impl Post {
    /// The post as every agent path hands it to a model.
    pub fn agent_text(&self) -> String {
        if self.is_locked() {
            return self.content.clone();
        }
        let mut text = with_files_note(&self.content, &self.extras.files);
        let notes = [self.voice_note()];
        for note in notes.into_iter().flatten() {
            if !text.is_empty() {
                text.push_str("\n\n");
            }
            text.push_str(&note);
        }
        text
    }

    /// `[Voice note, 0:42] Transcript: …` (or that it was not transcribed yet).
    fn voice_note(&self) -> Option<String> {
        let voice = self.extras.voice.as_deref()?;
        let length = voice.duration_ms.map(|ms| format!(", {}", duration_label(ms))).unwrap_or_default();
        Some(match &voice.transcript {
            Some(transcript) => format!("[Voice note{length}] Transcript: {transcript}"),
            None => format!("[Voice note{length}, not transcribed]"),
        })
    }
}
