//! The draft as prompt lines (`path: value`), so suggested values can fit what is already set.

use super::super::choice::current_keys;
use super::super::param::Param;
use crate::domain::Profile;

/// Longest text shown per field in the summary.
const MAX_SUMMARY_CHARS: usize = 220;
const KEY_SEPARATOR: &str = ", ";
const ELLIPSIS: &str = "…";

fn shortened(text: &str) -> String {
    let flat = text.split_whitespace().collect::<Vec<_>>().join(" ");
    match flat.char_indices().nth(MAX_SUMMARY_CHARS) {
        Some((at, _)) => format!("{}{ELLIPSIS}", &flat[..at]),
        None => flat,
    }
}

/// The current value of `param` as one line of text, `None` when blank or not shown.
fn value(param: Param, profile: &Profile) -> Option<String> {
    let cfg = &profile.config;
    let text = match param {
        Param::Text(field) => shortened(field.get(profile)),
        Param::Score(field) => field.get(profile).to_string(),
        Param::Hour(field) => field.get(profile).to_string(),
        Param::Count(field) => field.get(profile).to_string(),
        Param::Trait(field) => field.read(cfg).key().to_string(),
        Param::Set(_) | Param::Toggle(_) | Param::Engine(_) => current_keys(param, profile).join(KEY_SEPARATOR),
        Param::OptionalText(_) | Param::Avatar(_) | Param::PhotoZoom => String::new(),
    };
    (!text.trim().is_empty()).then_some(text)
}

/// One `- path: value` line per parameter that has a value, the targets left out.
pub(super) fn draft_lines(profile: &Profile, targets: &[Param]) -> Vec<String> {
    Param::all()
        .into_iter()
        .filter(|param| !targets.contains(param))
        .filter_map(|param| value(param, profile).map(|text| format!("- {}: {text}", param.path())))
        .collect()
}
