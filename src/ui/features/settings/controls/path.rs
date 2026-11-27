//! A vault folder: its path in monospace and a button opening it.

use super::super::catalog::{DataFolder, SettingRow};
use super::super::row::{element_id_of, RowContext};
use super::action::label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const OPEN_PART: &str = "open";
/// Longest path shown; a longer one keeps its end, from a folder boundary.
const PATH_MAX_CHARS: usize = 56;
const ELLIPSIS: char = '…';
/// How the home folder is written at the start of a path.
const HOME_MARK: &str = "~";
const SEPARATORS: [char; 2] = ['\\', '/'];

/// The user's home folder, from the environment.
pub(in crate::ui::features::settings) fn home_dir() -> Option<String> {
    std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")).ok()
}

/// `path` with the home folder written `~`, so the readout keeps to what tells folders
/// apart (and leaves the account name out of screenshots) (pure).
pub(in crate::ui::features::settings) fn home_relative(path: &str, home: Option<&str>) -> String {
    match home.filter(|home| !home.is_empty()).and_then(|home| path.strip_prefix(home)) {
        Some(rest) if rest.is_empty() || rest.starts_with(SEPARATORS) => format!("{HOME_MARK}{rest}"),
        _ => path.to_string(),
    }
}

/// `path` shortened to `max` characters by dropping its start: the end names the folder and
/// file, so it is kept whole from a separator when one leaves a folder and a file (pure).
pub(in crate::ui::features::settings) fn elide_start(path: &str, max: usize) -> String {
    let chars: Vec<char> = path.chars().collect();
    if chars.len() <= max {
        return path.to_string();
    }
    let tail: String = chars[chars.len() - max.saturating_sub(1)..].iter().collect();
    let cut = tail.find(SEPARATORS).filter(|&at| tail[at + 1..].contains(SEPARATORS)).unwrap_or(0);
    format!("{ELLIPSIS}{}", &tail[cut..])
}

/// The path readout and its "Open folder" button.
pub(super) fn render(ctx: &RowContext, row: SettingRow, folder: DataFolder) -> AnyElement {
    let theme = ctx.theme;
    let path = match folder {
        DataFolder::Database => &ctx.data.database_path,
        DataFolder::Media => &ctx.data.media_path,
    };
    let full = home_relative(path, home_dir().as_deref());
    let path = elide_start(&full, PATH_MAX_CHARS);
    let (on_command, command) = (ctx.actions.on_command.clone(), folder.open_command());
    div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_hint)
                .text_color(theme.text_secondary)
                .child(path),
        )
        .child(
            Button::new(element_id_of(row, OPEN_PART))
                .outline()
                .small()
                .icon(Icon::new(LucideIcon::Folder).size(theme.icon_size_sm))
                .label(label(command))
                .tooltip(full)
                .on_click(move |_, window, cx| on_command(command, window, cx)),
        )
        .into_any_element()
}
