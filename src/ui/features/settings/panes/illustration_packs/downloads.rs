//! Getting more packs: "Install pack from file…" and the list of packs offered for download
//! (empty until a pack is released), with a running download's progress and Cancel.

use super::super::common::{field_title, muted_note};
use super::card::button_action;
use super::{DownloadableView, IllustrationPacksProps, PackAction};
use crate::theme::ThemeTokens;
use crate::ui::features::import::file_size;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const INSTALL_ID: &str = "settings-packs-install-file";
const DOWNLOAD_ID: &str = "settings-packs-download-";
const CANCEL_ID: &str = "settings-packs-cancel";

/// The install button and what it accepts.
pub(super) fn install_row(theme: &ThemeTokens, props: &IllustrationPacksProps) -> Div {
    let run = button_action(props, PackAction::InstallFromFile);
    let busy = props.installing || props.download.is_some();
    let button = Button::new(INSTALL_ID).small().outline().icon(LucideIcon::Package).label(t!("packs.install_file").to_string()).disabled(busy).on_click(move |_, window, cx| run(window, cx));
    div().flex().items_center().gap(theme.spacing_sm).child(div().flex_none().child(button)).child(muted_note(theme, t!("packs.install_hint").to_string()).flex_1())
}

/// The DOWNLOADABLE PACKS list, or a note that there are none yet.
pub(super) fn downloadable(theme: &ThemeTokens, props: &IllustrationPacksProps) -> Div {
    let list = div().flex().flex_col().gap(theme.spacing_2xs).child(field_title(theme, t!("packs.downloadable_title").to_string()));
    if props.downloadable.is_empty() {
        return list.child(muted_note(theme, t!("packs.downloadable_empty").to_string()));
    }
    list.children(props.downloadable.iter().map(|pack| row(theme, pack, props)))
}

fn row(theme: &ThemeTokens, pack: &DownloadableView, props: &IllustrationPacksProps) -> Div {
    let details = t!("packs.details", version = pack.version.as_str(), size = file_size(pack.bytes)).to_string();
    let info = div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).child(field_title(theme, pack.name.clone())).child(muted_note(theme, pack.description.clone())).child(muted_note(theme, details));
    let downloading = props.download.as_ref().filter(|job| job.set == pack.id);
    let action = match (downloading, pack.installed) {
        (Some(job), _) => progress(theme, job.fraction, props),
        (None, true) => div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("packs.installed_badge").to_string()),
        (None, false) => {
            let run = button_action(props, PackAction::Download(pack.id.clone()));
            let busy = props.installing || props.download.is_some();
            let button = Button::new(SharedString::from(format!("{DOWNLOAD_ID}{}", pack.id))).ghost().xsmall().icon(LucideIcon::Download).label(t!("packs.download").to_string()).disabled(busy).on_click(move |_, window, cx| run(window, cx));
            div().flex_none().child(button)
        }
    };
    div().flex().items_center().gap(theme.spacing_sm).py(theme.spacing_2xs).child(info).child(action)
}

/// A running download's bar, percentage and Cancel.
fn progress(theme: &ThemeTokens, fraction: Option<f32>, props: &IllustrationPacksProps) -> Div {
    let fraction = fraction.unwrap_or_default().clamp(0.0, 1.0);
    let percent = t!("packs.progress", percent = (fraction * 100.0).round() as u32).to_string();
    let bar = div().h_full().w(theme.symbol_progress_width * fraction).rounded(theme.radius_full).bg(theme.accent_brand);
    let track = div().w(theme.symbol_progress_width).h(theme.symbol_progress_height).rounded(theme.radius_full).bg(theme.border_subtle).child(bar);
    let run = button_action(props, PackAction::CancelDownload);
    let cancel = Button::new(CANCEL_ID).ghost().xsmall().label(t!("packs.cancel_download").to_string()).on_click(move |_, window, cx| run(window, cx));
    div().flex_none().flex().items_center().gap(theme.spacing_xs).child(track).child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(percent)).child(cancel)
}
