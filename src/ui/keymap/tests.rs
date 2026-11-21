use super::*;
use crate::ui::features::command_palette::{spec, CommandId};
use gpui_kit::{KeyBindingContextPredicate, Keystroke, OperatingSystem};
use std::collections::HashSet;

fn english(key: &str) -> String {
    rust_i18n::t!(key, locale = "en").to_string()
}

#[test]
fn every_chord_parses_on_every_desktop_os() {
    for os in [OperatingSystem::Windows, OperatingSystem::Mac, OperatingSystem::Linux] {
        for shortcut in SHORTCUTS {
            assert!(Keystroke::parse_with_os(shortcut.keys, os).is_ok(), "unparsable chord {} on {os:?}", shortcut.keys);
        }
    }
}

#[test]
fn every_scope_is_a_valid_context_predicate() {
    for shortcut in SHORTCUTS {
        assert!(KeyBindingContextPredicate::parse(context_of(shortcut.scope)).is_ok(), "{:?}", shortcut.scope);
    }
}

#[test]
fn no_chord_is_bound_twice_in_one_scope() {
    let mut seen = HashSet::new();
    for shortcut in SHORTCUTS {
        let scope = context_of(shortcut.scope);
        assert!(seen.insert((scope, shortcut.keys)), "{} bound twice in {scope}", shortcut.keys);
    }
}

#[test]
fn the_brief_shortcuts_are_present() {
    let has = |keys: &str, id: CommandId| SHORTCUTS.iter().any(|s| s.keys == keys && s.target == ShortcutTarget::Command(id) && s.scope == KeyScope::Shell);
    assert!(has("secondary-p", CommandId::OpenPalette));
    assert!(has("secondary-k", CommandId::OpenPalette), "Ctrl+K is an alias");
    assert!(has("secondary-shift-p", CommandId::OpenCommands));
    assert!(has("secondary-/", CommandId::ShowShortcuts));
    assert!(has("secondary-b", CommandId::ToggleSidebar));
}

#[test]
fn the_composer_submit_chord_is_kept() {
    let submit = SHORTCUTS.iter().find(|s| s.target == ShortcutTarget::SubmitPost).expect("submit chord");
    assert_eq!((submit.keys, submit.scope), ("secondary-enter", KeyScope::Input));
    assert_eq!(context_of(KeyScope::Input), "Input");
}

#[test]
fn format_chords_are_bound_in_text_fields() {
    use crate::domain::post::format::FormatMark;
    let chords: Vec<(&str, FormatMark)> = SHORTCUTS
        .iter()
        .filter_map(|s| match s.target {
            ShortcutTarget::Format(mark) => Some((s.keys, mark)),
            _ => None,
        })
        .collect();
    let expected = [("secondary-b", FormatMark::Bold), ("secondary-i", FormatMark::Italic), ("secondary-u", FormatMark::Underline), ("secondary-e", FormatMark::Code), ("secondary-shift-s", FormatMark::Spoiler)];
    assert_eq!(chords, expected);
    assert!(SHORTCUTS.iter().filter(|s| matches!(s.target, ShortcutTarget::Format(_))).all(|s| s.scope == KeyScope::Input));
}

