//! JSON import/export of palettes through the clipboard or a native file dialog.

use super::super::common::{card, section, section_label};
use super::consts::{COPY_JSON_ID, OPEN_FILE_ID, PASTE_JSON_ID, SAVE_FILE_ID, THEME_FILE_EXTENSIONS, THEME_FILE_SUFFIX};
use super::handlers::{act, PanelHandle};
use super::AppearancePanel;
use crate::theme::ThemePalette;
use gpui_kit::component::button::Button;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{Sizable, WindowExt};
use gpui_kit::*;
use rust_i18n::t;

/// Notification id so repeated imports/exports replace the previous toast.
struct ThemeTransferNotice;

/// The import/export section.
pub(super) fn render(pane: &AppearancePanel, handle: &PanelHandle) -> Div {
    let theme = &pane.tokens;
    let button = |id: &'static str, key: &str, action: fn(&mut AppearancePanel, &mut Window, &mut Context<AppearancePanel>)| {
        let run = act(handle, action);
        Button::new(id).small().outline().label(t!(key).to_string()).on_click(move |_, window, cx| run(window, cx))
    };
    let body = card(theme).child(
        div()
            .flex()
            .flex_wrap()
            .gap(theme.spacing_sm)
            .child(button(COPY_JSON_ID, "appearance.copy_json", AppearancePanel::copy_json))
            .child(button(PASTE_JSON_ID, "appearance.paste_json", AppearancePanel::paste_json))
            .child(button(SAVE_FILE_ID, "appearance.save_file", AppearancePanel::save_file))
            .child(button(OPEN_FILE_ID, "appearance.open_file", AppearancePanel::open_file)),
    );
    section(section_label(theme, t!("appearance.transfer_title").to_string()), body)
}

fn toast(window: &mut Window, cx: &mut App, note: Notification) {
    window.push_notification(note.id::<ThemeTransferNotice>(), cx);
}

impl AppearancePanel {
    /// The active palette (with edits) as pretty JSON.
    fn export_json(&self) -> String {
        self.settings.effective_palette().to_json_pretty().unwrap_or_default()
    }

    fn copy_json(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(self.export_json()));
        toast(window, cx, Notification::success(t!("appearance.copied").to_string()));
    }

    fn paste_json(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        match cx.read_from_clipboard().and_then(|item| item.text()) {
            Some(json) => self.import_json(&json, window, cx),
            None => toast(window, cx, Notification::error(t!("appearance.clipboard_empty").to_string())),
        }
    }

    /// Imports one palette JSON document as a new custom palette and activates it.
    pub(super) fn import_json(&mut self, json: &str, window: &mut Window, cx: &mut Context<Self>) {
        match ThemePalette::from_json(json) {
            Ok(palette) => {
                let note = t!("appearance.imported", name = palette.name.as_str()).to_string();
                self.edit(window, cx, |s| {
                    s.import(palette);
                });
                toast(window, cx, Notification::success(note));
            }
            Err(err) => {
                let note = t!("appearance.import_failed", error = err.to_string()).to_string();
                toast(window, cx, Notification::error(note));
            }
        }
    }

    fn save_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let json = self.export_json();
        let file_name = format!("{}{THEME_FILE_SUFFIX}", self.settings.preset_id);
        let filter = t!("appearance.file_filter").to_string();
        cx.spawn_in(window, async move |pane, cx| {
            let dialog = rfd::AsyncFileDialog::new().set_file_name(file_name).add_filter(filter, &THEME_FILE_EXTENSIONS);
            let Some(file) = dialog.save_file().await else { return };
            let written = std::fs::write(file.path(), json);
            let _ = pane.update_in(cx, |_, window, cx| match written {
                Ok(()) => toast(window, cx, Notification::success(t!("appearance.saved").to_string())),
                Err(err) => toast(window, cx, Notification::error(err.to_string())),
            });
        })
        .detach();
    }

    fn open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let filter = t!("appearance.file_filter").to_string();
        cx.spawn_in(window, async move |pane, cx| {
            let dialog = rfd::AsyncFileDialog::new().add_filter(filter, &THEME_FILE_EXTENSIONS);
            let Some(file) = dialog.pick_file().await else { return };
            let read = std::fs::read_to_string(file.path());
            let _ = pane.update_in(cx, |pane, window, cx| match read {
                Ok(json) => pane.import_json(&json, window, cx),
                Err(err) => toast(window, cx, Notification::error(err.to_string())),
            });
        })
        .detach();
    }
}
