//! N11: voice notes at the edges: audio stored in the media folder once per content
//! (`voice/<hash>.<ext>`), the microphone recorder, and playing a note back.
//!
//! - [`record`]: records the default microphone as 16 kHz mono PCM (Windows, through the
//!   system's WinMM wave-in API, so no new dependency); elsewhere recording reports itself
//!   unavailable and the user chooses an audio file instead.

pub mod record;

use crate::domain::post::voice::{mime_of, wav_duration_ms, VoiceNote, MAX_VOICE_BYTES};
use crate::import::parse::sha256_hex;
use std::path::Path;

/// The media folder's voice notes.
const VOICE_DIR: &str = "voice";
/// Hex characters of the content hash used as a file name.
const NAME_HASH_CHARS: usize = 24;
const WAV_EXTENSION: &str = "wav";

/// Writes `bytes` as `voice/<hash>.<extension>` (once per content) and describes the note.
fn store_voice_bytes(bytes: &[u8], extension: &str, media_dir: &Path) -> Result<VoiceNote, String> {
    let mime = mime_of(Path::new(&format!("a.{extension}"))).ok_or_else(|| format!("not an audio file: .{extension}"))?;
    if bytes.len() as u64 > MAX_VOICE_BYTES {
        return Err(format!("audio is larger than {MAX_VOICE_BYTES} bytes"));
    }
    let name: String = sha256_hex(bytes).chars().take(NAME_HASH_CHARS).collect();
    let folder = media_dir.join(VOICE_DIR);
    let path = folder.join(format!("{name}.{}", extension.to_ascii_lowercase()));
    if !path.exists() {
        std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        std::fs::write(&path, bytes).map_err(|e| e.to_string())?;
    }
    let duration_ms = (extension.eq_ignore_ascii_case(WAV_EXTENSION)).then(|| wav_duration_ms(bytes)).flatten();
    Ok(VoiceNote { path: path.display().to_string(), mime: mime.to_string(), duration_ms, transcript: None })
}

/// Stores a recording (WAV bytes) in the media folder.
pub fn store_recording(wav: &[u8], media_dir: &Path) -> Result<VoiceNote, String> {
    store_voice_bytes(wav, WAV_EXTENSION, media_dir)
}

/// Copies the audio file at `source` into the media folder.
pub fn store_voice_file(source: &Path, media_dir: &Path) -> Result<VoiceNote, String> {
    let extension = source.extension().and_then(|ext| ext.to_str()).filter(|_| mime_of(source).is_some()).ok_or("not an audio file")?;
    let size = std::fs::metadata(source).map_err(|e| e.to_string())?.len();
    if size > MAX_VOICE_BYTES {
        return Err(format!("audio is larger than {MAX_VOICE_BYTES} bytes"));
    }
    let bytes = std::fs::read(source).map_err(|e| e.to_string())?;
    store_voice_bytes(&bytes, extension, media_dir)
}

#[cfg(test)]
mod tests;
