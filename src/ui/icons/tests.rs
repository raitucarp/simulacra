use super::*;
use gpui_kit::assets::IconName;
use std::collections::HashSet;

/// A path missing from the bundled catalog renders as a blank icon at runtime,
/// which happens silently when Lucide renames an icon upstream.
#[test]
fn every_lucide_icon_path_exists_in_bundled_catalog() {
    let bundled: HashSet<SharedString> = IconName::ALL.iter().map(|n| n.path()).collect();
    let missing: Vec<_> = LucideIcon::ALL
        .iter()
        .filter(|icon| !bundled.contains(&icon.path()))
        .map(|icon| (*icon, icon.path()))
        .collect();
    assert!(missing.is_empty(), "icons missing from bundled assets: {missing:?}");
}

#[test]
fn all_lists_every_variant_once() {
    let unique: HashSet<_> = LucideIcon::ALL.iter().collect();
    assert_eq!(unique.len(), LucideIcon::ALL.len());
}
