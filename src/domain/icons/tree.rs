//! One icon's tree as the package's modules write it (`{"tag", "attr", "child"}`), and its
//! conversion to a standalone SVG file (S11). The trees come from a downloaded package, so
//! they are untrusted: only drawing elements and presentation attributes are kept, every
//! value is escaped, and nothing that could reference or run anything survives.

use serde::Deserialize;
use std::collections::BTreeMap;

/// Elements kept (anything else is dropped with its children).
const KEPT_TAGS: [&str; 15] = [
    "svg", "g", "path", "circle", "ellipse", "rect", "line", "polyline", "polygon", "defs",
    "clipPath", "mask", "linearGradient", "radialGradient", "stop",
];
/// Attributes kept, as the modules spell them (camelCase becomes kebab-case on output).
const KEPT_ATTRS: [&str; 38] = [
    "viewBox", "d", "fill", "fillRule", "fillOpacity", "stroke", "strokeWidth", "strokeLinecap",
    "strokeLinejoin", "strokeMiterlimit", "strokeOpacity", "strokeDasharray", "strokeDashoffset",
    "clipRule", "clipPath", "mask", "opacity", "transform", "cx", "cy", "r", "rx", "ry", "x", "y",
    "x1", "y1", "x2", "y2", "width", "height", "points", "id", "offset", "stopColor",
    "stopOpacity", "gradientUnits", "gradientTransform",
];
/// Attributes whose camelCase spelling is also the SVG spelling.
const VERBATIM_ATTRS: [&str; 3] = ["viewBox", "gradientUnits", "gradientTransform"];
/// The SVG namespace every converted file declares.
const SVG_NS: &str = "http://www.w3.org/2000/svg";
/// Deepest nesting converted (a real icon is a few levels deep).
const MAX_DEPTH: usize = 16;

/// One element of an icon tree.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Default)]
#[serde(default)]
pub struct IconNode {
    pub tag: String,
    pub attr: BTreeMap<String, String>,
    pub child: Vec<IconNode>,
}

/// Why a tree is not an icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvertError {
    /// The root element is not `svg`.
    NotSvg,
    /// The root has no `viewBox`.
    NoViewBox,
}

/// The tree as a standalone SVG document: the root's `viewBox` and drawing attributes, the
/// SVG namespace, and every kept descendant.
pub fn to_svg(root: &IconNode) -> Result<String, ConvertError> {
    if root.tag != "svg" {
        return Err(ConvertError::NotSvg);
    }
    if !root.attr.contains_key("viewBox") {
        return Err(ConvertError::NoViewBox);
    }
    let mut out = String::new();
    write_node(root, 0, &mut out);
    Ok(out)
}

fn write_node(node: &IconNode, depth: usize, out: &mut String) {
    if depth > MAX_DEPTH || !KEPT_TAGS.contains(&node.tag.as_str()) {
        return;
    }
    out.push('<');
    out.push_str(&node.tag);
    if depth == 0 {
        out.push_str(&format!(" xmlns=\"{SVG_NS}\""));
    }
    for (name, value) in node.attr.iter().filter(|(name, value)| KEPT_ATTRS.contains(&name.as_str()) && !value.contains("url(") || is_local_ref(name, value)) {
        out.push_str(&format!(" {}=\"{}\"", svg_attr_name(name), escape(value)));
    }
    if node.child.is_empty() {
        out.push_str("/>");
        return;
    }
    out.push('>');
    for child in &node.child {
        write_node(child, depth + 1, out);
    }
    out.push_str(&format!("</{}>", node.tag));
}

/// Whether `value` is a kept attribute pointing at an element of the same file (`url(#id)`).
fn is_local_ref(name: &str, value: &str) -> bool {
    KEPT_ATTRS.contains(&name) && value.strip_prefix("url(#").and_then(|rest| rest.strip_suffix(')')).is_some_and(|id| id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_'))
}

/// `strokeWidth` → `stroke-width`; `viewBox` and the gradient attributes stay as they are.
pub fn svg_attr_name(name: &str) -> String {
    if VERBATIM_ATTRS.contains(&name) {
        return name.to_string();
    }
    let mut out = String::with_capacity(name.len() + 4);
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            out.push('-');
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// `value` safe inside a double-quoted attribute.
fn escape(value: &str) -> String {
    value.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}
