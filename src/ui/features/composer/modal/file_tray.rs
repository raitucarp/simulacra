//! L12: the files attached to the first post, one chip each (type icon, name, size and a
//! remove button), plus a placeholder chip per file still being read.

use super::super::consts::MODAL_FILE_CHIP_ID;
use super::ComposerModalView;
use crate::domain::media::files::file_family;
use crate::domain::media::FileRef;
use crate::import::FormatFamily;
use crate::ui::features::import::file_size;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

impl ComposerModalView {
    /// The file chips, or `None` when no file is attached.
    pub(super) fn file_tray(&self) -> Option<Div> {
        if self.files.is_empty() && self.pending_files == 0 {
            return None;
        }
        let chips = self.files.iter().enumerate().map(|(index, file)| self.file_chip(index, file).into_any_element());
        let pending = (0..self.pending_files).map(|_| self.chip_frame().child(Icon::new(LucideIcon::Paperclip).size(self.theme.icon_size_sm)).into_any_element());
        Some(div().w_full().flex().flex_wrap().gap(self.theme.control_gap).children(chips).children(pending))
    }

    fn chip_frame(&self) -> Div {
        let theme = &self.theme;
        div()
            .max_w_full()
            .px(theme.spacing_sm)
            .py(theme.spacing_2xs)
            .rounded(theme.radius_full)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.bg_app)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .text_size(theme.font_size_xs)
            .text_color(theme.text_secondary)
    }

    fn file_chip(&self, index: usize, file: &FileRef) -> Div {
        let (on_remove, hover) = (self.callbacks.on_remove_file.clone(), self.theme.text_primary);
        let remove = div()
            .id((MODAL_FILE_CHIP_ID, index))
            .flex_none()
            .rounded(self.theme.radius_full)
            .cursor_pointer()
            .text_color(self.theme.text_muted)
            .hover(move |s| s.text_color(hover))
            .tooltip(move |window, cx| Tooltip::new(t!("composer.file_remove").to_string()).build(window, cx))
            .on_click(move |_, window, cx| on_remove(index, window, cx))
            .child(Icon::new(LucideIcon::X).size(self.theme.icon_size_sm));
        self.chip_frame()
            .child(Icon::new(family_icon(file_family(&file.mime))).size(self.theme.icon_size_sm))
            .child(div().min_w_0().truncate().text_color(self.theme.text_primary).child(file.name().to_string()))
            .child(div().flex_none().text_color(self.theme.text_muted).child(file_size(file.size)))
            .child(remove)
    }
}

/// The icon a chip shows for a file's family.
fn family_icon(family: FormatFamily) -> LucideIcon {
    match family {
        FormatFamily::Image => LucideIcon::Image,
        FormatFamily::Video | FormatFamily::Audio => LucideIcon::Play,
        FormatFamily::Text | FormatFamily::Data | FormatFamily::Web | FormatFamily::Pdf | FormatFamily::Book | FormatFamily::Office => LucideIcon::FileText,
        FormatFamily::Archive | FormatFamily::Unknown => LucideIcon::Paperclip,
    }
}
