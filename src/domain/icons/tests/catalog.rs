//! The pinned package's sets (S12/S13): unique ids, licences and neutral labels; and what the
//! store's index and licence texts hold.

use crate::domain::icons::*;

#[test]
fn every_set_has_a_unique_plain_id_a_licence_and_a_place() {
    let mut ids: Vec<&str> = ICON_SETS.iter().map(|set| set.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ICON_SETS.len());
    for set in ICON_SETS {
        assert!(set.id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit()), "{}", set.id);
        assert!(!set.licence.is_empty() && set.icons > 0 && set.bytes > 0, "{}", set.id);
        assert!(set.project_url.starts_with("http") && set.licence_url.starts_with("http"), "{}", set.id);
        assert_eq!(icon_set(set.id), Some(set));
    }
    assert_eq!(set_number(ICON_SETS[0].id), Some(1));
    assert_eq!(icon_set("../x"), None);
}

#[test]
fn the_index_keeps_one_entry_per_set_sorted() {
    let stored = |id: &str| StoredSet { id: id.into(), icons: 1, ..StoredSet::default() };
    let mut index = StoreIndex::default();
    index.put(stored("lu"));
    index.put(stored("fi"));
    index.put(StoredSet { icons: 2, ..stored("lu") });
    assert_eq!(index.sets.iter().map(|s| (s.id.as_str(), s.icons)).collect::<Vec<_>>(), [("fi", 1), ("lu", 2)]);
    assert!(index.take("fi") && !index.take("fi"));
    let json = serde_json::to_string(&index).unwrap();
    assert_eq!(serde_json::from_str::<StoreIndex>(&json).unwrap(), index);
    assert_eq!(serde_json::from_str::<StoreIndex>("{}").unwrap(), StoreIndex::default());
}

#[test]
fn a_set_licence_names_its_project_then_the_package_licence() {
    let set = icon_set("fi").unwrap();
    let package = "Copyright 2018 Someone\n\nPermission granted.\n\n---\nIcons are taken from the other projects\n\nFeather - https://feathericons.com/\nLicense: MIT https://example.org/feather\n\nLucide - https://lucide.dev/\nLicense: ISC https://example.org/lucide\n";
    let text = set_attribution(set, package);
    assert!(text.starts_with("Feather - https://feathericons.com/\nLicense: MIT"), "{text}");
    assert!(text.contains("Copyright 2018 Someone") && !text.contains("Lucide"), "{text}");
    let fallback = set_attribution(set, "");
    assert!(fallback.starts_with(&format!("{} - {}", set.project, set.project_url)), "{fallback}");
}
