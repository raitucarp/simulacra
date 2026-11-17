use super::layouts::{restored_size, PanelLayouts};

#[test]
fn layouts_roundtrip_through_json() {
    let mut layouts = PanelLayouts::default();
    assert!(layouts.set("shell", vec![240.0, 900.0, 320.0]));
    assert!(layouts.set("timeline", vec![520.0, 480.0]));
    let restored = PanelLayouts::from_json(&layouts.to_json());
    assert_eq!(restored, layouts);
    assert_eq!(restored.size_of("shell", 2), Some(320.0));
    assert_eq!(restored.size_of("shell", 3), None);
    assert_eq!(restored.size_of("missing", 0), None);
}

#[test]
fn corrupt_json_gives_no_layouts() {
    for raw in ["", "not json", "[1,2,3]", "42", "{\"shell\":"] {
        assert!(PanelLayouts::from_json(raw).is_empty(), "{raw:?}");
    }
}

#[test]
fn unreadable_entries_are_skipped() {
    let raw = r#"{"good":[200,400],"text":"wide","mixed":[200,"x"],"negative":[-5,10],"zero":[0],"empty":[]}"#;
    let layouts = PanelLayouts::from_json(raw);
    assert_eq!(layouts.get("good"), Some([200.0, 400.0].as_slice()));
    for id in ["text", "mixed", "negative", "zero", "empty"] {
        assert_eq!(layouts.get(id), None, "{id}");
    }
}

#[test]
fn set_ignores_unchanged_and_unusable_sizes() {
    let mut layouts = PanelLayouts::default();
    assert!(layouts.set("a", vec![100.0, 200.0]));
    assert!(!layouts.set("a", vec![100.0, 200.0]));
    assert!(!layouts.set("a", vec![f32::NAN, 200.0]));
    assert!(!layouts.set("a", vec![]));
    assert!(!layouts.set("a", vec![0.0, 200.0]));
    assert_eq!(layouts.get("a"), Some([100.0, 200.0].as_slice()));
    assert!(layouts.remove("a"));
    assert!(!layouts.remove("a"));
    assert!(layouts.is_empty());
}

#[test]
fn clear_forgets_every_layout() {
    let mut layouts = PanelLayouts::default();
    layouts.set("a", vec![1.0]);
    layouts.set("b", vec![2.0]);
    layouts.clear();
    assert!(layouts.is_empty());
    assert_eq!(layouts.to_json(), "{}");
}

#[test]
fn restored_size_clamps_to_the_range() {
    assert_eq!(restored_size(Some(300.0), 240.0, 180.0, 380.0), 300.0);
    assert_eq!(restored_size(Some(50.0), 240.0, 180.0, 380.0), 180.0);
    assert_eq!(restored_size(Some(900.0), 240.0, 180.0, 380.0), 380.0);
    assert_eq!(restored_size(None, 240.0, 180.0, 380.0), 240.0);
    assert_eq!(restored_size(Some(f32::INFINITY), 240.0, 180.0, 380.0), 240.0);
    assert_eq!(restored_size(Some(-10.0), 240.0, 180.0, 380.0), 240.0);
    // An inverted range resolves to its minimum.
    assert_eq!(restored_size(Some(300.0), 240.0, 400.0, 200.0), 400.0);
}
