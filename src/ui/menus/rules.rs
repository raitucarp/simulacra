//! The structure rules of `docs/decisions/M1-menu-architecture.md`, checked over a spec.
//! Each check returns readable violations ("Organize: 12 entries without labelled sections").

use super::entry::MenuEntry;
use super::query::MenuPath;
use super::spec::MenuSpec;
use std::collections::{HashMap, HashSet};

/// Most entries at a menu's top level (about 8; 10 for a post you wrote, every feature on).
pub const MAX_TOP_LEVEL: usize = 10;
/// Most entries of a submenu, or of one labelled section of a longer one.
pub const MAX_SUBMENU: usize = 10;
/// Most levels of nesting (the top level counts as one).
pub const MAX_DEPTH: usize = 3;

/// Whether every section of `menu` is labelled and within [`MAX_SUBMENU`] (a picker such as
/// the view menu may then list more than the top-level budget).
fn is_sectioned(menu: &MenuSpec) -> bool {
    menu.sections.iter().all(|section| section.label.is_some() && section.entries.len() <= MAX_SUBMENU)
}

/// Every violation of the structure rules in `spec`.
pub fn violations(spec: &MenuSpec) -> Vec<String> {
    let mut found = Vec::new();
    let top = spec.entries().count();
    if top > MAX_TOP_LEVEL && !is_sectioned(spec) {
        found.push(format!("top level: {top} entries (at most {MAX_TOP_LEVEL}, or labelled sections)"));
    }
    if spec.depth() > MAX_DEPTH {
        found.push(format!("{} levels deep (at most {MAX_DEPTH})", spec.depth()));
    }
    for (path, menu) in spec.menus() {
        let name = if path.is_empty() { "top level".to_string() } else { path.join(" › ") };
        found.extend(size_violations(&name, menu, !path.is_empty()));
        found.extend(destructive_violations(&name, menu));
        found.extend(duplicate_labels(&name, menu));
        found.extend(menu.entries().filter(|entry| !entry.is_shown()).map(|entry| format!("{name}: empty submenu {}", entry.label().text)));
    }
    found.extend(repeated_concepts(spec));
    found
}

/// A submenu of more than [`MAX_SUBMENU`] entries needs labelled sections, each within it.
fn size_violations(name: &str, menu: &MenuSpec, is_submenu: bool) -> Vec<String> {
    let total = menu.entries().count();
    if !is_submenu || total <= MAX_SUBMENU || is_sectioned(menu) {
        return Vec::new();
    }
    menu.sections.iter().filter(|section| section.label.is_none() || section.entries.len() > MAX_SUBMENU).map(|section| format!("{name}: {total} entries, section of {} not labelled or too long", section.entries.len())).collect()
}

/// Destructive entries sit in the last section, after every other entry of it.
fn destructive_violations(name: &str, menu: &MenuSpec) -> Vec<String> {
    let last = menu.sections.len().saturating_sub(1);
    let mut found = Vec::new();
    for (index, section) in menu.sections.iter().enumerate() {
        let first_destructive = section.entries.iter().position(MenuEntry::is_destructive);
        let Some(first) = first_destructive else { continue };
        if index != last {
            found.push(format!("{name}: destructive {} is not in the last section", section.entries[first].label().text));
        }
        if let Some(safe) = section.entries[first..].iter().find(|entry| !entry.is_destructive()) {
            found.push(format!("{name}: {} follows a destructive entry", safe.label().text));
        }
    }
    found
}

fn duplicate_labels(name: &str, menu: &MenuSpec) -> Vec<String> {
    let mut seen = HashSet::new();
    menu.entries().filter(|entry| !seen.insert(entry.label().text.clone())).map(|entry| format!("{name}: \"{}\" twice", entry.label().text)).collect()
}

/// No concept twice: an action id sits in one menu only (a family repeats within its menu).
fn repeated_concepts(spec: &MenuSpec) -> Vec<String> {
    let mut homes: HashMap<String, MenuPath> = HashMap::new();
    let mut found = Vec::new();
    for (path, item) in spec.items() {
        let menu: MenuPath = path[..path.len() - 1].to_vec();
        match homes.get(item.id.as_ref()) {
            Some(home) if *home != menu => found.push(format!("{} in both {} and {}", item.id, home.join(" › "), menu.join(" › "))),
            Some(_) => {}
            None => {
                homes.insert(item.id.to_string(), menu);
            }
        }
    }
    found
}

/// Every locale key a label, a reason or a section heading of `spec` uses.
pub fn label_keys(spec: &MenuSpec) -> Vec<String> {
    let mut keys = Vec::new();
    for (_, menu) in spec.menus() {
        keys.extend(menu.sections.iter().filter_map(|section| section.label.as_ref()?.key.as_ref().map(|key| key.to_string())));
        for entry in menu.entries() {
            keys.extend(entry.label().key.as_ref().map(|key| key.to_string()));
            if let MenuEntry::Item(item) = entry {
                keys.extend(item.disabled.as_ref().and_then(|reason| reason.key.as_ref()).map(|key| key.to_string()));
            }
        }
    }
    keys
}
