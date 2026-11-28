//! Settings › Appearance › Illustrations (WS5): a picker card per pack (name, description,
//! coverage and three samples drawn through that pack, Classic filling the gaps), the pack
//! in use ticked; "Install pack from file…"; and the packs offered for download (none until
//! one is released), with progress and Cancel while one downloads.

mod card;
mod downloads;
mod errors;

pub use errors::pack_error_text;

use super::common::{card as surface, muted_note, section, section_label};
use super::SetDownload;
use crate::theme::ThemeTokens;
use crate::ui::illustration::IllustrationKind;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Something asked of the packs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackAction {
    /// Use pack `id` for every illustration.
    Select(String),
    InstallFromFile,
    /// Ask before removing installed pack `id`, then remove it (or not).
    AskRemove(String),
    ConfirmRemove(String),
    CancelRemove,
    /// Download listed pack `id`, or stop the download under way.
    Download(String),
    CancelDownload,
}

pub type PackCallback = Rc<dyn Fn(PackAction, &mut Window, &mut App) + 'static>;

/// One pack's picker card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackCardView {
    pub id: String,
    pub name: String,
    pub description: String,
    /// Classic illustrations the pack covers, of how many.
    pub covered: usize,
    pub total: usize,
    /// Asset paths (and their kind) shown on the card.
    pub samples: Vec<(String, IllustrationKind)>,
    pub active: bool,
    /// Installed (not built in), so it can be removed.
    pub removable: bool,
}

/// One pack offered for download.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadableView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub bytes: u64,
    pub installed: bool,
}

/// What the section shows.
#[derive(Clone)]
pub struct IllustrationPacksProps {
    pub cards: Vec<PackCardView>,
    pub downloadable: Vec<DownloadableView>,
    pub download: Option<SetDownload>,
    /// The installed pack awaiting a confirmed removal.
    pub confirm_remove: Option<String>,
    /// An install from a file is under way.
    pub installing: bool,
    /// The last failure, already translated.
    pub error: Option<String>,
    pub on_action: PackCallback,
}

/// The ILLUSTRATIONS section.
pub fn render_illustration_packs(theme: &ThemeTokens, props: IllustrationPacksProps) -> Div {
    let cards = props.cards.iter().map(|view| card::render(theme, view, &props));
    let mut body = surface(theme).gap(theme.stack_gap).child(muted_note(theme, t!("packs.note").to_string()));
    if let Some(error) = &props.error {
        body = body.child(div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(error.clone()));
    }
    body = body
        .child(div().flex().flex_wrap().gap(theme.spacing_sm).children(cards))
        .child(downloads::install_row(theme, &props))
        .child(downloads::downloadable(theme, &props));
    section(section_label(theme, t!("packs.title").to_string()), body, theme)
}
