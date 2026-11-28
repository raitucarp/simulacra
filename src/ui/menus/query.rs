//! Walking a spec: where an action sits ("Organize › Garden › Grow to Sapling"), every item
//! of a tree, every submenu. Used by the structure and inventory tests and the docs mapping.

use super::entry::MenuEntry;
use super::item::MenuItem;
use super::spec::MenuSpec;

/// The labels from the top level down to an item.
pub type MenuPath = Vec<String>;

impl MenuSpec {
    /// Every item of the tree with its path, depth first, top to bottom.
    pub fn items(&self) -> Vec<(MenuPath, &MenuItem)> {
        let mut found = Vec::new();
        collect_items(self, &mut Vec::new(), &mut found);
        found
    }

    /// The path of the first item whose id is `id`.
    pub fn path_of(&self, id: &str) -> Option<MenuPath> {
        self.items().into_iter().find(|(_, item)| item.id.as_ref() == id).map(|(path, _)| path)
    }

    /// Whether an item with id `id` is reachable.
    pub fn reaches(&self, id: &str) -> bool {
        self.path_of(id).is_some()
    }

    /// The first item whose id is `id`.
    pub fn item(&self, id: &str) -> Option<&MenuItem> {
        self.items().into_iter().find(|(_, item)| item.id.as_ref() == id).map(|(_, item)| item)
    }

    /// This menu and every submenu below it, each with the path that opens it.
    pub fn menus(&self) -> Vec<(MenuPath, &MenuSpec)> {
        let mut found = vec![(Vec::new(), self)];
        collect_menus(self, &mut Vec::new(), &mut found);
        found
    }

    /// How many levels the tree has (1: no submenus).
    pub fn depth(&self) -> usize {
        1 + self.entries().filter_map(|entry| match entry {
            MenuEntry::Submenu(submenu) => Some(submenu.spec.depth()),
            MenuEntry::Item(_) => None,
        }).max().unwrap_or(0)
    }
}

fn collect_items<'a>(spec: &'a MenuSpec, path: &mut MenuPath, out: &mut Vec<(MenuPath, &'a MenuItem)>) {
    for entry in spec.entries() {
        path.push(entry.label().text.to_string());
        match entry {
            MenuEntry::Item(item) => out.push((path.clone(), item)),
            MenuEntry::Submenu(submenu) => collect_items(&submenu.spec, path, out),
        }
        path.pop();
    }
}

fn collect_menus<'a>(spec: &'a MenuSpec, path: &mut MenuPath, out: &mut Vec<(MenuPath, &'a MenuSpec)>) {
    for entry in spec.entries() {
        if let MenuEntry::Submenu(submenu) = entry {
            path.push(submenu.label.text.to_string());
            out.push((path.clone(), &submenu.spec));
            collect_menus(&submenu.spec, path, out);
            path.pop();
        }
    }
}
