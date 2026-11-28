//! Illustration packs in the shell (WS5): what Settings › Appearance › Illustrations shows
//! (a card per pack, the downloadable list) and keeping the UI thread's pack registry in
//! step with the preference and the pack store.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::packs::{builtin_downloadable_packs, is_built_in, CLASSIC_PACK_ID};
use crate::ui::features::settings::{DownloadableView, IllustrationPacksProps, PackCardView};
use crate::ui::illustration::pack::{active_pack_id, available_packs, configure_packs, coverage, set_active_pack, RasterPack};
use crate::ui::illustration::IllustrationKind;
use gpui_kit::Context;
use rust_i18n::t;
use std::rc::Rc;

/// Samples of a pack that names none.
const DEFAULT_SAMPLES: [&str; 3] = ["guide/cheer", "spot/ideas", "empty/no_posts"];
/// Packs whose name and description are translated (others show their manifest's).
const KNOWN_PACKS: [&str; 5] = ["classic", "flat", "clay3d", "cozy", "isometric"];

/// The packs' state in the shell.
#[derive(Default)]
pub struct PacksHost {
    /// The download under way.
    pub download: Option<super::handlers::PackDownloadJob>,
    /// An install from a file is under way.
    pub installing: bool,
    /// The installed pack awaiting a confirmed removal.
    pub confirm_remove: Option<String>,
    /// The last failure, translated.
    pub error: Option<String>,
}

/// The translated name and description of pack `id`, or the manifest's.
fn labels(id: &str, name: &str, description: &str) -> (String, String) {
    if !KNOWN_PACKS.contains(&id) {
        return (name.to_string(), description.to_string());
    }
    let (name_key, description_key) = (format!("packs.name.{id}"), format!("packs.description.{id}"));
    (t!(name_key.as_str()).to_string(), t!(description_key.as_str()).to_string())
}

/// The asset path and kind of each sample key.
fn samples(keys: &[String]) -> Vec<(String, IllustrationKind)> {
    let keys: Vec<&str> = if keys.is_empty() { DEFAULT_SAMPLES.to_vec() } else { keys.iter().map(String::as_str).take(DEFAULT_SAMPLES.len()).collect() };
    keys.into_iter().map(|key| (crate::domain::packs::asset_path_of_key(key), IllustrationKind::of_key(key))).collect()
}

/// The card of Classic, then of every available raster pack.
fn cards() -> Vec<PackCardView> {
    let active = active_pack_id();
    let (_, total) = coverage(None);
    let (name, description) = labels(CLASSIC_PACK_ID, "", "");
    let classic = PackCardView { id: CLASSIC_PACK_ID.into(), name, description, covered: total, total, samples: samples(&[]), active: active == CLASSIC_PACK_ID, removable: false };
    let raster = available_packs().into_iter().map(|pack: std::sync::Arc<RasterPack>| {
        let manifest = &pack.manifest;
        let (name, description) = labels(&manifest.id, &manifest.name, &manifest.description);
        let (covered, total) = coverage(Some(pack.as_ref()));
        PackCardView { id: manifest.id.clone(), name, description, covered, total, samples: samples(&manifest.samples), active: active == manifest.id, removable: !is_built_in(&manifest.id) }
    });
    std::iter::once(classic).chain(raster).collect()
}

/// Every pack offered for download, marked when already installed.
fn downloadable() -> Vec<DownloadableView> {
    let installed: Vec<String> = available_packs().iter().map(|pack| pack.id().to_string()).collect();
    builtin_downloadable_packs()
        .into_iter()
        .map(|pack| {
            let (name, description) = labels(&pack.id, &pack.name, &pack.description);
            DownloadableView { installed: installed.contains(&pack.id), id: pack.id, name, description, version: pack.version, bytes: pack.bytes }
        })
        .collect()
}

impl AppView {
    /// What Settings › Appearance › Illustrations shows.
    pub(super) fn illustration_packs_props(&self, bind: &Binder) -> IllustrationPacksProps {
        IllustrationPacksProps {
            cards: cards(),
            downloadable: downloadable(),
            download: self.packs.download.as_ref().map(|job| job.view()),
            confirm_remove: self.packs.confirm_remove.clone(),
            installing: self.packs.installing,
            error: self.packs.error.clone(),
            on_action: Rc::new(bind.with(Self::on_pack_action)),
        }
    }

    /// Draws illustrations from the preferred pack (after the preference changed).
    pub(super) fn sync_illustration_pack(&mut self, cx: &mut Context<Self>) {
        for stale in set_active_pack(&self.state.preferences().appearance.illustration_pack) {
            stale.remove_asset(cx);
        }
    }

    /// Reads the pack store again (after an install or a removal) and re-activates the
    /// preferred pack.
    pub(super) fn reload_illustration_packs(&mut self, cx: &mut Context<Self>) {
        for stale in configure_packs(&self.state.packs_dir(), &self.state.preferences().appearance.illustration_pack) {
            stale.remove_asset(cx);
        }
    }
}
