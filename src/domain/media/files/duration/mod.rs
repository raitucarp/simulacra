//! L7: how long a video file plays, read from its container headers without decoding a frame
//! (no codec is bundled): MP4 / QuickTime boxes, Matroska / WebM EBML and AVI's RIFF header.

mod bytes;
mod ebml;
mod mp4;
mod riff;

pub use bytes::{ByteSource, Bytes};

/// Longest duration believed (anything longer is a misread header).
const MAX_SECONDS: f64 = 100.0 * 3600.0;

/// The playing time of the file in `source` (typed `mime`), in whole seconds.
pub fn media_duration(mime: &str, source: &mut dyn ByteSource) -> Option<u32> {
    let seconds = match mime {
        "video/mp4" | "video/quicktime" => mp4::duration(source),
        "video/x-matroska" | "video/webm" => ebml::duration(source),
        "video/x-msvideo" => riff::duration(source),
        _ => None,
    }?;
    (seconds.is_finite() && seconds > 0.0 && seconds < MAX_SECONDS).then(|| seconds.round() as u32)
}

#[cfg(test)]
mod tests;
