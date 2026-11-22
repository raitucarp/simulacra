//! The icon tree converter (S11): golden SVG output, untrusted parts dropped, export names.

use crate::domain::icons::{icon_name, parse_module, svg_attr_name, to_svg, ConvertError, IconNode};

const MODULE: &str = r#"// THIS FILE IS AUTO GENERATED
import { GenIcon } from '../lib/index.mjs';
export function FiZoomOut (props) {
  return GenIcon({"tag":"svg","attr":{"viewBox":"0 0 24 24","fill":"none","stroke":"currentColor","strokeWidth":"2","strokeLinecap":"round","strokeLinejoin":"round"},"child":[{"tag":"circle","attr":{"cx":"11","cy":"11","r":"8"},"child":[]},{"tag":"line","attr":{"x1":"8","y1":"11","x2":"14","y2":"11"},"child":[]}]})(props);
};
export function FiFlag (props) {
  return GenIcon({"tag":"svg","attr":{"viewBox":"0 0 24 24"},"child":[{"tag":"path","attr":{"fillRule":"evenodd","d":"M4 15s1-1 4-1"},"child":[]}]})(props);
};
export function Fi (props) { return 1; };
"#;

const ZOOM_OUT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"/><line x1="8" x2="14" y1="11" y2="11"/></svg>"#;
const FLAG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path d="M4 15s1-1 4-1" fill-rule="evenodd"/></svg>"#;

#[test]
fn a_module_converts_to_golden_svgs() {
    let icons = parse_module(MODULE);
    assert_eq!(icons.iter().map(|i| i.export.as_str()).collect::<Vec<_>>(), ["FiZoomOut", "FiFlag"], "a block without a tree is skipped");
    assert_eq!(to_svg(&icons[0].tree).unwrap(), ZOOM_OUT);
    assert_eq!(to_svg(&icons[1].tree).unwrap(), FLAG);
    for svg in [ZOOM_OUT, FLAG] {
        assert!(resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()).is_ok(), "parses: {svg}");
    }
}

#[test]
fn unknown_elements_scripts_handlers_and_outside_references_are_dropped() {
    let tree: IconNode = serde_json::from_str(
        r##"{"tag":"svg","attr":{"viewBox":"0 0 8 8","onload":"x()"},"child":[
            {"tag":"script","attr":{},"child":[]},
            {"tag":"foreignObject","attr":{},"child":[{"tag":"path","attr":{"d":"M0 0"},"child":[]}]},
            {"tag":"path","attr":{"d":"M1 1\"/><x","fill":"url(https://x.example/a)","clipPath":"url(#c-1)","href":"#a"},"child":[]}]}"##,
    )
    .unwrap();
    let svg = to_svg(&tree).unwrap();
    assert_eq!(svg, r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 8 8"><path clip-path="url(#c-1)" d="M1 1&quot;/&gt;&lt;x"/></svg>"##);
    assert_eq!(to_svg(&IconNode { tag: "g".into(), ..IconNode::default() }), Err(ConvertError::NotSvg));
    assert_eq!(to_svg(&IconNode { tag: "svg".into(), ..IconNode::default() }), Err(ConvertError::NoViewBox));
}

#[test]
fn export_names_become_kebab_case_names_within_their_set() {
    assert_eq!(icon_name("fi", "FiZoomOut").as_deref(), Some("zoom-out"));
    assert_eq!(icon_name("fa6", "FaRegAddressBook").as_deref(), Some("reg-address-book"));
    assert_eq!(icon_name("fa", "Fa500Px").as_deref(), Some("500-px"));
    assert_eq!(icon_name("hi2", "HiOutlineArrowUp").as_deref(), Some("outline-arrow-up"));
    assert_eq!(icon_name("fi", "Fi"), None);
    assert_eq!(svg_attr_name("strokeLinejoin"), "stroke-linejoin");
    assert_eq!(svg_attr_name("viewBox"), "viewBox");
}
