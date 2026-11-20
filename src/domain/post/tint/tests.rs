use super::*;
use crate::domain::post::PostExtras;

#[test]
fn tints_have_stable_keys() {
    let keys: Vec<&str> = Tint::ALL.iter().map(|tint| tint.key()).collect();
    assert_eq!(keys, ["amber", "rose", "sky", "mint", "violet", "slate"]);
    for &tint in Tint::ALL {
        assert_eq!(Tint::from_key(tint.key()), Some(tint));
    }
}

#[test]
fn extras_store_the_tint_by_key() {
    let extras = PostExtras { tint: Some(Tint::Mint), ..PostExtras::default() };
    let json = serde_json::to_string(&extras).expect("serialized");
    assert_eq!(json, r#"{"tint":"mint"}"#);
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("read back"), extras);
    assert!(!extras.is_empty());
    assert!(!extras.has_details(), "a tint alone is nothing to show behind the post");
}

#[test]
fn extras_without_a_tint_are_unchanged() {
    assert_eq!(serde_json::to_string(&PostExtras::default()).expect("serialized"), "{}");
    let old: PostExtras = serde_json::from_str(r#"{"mood":"calm"}"#).expect("older extras load");
    assert_eq!((old.tint, old.mood.as_deref()), (None, Some("calm")));
}

#[test]
fn unknown_or_malformed_tints_read_as_none() {
    for json in [r#"{"tint":"chartreuse","mood":"calm"}"#, r#"{"tint":7,"mood":"calm"}"#, r#"{"tint":null,"mood":"calm"}"#] {
        let extras: PostExtras = serde_json::from_str(json).unwrap_or_else(|e| panic!("{json}: {e}"));
        assert_eq!((extras.tint, extras.mood.as_deref()), (None, Some("calm")), "{json}");
    }
}