/// Ctrl/Cmd+B is also the shell's "toggle sidebar". In a text field the format binding is
/// deeper, so it is tried first; the shell's stays next in line (a field that does not
/// handle formatting lets it through). With no field focused only the shell's matches.
/// The toolkit registers its own bindings first, as here; none of them uses these chords.
#[test]
fn format_chords_do_not_clash_with_the_shell_keymap() {
    use crate::ui::features::composer::FormatBold;
    use gpui_kit::{KeyBinding, KeyContext, Keymap};
    let keys = chord_for(ShortcutTarget::Format(crate::domain::post::format::FormatMark::Bold)).expect("bold chord");
    assert_eq!(Some(keys), shortcut_for(CommandId::ToggleSidebar), "the chords overlap on purpose");
    assert_eq!(keys, "secondary-b");
    let keys = if cfg!(target_os = "macos") { "cmd-b" } else { "ctrl-b" };
    let shell = KeyBinding::new(keys, RunCommand(CommandId::ToggleSidebar), Some(SHELL_KEY_CONTEXT));
    let field = KeyBinding::new(keys, FormatBold, Some(context_of(KeyScope::Input)));
    let keymap = Keymap::new(vec![shell, field]);
    let chord = [Keystroke::parse(keys).expect("chord")];
    let context = |name: &str| KeyContext::parse(name).expect("context");
    let (in_field, _) = keymap.bindings_for_input(&chord, &[context(SHELL_KEY_CONTEXT), context("Input")]);
    assert!(in_field[0].action().partial_eq(&FormatBold), "a focused field formats first");
    assert!(in_field.iter().any(|b| b.action().partial_eq(&RunCommand(CommandId::ToggleSidebar))), "the shell binding stays next");
    let (no_field, _) = keymap.bindings_for_input(&chord, &[context(SHELL_KEY_CONTEXT)]);
    assert_eq!(no_field.len(), 1);
    assert!(no_field[0].action().partial_eq(&RunCommand(CommandId::ToggleSidebar)));
    for mark_keys in ["secondary-i", "secondary-u", "secondary-e", "secondary-shift-s"] {
        assert!(!SHORTCUTS.iter().any(|s| s.keys == mark_keys && s.scope == KeyScope::Shell), "{mark_keys} is free in the shell");
    }
}

#[test]
fn palette_keys_outrank_the_kit_input_context() {
    assert!(context_of(KeyScope::Palette).ends_with("> Input"));
    assert!(context_of(KeyScope::Palette).starts_with(PALETTE_KEY_CONTEXT));
}

#[test]
fn shown_shortcut_is_the_first_binding() {
    assert_eq!(shortcut_for(CommandId::OpenPalette), Some("secondary-p"));
    assert_eq!(shortcut_for(CommandId::ReplayWelcome), None);
}

#[test]
fn every_bound_command_can_run() {
    for shortcut in SHORTCUTS {
        if let ShortcutTarget::Command(id) = shortcut.target {
            assert!(spec(id).is_runnable(), "{id:?} is bound but planned");
        }
    }
}

#[test]
fn secondary_resolves_per_platform() {
    assert_eq!(key_caps("secondary-shift-p", false), vec![KeyCap::Ctrl, KeyCap::Shift, KeyCap::Char("P".into())]);
    assert_eq!(key_caps("secondary-shift-p", true), vec![KeyCap::Command, KeyCap::Shift, KeyCap::Char("P".into())]);
    assert_eq!(key_caps("secondary-/", false), vec![KeyCap::Ctrl, KeyCap::Char("/".into())]);
    assert_eq!(key_caps("shift-tab", false), vec![KeyCap::Shift, KeyCap::Tab]);
    assert_eq!(key_caps("up", false), vec![KeyCap::Up]);
}

#[test]
fn sheet_merges_aliases_and_covers_every_shortcut() {
    let rows: Vec<SheetRow> = ShortcutGroup::ALL.into_iter().flat_map(sheet_rows).collect();
    let palette = rows.iter().find(|row| row.target == ShortcutTarget::Command(CommandId::OpenPalette)).expect("palette row");
    assert_eq!(palette.chords, vec!["secondary-p", "secondary-k"]);
    let listed: usize = rows.iter().map(|row| row.chords.len()).sum();
    assert_eq!(listed, SHORTCUTS.len(), "every binding appears once on the sheet");
}

#[test]
fn every_sheet_label_and_group_title_is_translated() {
    for group in ShortcutGroup::ALL {
        assert_ne!(english(group.title_key()), group.title_key());
    }
    for key in ["keymap.title", "keymap.subtitle", "keymap.key.ctrl", "keymap.key.shift", "keymap.key.alt", "keymap.key.enter", "keymap.key.escape", "keymap.key.tab", "keymap.key.space"] {
        assert_ne!(english(key), key);
    }
    for key in SHORTCUTS.iter().filter_map(|s| target_label_key(s.target)) {
        assert_ne!(english(key), key, "untranslated shortcut label");
    }
}
