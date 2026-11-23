//! How an agent reads a post: its text, then a note of what else it carries: its files (L13:
//! names and facts, never contents), its retraction or correction (N14) and its voice note with
//! the transcript (N11). A locked
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
        let question = super::open_question::agent_note(self).map(str::to_string); // N18
        let notes = [self.extras.correction.as_ref().map(|mark| mark.agent_note()), self.voice_note(), question];
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

impl Post {
    /// N15: the post as it goes into a distilled vault document: its text, its voice transcript
    /// as a quotation, and its drawing as an image of its SVG. A locked post is its placeholder.
    pub fn distill_text(&self) -> String {
        if self.is_locked() {
            return self.content.clone();
        }
        let mut parts = vec![self.content.trim().to_string()];
        parts.extend(self.extras.voice.as_ref().and_then(|voice| voice.transcript.as_ref()).map(|text| format!("> {}", text.replace('\n', "\n> "))));
        parts.extend(self.extras.ink.as_ref().and_then(|ink| ink.svg_path.as_ref()).map(|path| format!("![]({})", path.replace(' ', "%20"))));
        parts.into_iter().filter(|part| !part.is_empty()).collect::<Vec<_>>().join("\n\n")
    }
}
