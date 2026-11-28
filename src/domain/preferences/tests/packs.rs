//! WS5: the illustration pack preference defaults to Flat, survives a round trip, and an id
//! that cannot name a pack becomes Classic.

use crate::domain::packs::{CLASSIC_PACK_ID, FLAT_PACK_ID};
use crate::domain::preferences::*;

#[test]
fn the_illustration_pack_defaults_to_flat() {
    assert_eq!(Preferences::default().appearance.illustration_pack, FLAT_PACK_ID);
    let older: AppearancePrefs = serde_json::from_str(r#"{"reduced_motion":true}"#).expect("an older file");
    assert_eq!(older.illustration_pack, FLAT_PACK_ID, "a file saved before packs gets the default");
    let empty: Preferences = serde_json::from_str("{}").expect("empty");
    assert_eq!(empty.appearance.illustration_pack, FLAT_PACK_ID);
}

#[test]
fn a_chosen_pack_round_trips_and_unusable_ids_become_classic() {
    let mut prefs = Preferences::default();
    prefs.appearance.illustration_pack = "clay3d".into();
    let back: Preferences = serde_json::from_str(&serde_json::to_string(&prefs).unwrap()).unwrap();
    assert_eq!(back.validated().appearance.illustration_pack, "clay3d", "an uninstalled pack is kept (it draws Classic until installed)");
    for bad in ["", "  ", "../escape", "Flat Pack"] {
        prefs.appearance.illustration_pack = bad.into();
        assert_eq!(prefs.clone().validated().appearance.illustration_pack, CLASSIC_PACK_ID, "{bad:?}");
    }
    prefs.appearance.illustration_pack = " cozy ".into();
    assert_eq!(prefs.validated().appearance.illustration_pack, "cozy");
}
