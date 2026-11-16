use super::*;
use crate::domain::traits::MbtiType;
use crate::domain::TraitOption;
use crate::ui::icons::LucideIcon;

/// Every icon the catalog maps must load through the app source, or it renders blank.
#[test]
fn every_lucide_icon_path_resolves_through_app_assets() {
    let missing: Vec<_> = LucideIcon::ALL
        .iter()
        .filter(|icon| !matches!(AppAssets.load(&icon.path()), Ok(Some(bytes)) if !bytes.is_empty()))
        .map(|icon| icon.path())
        .collect();
    assert!(missing.is_empty(), "icons the app source cannot serve: {missing:?}");
}

#[test]
fn illustrations_are_embedded_and_listed() {
    let path = MbtiType::Intj.illustration();
    let bytes = AppAssets.load(&path).expect("load").expect("embedded");
    assert!(bytes.starts_with(b"<svg"));
    assert!(template_text(&path).is_some_and(|text| text.contains("{{")));
    let listed = AppAssets.list("illustrations/guide/").expect("list");
    assert_eq!(listed.len(), 5, "{listed:?}");
}

#[test]
fn kit_default_icons_still_resolve_and_unknown_paths_do_not_panic() {
    assert!(matches!(AppAssets.load("icons/chevron-down.svg"), Ok(Some(_))));
    assert!(!matches!(AppAssets.load("illustrations/nope.svg"), Ok(Some(_))));
    assert!(template_text("illustrations/nope.svg").is_none());
    assert!(!matches!(AppAssets.load(""), Ok(Some(_))));
}
