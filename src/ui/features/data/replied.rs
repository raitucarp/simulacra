//! The quiet "replied to v1.x" hint on a reply whose parent has had a major edit since (V12),
//! so a reader knows the reply answered an earlier version.

use super::post_menu::DataControls;
use crate::domain::data::history::SemVer;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

/// "replied to v1.x" for a reply that answered `version`.
pub fn replied_text(version: SemVer) -> String {
    let major = t!("data.version_major", major = version.major).to_string();
    t!("data.replied_to", version = major).to_string()
}

/// The hint, or `None` while the reply answers the version its parent shows.
pub fn render_replied_hint(controls: &DataControls, theme: &ThemeTokens) -> Option<AnyElement> {
    let version = controls.replied_to?;
    let hint = div().flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(replied_text(version));
    Some(hint.into_any_element())
}
