//! Reading one icon set's module (`<set>/index.mjs` in the package, S11): a list of
//! `export function <Name> (props) { return GenIcon(<tree>)(props); };` blocks. The module is
//! only ever read as text: each block's name and the JSON tree inside `GenIcon(…)`.

use super::tree::IconNode;

const EXPORT: &str = "export function ";
const GEN_ICON: &str = "GenIcon(";

/// One icon of a module: its exported name (`FiZoomOut`) and its tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleIcon {
    pub export: String,
    pub tree: IconNode,
}

/// Every icon of `module`, in file order. Blocks that do not read as an icon are skipped.
pub fn parse_module(module: &str) -> Vec<ModuleIcon> {
    module.split(EXPORT).skip(1).filter_map(parse_block).collect()
}

fn parse_block(block: &str) -> Option<ModuleIcon> {
    let export: String = block.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_').collect();
    if export.is_empty() {
        return None;
    }
    let json = &block[block.find(GEN_ICON)? + GEN_ICON.len()..];
    let tree = serde_json::Deserializer::from_str(json).into_iter::<IconNode>().next()?.ok()?;
    Some(ModuleIcon { export, tree })
}

/// The icon's name inside its set: the export without the set's prefix, in kebab case
/// (`FiZoomOut` in set `fi` → `zoom-out`, `Fa500Px` in `fa` → `500-px`). `None` when nothing
/// is left.
pub fn icon_name(set: &str, export: &str) -> Option<String> {
    let prefix: String = set.chars().take_while(char::is_ascii_alphabetic).collect();
    let rest = match export.get(..prefix.len()) {
        Some(head) if head.eq_ignore_ascii_case(&prefix) => &export[prefix.len()..],
        _ => export,
    };
    let name = kebab(rest);
    (!name.is_empty()).then_some(name)
}

/// `ZoomOut2X` → `zoom-out-2-x`: a dash before every capital that follows a lower-case
/// letter or digit, and between letters and digits.
fn kebab(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + 4);
    let mut previous: Option<char> = None;
    for c in text.chars().filter(|c| c.is_ascii_alphanumeric()) {
        let boundary = previous.is_some_and(|p| (c.is_ascii_uppercase() && (p.is_ascii_lowercase() || p.is_ascii_digit())) || (c.is_ascii_digit() != p.is_ascii_digit()));
        if boundary {
            out.push('-');
        }
        out.push(c.to_ascii_lowercase());
        previous = Some(c);
    }
    out
}
