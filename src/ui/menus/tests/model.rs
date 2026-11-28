//! The menu model on its own: building, walking and the structure rules.

use super::super::rules::{label_keys, violations, MAX_SUBMENU, MAX_TOP_LEVEL};
use super::super::{MenuItem, MenuSection, MenuSpec, MenuText, Submenu};

fn item(id: &str) -> MenuItem {
    MenuItem::new(id.to_string(), MenuText::plain(id.to_string()))
}

fn items(prefix: &str, count: usize) -> Vec<MenuItem> {
    (0..count).map(|n| item(&format!("{prefix}{n}"))).collect()
}

#[test]
fn empty_sections_and_submenus_are_left_out() {
    let empty = Submenu::new("sub", MenuText::plain("Sub"), MenuSpec::new());
    let spec = MenuSpec::new().section(MenuSection::new()).section(MenuSection::new().entry(item("a")).entry(empty));
    assert_eq!(spec.sections.len(), 1);
    assert_eq!(spec.entries().count(), 1, "a submenu with nothing in it is not shown");
    assert!(MenuSpec::of(Vec::<MenuItem>::new()).is_empty());
}

#[test]
fn paths_name_every_level() {
    let inner = Submenu::new("inner", MenuText::plain("Inner"), MenuSpec::of([item("leaf")]));
    let outer = Submenu::new("outer", MenuText::plain("Outer"), MenuSpec::of([inner]));
    let spec = MenuSpec::of([outer]);
    assert_eq!(spec.path_of("leaf"), Some(vec!["Outer".to_string(), "Inner".to_string(), "leaf".to_string()]));
    assert_eq!(spec.depth(), 3);
    assert_eq!(spec.menus().len(), 3);
    assert!(spec.reaches("leaf") && !spec.reaches("outer"), "submenus are not actions");
}

#[test]
fn a_tidy_menu_has_no_violations() {
    let sub = Submenu::new("sub", MenuText::plain("Sub"), MenuSpec::new().section(MenuSection::new().entries(items("s", 3))).section(MenuSection::new().entry(item("gone").destructive())));
    let spec = MenuSpec::new().section(MenuSection::new().entries(items("t", 4)).entry(sub)).section(MenuSection::new().entry(item("delete").destructive()));
    assert_eq!(violations(&spec), Vec::<String>::new());
}

#[test]
fn the_rules_catch_long_menus_misplaced_danger_and_repeats() {
    let crowded = MenuSpec::of(items("t", MAX_TOP_LEVEL + 1));
    assert!(violations(&crowded).iter().any(|v| v.contains("top level")));
    let long = Submenu::new("long", MenuText::plain("Long"), MenuSpec::of(items("l", MAX_SUBMENU + 1)));
    assert!(violations(&MenuSpec::of([long])).iter().any(|v| v.contains("not labelled")));
    let early = MenuSpec::new().section(MenuSection::new().entry(item("x").destructive())).section(MenuSection::new().entry(item("y")));
    assert!(violations(&early).iter().any(|v| v.contains("not in the last section")));
    let mixed = MenuSpec::of([item("x").destructive(), item("y")]);
    assert!(violations(&mixed).iter().any(|v| v.contains("follows a destructive")));
    let twice = MenuSpec::of([item("same"), MenuItem::new("other", MenuText::plain("same"))]);
    assert!(violations(&twice).iter().any(|v| v.contains("twice")));
    let elsewhere = Submenu::new("sub", MenuText::plain("Sub"), MenuSpec::of([item("dup")]));
    assert!(violations(&MenuSpec::of([MenuItem::new("dup", MenuText::plain("Top")).into(), super::super::MenuEntry::from(elsewhere)])).iter().any(|v| v.contains("in both")));
}

#[test]
fn labelled_sections_let_a_submenu_run_longer() {
    let sections = MenuSpec::new().section(MenuSection::labelled("menus.post.section.review").entries(items("a", 6))).section(MenuSection::labelled("menus.post.section.questions").entries(items("b", 6)));
    assert_eq!(violations(&sections), Vec::<String>::new(), "a sectioned picker may list more at its top level");
    let spec = MenuSpec::of([Submenu::new("sub", MenuText::plain("Sub"), sections)]);
    assert_eq!(violations(&spec), Vec::<String>::new());
    assert_eq!(label_keys(&spec), ["menus.post.section.review", "menus.post.section.questions"]);
}

#[test]
fn translated_labels_keep_their_key() {
    let text = MenuText::key("post.menu_reply");
    assert_eq!(text.key.as_ref().map(|key| key.as_ref()), Some("post.menu_reply"));
    assert_ne!(text.text.as_ref(), "post.menu_reply");
    assert_eq!(MenuText::plain("Reading list").key, None);
}
