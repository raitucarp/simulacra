//! N11: voice posts. A post can carry one voice note ([`VoiceNote`], `extras.voice`): an audio
//! file in the vault's media folder (recorded in the app as 16-bit PCM WAV, or an audio file
//! the user chose), its length when known, and the transcript once the AI engine made one.
//! A voice note counts as an attachment, so a post that has one can never be locked; the card
//! and every agent path also show nothing of it on a post that is.

mod wav;

pub use wav::{wav_bytes, wav_duration_ms, RECORD_CHANNELS, RECORD_SAMPLE_RATE};

use serde::{Deserialize, Serialize};
use std::path::Path;

/// Audio files a voice note can be made from, by extension, with their media type.
const AUDIO_TYPES: [(&str, &str); 8] = [
    ("wav", "audio/wav"),
    ("mp3", "audio/mpeg"),
    ("m4a", "audio/mp4"),
    ("ogg", "audio/ogg"),
    ("oga", "audio/ogg"),
    ("opus", "audio/opus"),
    ("flac", "audio/flac"),
    ("webm", "audio/webm"),
];
/// The longest recording, in milliseconds (ten minutes).
pub const MAX_RECORDING_MS: u64 = 10 * 60 * 1000;
/// The largest audio file a voice note takes.
pub const MAX_VOICE_BYTES: u64 = 25 * 1024 * 1024;
/// The longest transcript kept, in characters.
pub const MAX_TRANSCRIPT_CHARS: usize = 20_000;
const MS_PER_SECOND: u64 = 1000;
const SECONDS_PER_MINUTE: u64 = 60;

/// A voice note on a post (`extras.voice`, boxed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct VoiceNote {
    /// The audio file, in the media folder.
    pub path: String,
    /// Its media type (`audio/wav`, …).
    pub mime: String,
    /// How long it plays, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
    /// What was said, once transcribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
}

impl VoiceNote {
    /// The file's name, for the card.
    pub fn file_name(&self) -> String {
        Path::new(&self.path).file_name().map_or_else(|| self.path.clone(), |name| name.to_string_lossy().into_owned())
    }

    /// Whether it is a WAV file (the app plays those itself).
    pub fn is_wav(&self) -> bool {
        self.mime == mime_of(Path::new("a.wav")).unwrap_or_default()
    }
}

/// The media type of the audio file at `path`, by its extension (`None`: not audio).
pub fn mime_of(path: &Path) -> Option<&'static str> {
    let extension = path.extension()?.to_str()?.to_ascii_lowercase();
    AUDIO_TYPES.iter().find(|(known, _)| *known == extension).map(|(_, mime)| *mime)
}

/// Whether `path` names an audio file a voice note can be made from.
pub fn is_audio_path(path: &Path) -> bool {
    mime_of(path).is_some()
}

/// The extensions of the audio files offered when choosing one.
pub fn audio_extensions() -> Vec<&'static str> {
    AUDIO_TYPES.iter().map(|(extension, _)| *extension).collect()
}

/// `m:ss` for a length in milliseconds.
pub fn duration_label(ms: u64) -> String {
    let seconds = ms / MS_PER_SECOND;
    format!("{}:{:02}", seconds / SECONDS_PER_MINUTE, seconds % SECONDS_PER_MINUTE)
}

/// A transcript as kept: lines trimmed, blank runs collapsed, cut to [`MAX_TRANSCRIPT_CHARS`];
/// `None` when nothing was said.
pub fn clean_transcript(raw: &str) -> Option<String> {
    let lines: Vec<&str> = raw.lines().map(str::trim).filter(|line| !line.is_empty()).collect();
    let text = lines.join("\n");
    (!text.is_empty()).then(|| text.chars().take(MAX_TRANSCRIPT_CHARS).collect())
}

#[cfg(test)]
mod tests;
