//! Settings › Symbols (S13): every icon set of the library with a neutral label (its style
//! and number, never its project's name, decision #19), its licence, icon count and size,
//! and Download or Remove; while one downloads, its progress and Cancel.

use super::common::{card, field_title, muted_note, section, section_label, spaced_row};
use crate::domain::icons::{icon_set, set_number, IconSet, ICON_SETS};
use crate::theme::ThemeTokens;
use crate::ui::features::import::file_size;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::base::Disableable;
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const DOWNLOAD_ID: &str = "settings-symbols-download-";
const REMOVE_ID: &str = "settings-symbols-remove-";
const CANCEL_ID: &str = "settings-symbols-cancel";

/// Something asked of a set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolSetAction {
    Download(String),
    Remove(String),
    Cancel,
}

pub type SymbolSetCallback = Rc<dyn Fn(SymbolSetAction, &mut Window, &mut App) + 'static>;

/// The download under way: its set and how far it is (`0.0..=1.0`, `None` while unknown).
#[derive(Debug, Clone, PartialEq)]
pub struct SetDownload {
    pub set: String,
    pub fraction: Option<f32>,
}

/// What the pane shows.
#[derive(Clone)]
pub struct SymbolSetsProps {
    /// Installed sets by id, with the bytes they take.
    pub installed: Vec<(String, u64)>,
    pub download: Option<SetDownload>,
    /// The last failure, already translated.
    pub error: Option<String>,
    pub on_action: SymbolSetCallback,
}

/// A set's label: its style and its number (`Line icons · set 9`).
pub fn set_label(set: &IconSet) -> String {
    let style = crate::ui::features::data::tr(&format!("symbols.style.{}", set.style.key()));
    t!("symbols.set_label", style = style, number = set_number(set.id).unwrap_or_default()).to_string()
}

/// The ICON SETS section.
pub fn render_symbol_sets(theme: &ThemeTokens, props: SymbolSetsProps) -> Div {
    let package = file_size(crate::icons::PACKAGE_BYTES);
    let mut body = card(theme).gap(theme.spacing_sm).child(muted_note(theme, t!("symbols.sets.note", size = package)));
    if let Some(error) = &props.error {
        body = body.child(div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(error.clone()));
    }
    for set in ICON_SETS {
        body = body.child(set_row(theme, set, &props));
    }
    section(spaced_row().child(section_label(theme, t!("symbols.sets.title"))), body, theme)
}

fn set_row(theme: &ThemeTokens, set: &IconSet, props: &SymbolSetsProps) -> Div {
    let installed = props.installed.iter().find(|(id, _)| id == set.id).map(|(_, bytes)| *bytes);
    let bytes = installed.unwrap_or(set.bytes);
    let details = t!("symbols.sets.details", licence = set.licence, count = set.icons, size = file_size(bytes)).to_string();
    let info = div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_2xs).child(field_title(theme, set_label(set))).child(muted_note(theme, details));
    let downloading = props.download.as_ref().filter(|job| job.set == set.id);
    let action = match (downloading, installed) {
        (Some(job), _) => progress(theme, job, props.on_action.clone()),
        (None, Some(_)) => button(REMOVE_ID, set.id, LucideIcon::Trash, t!("symbols.sets.remove").to_string(), SymbolSetAction::Remove(set.id.into()), props),
        (None, None) => button(DOWNLOAD_ID, set.id, LucideIcon::Download, t!("symbols.sets.download").to_string(), SymbolSetAction::Download(set.id.into()), props),
    };
    div().flex().items_center().gap(theme.spacing_sm).py(theme.spacing_2xs).child(info).child(action)
}

fn button(prefix: &str, id: &str, icon: LucideIcon, label: String, action: SymbolSetAction, props: &SymbolSetsProps) -> Div {
    let on_action = props.on_action.clone();
    let busy = props.download.is_some();
    let button = Button::new(SharedString::from(format!("{prefix}{id}"))).ghost().xsmall().icon(icon).label(label).disabled(busy).on_click(move |_, window, cx| on_action(action.clone(), window, cx));
    div().flex_none().child(button)
}

/// The bar of a running download, its percentage and Cancel.
fn progress(theme: &ThemeTokens, job: &SetDownload, on_action: SymbolSetCallback) -> Div {
    let fraction = job.fraction.unwrap_or_default().clamp(0.0, 1.0);
    let percent = t!("symbols.sets.progress", percent = (fraction * 100.0).round() as u32).to_string();
    let track = div().w(theme.symbol_progress_width).h(theme.symbol_progress_height).rounded(theme.radius_full).bg(theme.border_subtle).child(div().h_full().w(theme.symbol_progress_width * fraction).rounded(theme.radius_full).bg(theme.accent_brand));
    let cancel = Button::new(CANCEL_ID).ghost().xsmall().label(t!("symbols.sets.cancel").to_string()).on_click(move |_, window, cx| on_action(SymbolSetAction::Cancel, window, cx));
    div().flex_none().flex().items_center().gap(theme.spacing_xs).child(track).child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(percent)).child(cancel)
}

/// The set an action names, when it is one of the library's.
pub fn action_set(action: &SymbolSetAction) -> Option<&'static IconSet> {
    match action {
        SymbolSetAction::Download(id) | SymbolSetAction::Remove(id) => icon_set(id),
        SymbolSetAction::Cancel => None,
    }
}

#[cfg(test)]
mod tests {
    use super::set_label;
    use crate::domain::icons::ICON_SETS;

    #[test]
    fn every_set_has_a_neutral_numbered_label() {
        let labels: Vec<String> = ICON_SETS.iter().map(set_label).collect();
        for (label, set) in labels.iter().zip(ICON_SETS) {
            assert!(!label.contains("symbols.") && label.contains(&crate::domain::icons::set_number(set.id).unwrap().to_string()), "{label}");
            assert!(!label.to_lowercase().contains(&set.project.to_lowercase()), "{label} names its project");
        }
        let mut unique = labels.clone();
        unique.sort();
        unique.dedup();
        assert_eq!(unique.len(), labels.len());
    }

    /// Decision #19: no translated text names an icon project (only the Licences screen does,
    /// from the catalogue).
    #[test]
    fn no_locale_names_an_icon_project() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("locales");
        for entry in std::fs::read_dir(dir).expect("locales").flatten() {
            let raw = std::fs::read_to_string(entry.path()).expect("locale").to_lowercase();
            let text: Vec<&str> = raw.lines().filter_map(|line| line.split_once(": ").map(|(_, value)| value)).collect();
            for set in ICON_SETS {
                let name = set.project.to_lowercase();
                let named = text.iter().any(|value| value.match_indices(&name).any(|(at, _)| !value[at + name.len()..].starts_with(char::is_alphanumeric)));
                assert!(!named, "{:?} names {}", entry.file_name(), set.project);
            }
            let text = text.join("\n");
            assert!(!text.contains("react-icons"), "{:?} names the package", entry.file_name());
        }
    }
}
