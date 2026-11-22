//! A set's licence text (S12): the package names each bundled project with its site and
//! licence in its own licence file; the stored text is that entry, the set's licence link,
//! and the package's own licence.

use super::catalog::IconSet;

/// The licence text stored with `set`, from the package's licence file `package_licence`.
pub fn set_attribution(set: &IconSet, package_licence: &str) -> String {
    let entry = package_licence
        .split("\n\n")
        .find(|block| block.trim_start().starts_with(&format!("{} - ", set.project)))
        .map(str::trim)
        .map(str::to_string)
        .unwrap_or_else(|| format!("{} - {}\nLicense: {} {}", set.project, set.project_url, set.licence, set.licence_url));
    let own = package_licence.split("\n---").next().unwrap_or_default().trim();
    format!("{entry}\n\n{}\n\n{own}\n", set.licence_url)
}
