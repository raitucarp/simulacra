//! The Licences screen (decision #19): the only place that names the projects behind the
//! icon sets. Each downloaded set is listed with its project, site, licence and the licence
//! text stored with it; sets not downloaded are not listed.

use super::common::{card, field_title, muted_note, section, section_label, spaced_row};
use crate::domain::icons::IconSet;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use rust_i18n::t;

/// One downloaded set and the licence text stored with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LicenceView {
    pub set: &'static IconSet,
    pub text: String,
}

/// The ICON SETS section of the Licences screen.
pub fn render_licences(theme: &ThemeTokens, licences: &[LicenceView]) -> Div {
    // With nothing downloaded the card says so alone: "These sets were downloaded…" would
    // introduce a list that is not there.
    let intro = if licences.is_empty() { t!("licences.icons.none") } else { t!("licences.icons.note") };
    let mut body = card(theme).gap(theme.spacing_md).child(muted_note(theme, intro));
    for licence in licences {
        body = body.child(entry(theme, licence));
    }
    section(spaced_row().child(section_label(theme, t!("licences.icons.title"))), body, theme)
}

fn entry(theme: &ThemeTokens, licence: &LicenceView) -> Div {
    let set = licence.set;
    let heading = format!("{} · {}", set.project, set.licence);
    let text = div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(licence.text.clone());
    div().flex().flex_col().gap(theme.spacing_2xs).child(field_title(theme, heading)).child(muted_note(theme, set.project_url.to_string())).child(text)
}
