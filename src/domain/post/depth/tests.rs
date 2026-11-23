use super::*;
use crate::domain::data::history::PostRecord;

#[test]
fn a_rendering_s_depth_is_kept_in_its_version_record() {
    assert_eq!(Depth::ALL.iter().map(|d| d.key()).collect::<Vec<_>>(), vec!["simpler", "expert"]);
    let record = PostRecord { rendering: Some(Depth::Expert), ..PostRecord::default() };
    let json = record.to_json();
    assert!(json.contains("\"rendering\":\"expert\""), "{json}");
    assert_eq!(PostRecord::parse(&json).and_then(|r| r.rendering), Some(Depth::Expert));
    assert_eq!(PostRecord::parse("{}").and_then(|r| r.rendering), None, "records kept before renderings have none");
}
