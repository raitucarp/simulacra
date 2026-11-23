//! N11, N12, N14 in exports: a post's retraction or correction, its voice note (a link to the
//! audio, bundled, and the transcript as a quotation) and its drawing (its SVG, bundled), as
//! Markdown. Nothing for a locked post, which travels as its seal only.

use crate::domain::post::correction::CorrectionKind;
use crate::domain::post::voice::duration_label;
use crate::domain::Post;

/// The voice note and drawing files `post` carries: (path, description), bundled with it.
pub(super) fn post_media(post: &Post) -> Vec<(String, String)> {
    if post.is_locked() {
        return Vec::new();
    }
    let voice = post.extras.voice.as_ref().map(|voice| (voice.path.clone(), voice.file_name()));
    let ink = post.extras.ink.as_ref().and_then(|ink| ink.svg_path.clone()).map(|path| (path, String::new()));
    voice.into_iter().chain(ink).collect()
}

/// The Markdown of `post`'s mark, voice note and drawing; `media_path` names each file in the bundle.
pub(super) fn care_block(post: &Post, media_path: &dyn Fn(&str) -> String) -> String {
    if post.is_locked() {
        return String::new();
    }
    let mark = post.extras.correction.as_ref().map(|mark| {
        let label = if mark.kind == CorrectionKind::Retracted { "Retracted" } else { "Corrected" };
        format!("> **{label}:** {}\n\n", mark.reason)
    });
    let voice = post.extras.voice.as_ref().map(|voice| {
        let length = voice.duration_ms.map(|ms| format!(" ({})", duration_label(ms))).unwrap_or_default();
        let transcript = voice.transcript.as_ref().map(|text| format!("> {}\n\n", text.replace('\n', "\n> "))).unwrap_or_default();
        format!("[Voice note{length}]({})\n\n{transcript}", media_path(&voice.path))
    });
    let ink = post.extras.ink.as_ref().and_then(|ink| ink.svg_path.as_ref()).map(|path| format!("![Drawing]({})\n\n", media_path(path)));
    [mark, voice, ink].into_iter().flatten().collect()
}
