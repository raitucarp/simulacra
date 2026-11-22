//! Turning one set of the package into files (S11/S12): its module read out of the tarball,
//! every icon converted to SVG under its kebab-case name, and the set's licence: the
//! package's attribution of the set's project, then the package's own licence.

use crate::domain::icons::{icon_name, parse_module, read_files, set_attribution, to_svg, IconSet};
use std::collections::BTreeMap;

/// The package's licence file inside the tarball.
const PACKAGE_LICENCE: &str = "package/LICENSE";

/// One set ready to be written: icon name → SVG, and the licence text.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BuiltSet {
    pub icons: BTreeMap<String, String>,
    pub licence: String,
}

/// The path of `set`'s module inside the tarball.
pub fn module_path(set: &IconSet) -> String {
    format!("package/{}/index.mjs", set.id)
}

/// Builds `set` from the package tarball `package`.
pub fn build_set(package: &[u8], set: &IconSet) -> Result<BuiltSet, String> {
    let module = module_path(set);
    let files = read_files(package, |path| path == module || path == PACKAGE_LICENCE).map_err(|e| format!("{e:?}"))?;
    let source = files.get(&module).ok_or_else(|| format!("the package has no {module}"))?;
    let text = String::from_utf8_lossy(source);
    let mut icons = BTreeMap::new();
    for icon in parse_module(&text) {
        let (Some(name), Ok(svg)) = (icon_name(set.id, &icon.export), to_svg(&icon.tree)) else { continue };
        icons.entry(name).or_insert(svg);
    }
    if icons.is_empty() {
        return Err(format!("{module} holds no icons"));
    }
    let package_licence = files.get(PACKAGE_LICENCE).map(|bytes| String::from_utf8_lossy(bytes).into_owned()).unwrap_or_default();
    Ok(BuiltSet { icons, licence: set_attribution(set, &package_licence) })
}
