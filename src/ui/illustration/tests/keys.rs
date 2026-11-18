use super::super::keys::*;
use crate::ui::assets::{template_text, EmbeddedAssets};
use std::collections::HashSet;

fn check<T: Copy>(all: &[T], path: fn(T) -> String, dir: &str) {
    let paths: HashSet<String> = all.iter().map(|k| path(*k)).collect();
    assert_eq!(paths.len(), all.len(), "duplicate keys in {dir}");
    for p in &paths {
        assert!(template_text(p).is_some(), "missing asset {p}");
    }
    let prefix = format!("{ILLUSTRATION_ROOT}/{dir}/");
    for asset in EmbeddedAssets::iter().filter(|a| a.starts_with(&prefix)) {
        assert!(paths.contains(asset.as_ref()), "{asset} has no typed key");
    }
}

#[test]
fn every_typed_key_maps_to_an_existing_asset_and_back() {
    check(StepKey::ALL, StepKey::path, "steps");
    check(ClassKey::ALL, ClassKey::path, "classes");
    check(GuidePose::ALL, GuidePose::path, "guide");
    check(EmptyKey::ALL, EmptyKey::path, "empty");
    check(SpotKey::ALL, SpotKey::path, "spot");
    check(NoticeKey::ALL, NoticeKey::path, "notice");
}

#[test]
fn canvases_match_the_templates() {
    let pairs = [
        (StepKey::Origin.path(), StepKey::KIND),
        (ClassKey::StoicSage.path(), ClassKey::KIND),
        (GuidePose::Idle.path(), GuidePose::KIND),
        (EmptyKey::NoPosts.path(), EmptyKey::KIND),
        (SpotKey::Ideas.path(), SpotKey::KIND),
        (NoticeKey::NoKey.path(), NoticeKey::KIND),
    ];
    for (path, kind) in pairs {
        let text = template_text(&path).unwrap();
        assert_eq!(super::super::template::view_box_size(&text), Some(kind.canvas()), "{path}");
    }
}
