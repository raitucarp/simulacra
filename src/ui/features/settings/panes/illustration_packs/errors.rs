//! Why a pack was refused, in words.

use crate::domain::packs::PackError;
use rust_i18n::t;

/// The translated reason of `error` (it completes "The pack could not be installed: …").
pub fn pack_error_text(error: &PackError) -> String {
    let detail = error.detail();
    match error {
        PackError::Checksum(_) => t!("packs.error.checksum", file = detail),
        PackError::Missing(_) => t!("packs.error.missing", file = detail),
        PackError::Manifest(_) => t!("packs.error.manifest", detail = detail),
        PackError::Invalid(..) => t!("packs.error.invalid", detail = detail),
        PackError::Archive(_) => t!("packs.error.archive"),
        PackError::TooLarge => t!("packs.error.too_large"),
        PackError::BuiltIn(_) => t!("packs.error.built_in", id = detail),
        PackError::Io(_) => t!("packs.error.io", detail = detail),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::pack_error_text;
    use crate::domain::packs::{PackError, PackRule};

    /// Every refusal reads as a sentence naming what was wrong, never a raw key.
    #[test]
    fn every_refusal_has_a_translated_reason() {
        let errors = [
            PackError::Checksum("spot/ideas.webp".into()),
            PackError::Missing("guide/cheer.webp".into()),
            PackError::Manifest("eof".into()),
            PackError::Invalid(PackRule::Key, "../x".into()),
            PackError::Archive("bad zip".into()),
            PackError::TooLarge,
            PackError::BuiltIn("flat".into()),
            PackError::Io("disk full".into()),
        ];
        for error in errors {
            let text = pack_error_text(&error);
            assert!(!text.is_empty() && !text.contains("packs.error"), "{error:?}: {text}");
            let shows_detail = !matches!(error, PackError::Archive(_));
            assert!(!shows_detail || text.contains(error.detail()), "{error:?} names {:?}: {text}", error.detail());
        }
    }
}
