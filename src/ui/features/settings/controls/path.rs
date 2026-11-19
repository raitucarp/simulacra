//! A vault folder: its path in monospace and a button opening it.

use super::super::catalog::{DataFolder, SettingRow};
use super::super::row::{element_id_of, RowContext};
use super::action::label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const OPEN_PART: &str = "open";
/// Longest path shown; longer ones keep their start and their end.
const PATH_MAX_CHARS: usize = 44;
/// Characters kept from the start of an elided path (the rest of the budget goes to its end).
const PATH_HEAD_CHARS: usize = 12;
const ELLIPSIS: char = '…';

/// `text` shortened to `max` characters by eliding its middle (pure).
pub(in crate::ui::features::settings) fn elide_middle(text: &str, max: usize, head: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= max {
        return text.to_string();
    }
    let tail = max.saturating_sub(head + 1);
    chars[..head].iter().chain(std::iter::once(&ELLIPSIS)).chain(&chars[chars.len() - tail..]).collect()
}

/// The path readout and its "Open folder" button.
pub(super) fn render(ctx: &RowContext, row: SettingRow, folder: DataFolder) -> AnyElement {
    let theme = ctx.theme;
    let path = match folder {
        DataFolder::Database => &ctx.data.database_path,
        DataFolder::Media => &ctx.data.media_path,
    };
    let path = elide_middle(path, PATH_MAX_CHARS, PATH_HEAD_CHARS);
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
                .on_click(move |_, window, cx| on_command(command, window, cx)),
        )
        .into_any_element()
}
