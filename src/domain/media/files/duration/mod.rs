//! L7, L8: how long a video or audio file plays, read from its container headers without
//! decoding a frame (no codec is bundled): MP4 / QuickTime / M4A boxes, Matroska / WebM EBML,
//! RIFF (WAV, AVI), FLAC, Ogg (Vorbis, Opus) and MP3 (Xing / VBRI, else the constant bit rate).

mod bytes;
mod ebml;
mod mp3;
mod mp4;
mod riff;
mod xiph;

pub use bytes::{ByteSource, Bytes};

/// Longest duration believed (anything longer is a misread header).
const MAX_SECONDS: f64 = 100.0 * 3600.0;

/// The playing time of the file in `source` (typed `mime`), in whole seconds.
pub fn media_duration(mime: &str, source: &mut dyn ByteSource) -> Option<u32> {
    let seconds = match mime {
        "video/mp4" | "video/quicktime" | "audio/mp4" => mp4::duration(source),
        "video/x-matroska" | "video/webm" | "audio/webm" => ebml::duration(source),
        "video/x-msvideo" | "audio/wav" => riff::duration(source),
        "audio/flac" => xiph::flac_duration(source),
        "audio/ogg" | "video/ogg" => xiph::ogg_duration(source),
        "audio/mpeg" => mp3::duration(source),
        _ => None,
    }?;
    (seconds.is_finite() && seconds > 0.0 && seconds < MAX_SECONDS).then(|| seconds.round() as u32)
}

#[cfg(test)]
mod tests;
