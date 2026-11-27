//! Localized texts and art of the image ideas sheet.

use super::model::{IdeasDestination, IdeasFailure};
use crate::domain::image_prompt::ImageTarget;
use crate::ui::illustration::NoticeKey;
use rust_i18n::t;

/// Locale key prefix of the sheet.
const PREFIX: &str = "image_ideas";

/// `image_ideas.<key>` in the current locale.
pub fn tr(key: &str) -> String {
    let full = format!("{PREFIX}.{key}");
    t!(full.as_str()).to_string()
}

/// Why it failed, in words.
pub fn failure_text(failure: &IdeasFailure) -> String {
    match failure {
        IdeasFailure::NoEngine => tr("failure.no_engine"),
        IdeasFailure::NoImageKey => tr("failure.no_image_key"),
        IdeasFailure::NoTextKey => tr("failure.no_text_key"),
        IdeasFailure::Budget => tr("failure.budget"),
        IdeasFailure::Offline => tr("failure.offline"),
        IdeasFailure::Failed(detail) => t!("image_ideas.failure.failed", detail = detail).to_string(),
    }
}

/// The art beside a failure.
pub fn failure_art(failure: &IdeasFailure) -> NoticeKey {
    match failure {
        IdeasFailure::NoEngine | IdeasFailure::NoImageKey | IdeasFailure::NoTextKey => NoticeKey::NoKey,
        IdeasFailure::Offline => NoticeKey::Offline,
        IdeasFailure::Budget | IdeasFailure::Failed(_) => NoticeKey::Error,
    }
}

/// Whether Settings › Images fixes it.
pub fn fixed_in_image_settings(failure: &IdeasFailure) -> bool {
    matches!(failure, IdeasFailure::NoEngine | IdeasFailure::NoImageKey)
}

/// The subtitle under the sheet's title.
pub fn subtitle(target: ImageTarget, about: &str) -> String {
    match target {
        // A profile not named yet reads "An avatar for Unnamed thinker", not "An avatar for".
        ImageTarget::Avatar => {
            let name = if about.trim().is_empty() { t!("forge.ui.unnamed").to_string() } else { about.to_string() };
            t!("image_ideas.subtitle_avatar", name = name).to_string()
        }
        ImageTarget::Post => t!("image_ideas.subtitle_post", source = about).to_string(),
    }
}

/// The label of a result's button.
pub fn use_label(destination: &IdeasDestination) -> String {
    match destination {
        IdeasDestination::Composer => tr("attach"),
        IdeasDestination::Forge | IdeasDestination::Profile(_) => tr("set_avatar"),
        IdeasDestination::Locus(..) => tr("use_cue"),
    }
}
