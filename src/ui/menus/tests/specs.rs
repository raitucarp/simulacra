//! U3.8: the app's menus keep the structure rules and are translated, and the revamp's
//! promises hold (New › More holds the palette-only creation commands; views are sectioned).

use super::assert_tidy;
use crate::domain::views::{PostView, ViewScope};
use crate::domain::Network;
use crate::i18n::Language;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::features::popovers::{language_menu_spec, network_menu_spec};
use crate::ui::features::sidebar_left::new_menu_spec;
use crate::ui::features::views::{view_menu_spec, PickView};
use crate::ui::menus::COMMAND_PREFIX;
use gpui_kit::px;
use std::rc::Rc;

fn command(id: CommandId) -> String {
    format!("{COMMAND_PREFIX}{}", id.wire_key())
}

#[test]
fn the_new_menu_is_sectioned_and_reaches_every_creation_command() {
    let spec = new_menu_spec();
    assert_tidy("New", &spec);
    assert_eq!(spec.sections.len(), 4, "Write, Capture, Import, Create");
    let creation = [CommandId::NewThread, CommandId::NewNote, CommandId::NewVoiceNote, CommandId::NewDrawing, CommandId::ImportFile, CommandId::NewAgent, CommandId::NewJournalEntry, CommandId::LogMood, CommandId::NewTimeCapsule, CommandId::NewDebate, CommandId::NewThoughtExperiment, CommandId::NewConnectionMap, CommandId::CreateNetwork];
    for id in creation {
        assert!(spec.reaches(&command(id)), "{id:?} is not in the New menu");
        assert!(spec.item(&command(id)).is_some_and(|item| item.action.is_some()), "{id:?} shows its shortcut");
    }
    assert_eq!(spec.path_of(&command(CommandId::NewDebate)).map(|path| path.len()), Some(2), "palette-only commands sit under More");
    assert!(spec.items().iter().all(|(_, item)| spec_is_listed(&item.id)), "planned commands stay hidden");
}

fn spec_is_listed(id: &str) -> bool {
    let key = id.strip_prefix(COMMAND_PREFIX).expect("a command item");
    CommandId::from_wire_key(key).is_some_and(|id| spec(id).is_listed())
}

#[test]
fn the_view_menus_section_every_view_and_tick_the_active_one() {
    let on_pick: PickView = Rc::new(|_, _, _| {});
    for scope in [ViewScope::Thread, ViewScope::Profile] {
        for active in PostView::in_scope(scope) {
            let spec = view_menu_spec(scope, active, &on_pick);
            assert_tidy("View", &spec);
            assert_eq!(spec.items().len(), PostView::in_scope(scope).len());
            let ticked: Vec<_> = spec.items().into_iter().filter(|(_, item)| item.checked == Some(true)).map(|(_, item)| item.id.to_string()).collect();
            assert_eq!(ticked, vec![format!("view.{}", active.key())]);
        }
    }
    assert!(view_menu_spec(ViewScope::Thread, PostView::Cards, &on_pick).sections.iter().all(|section| section.label.is_some()), "Layouts, Lenses, Collections");
}

#[test]
fn the_app_menu_reaches_every_palette_command() {
    let spec = crate::ui::features::title_bar::app_menu_spec();
    assert_tidy("App", &spec);
    for listed in crate::ui::features::command_palette::COMMANDS.iter().filter(|spec| spec.is_listed()) {
        assert!(spec.reaches(&command(listed.id)), "{:?} is missing from the app menu", listed.id);
    }
    assert!(spec.items().iter().all(|(_, item)| item.action.is_some()), "every entry shows its shortcut");
}

#[test]
fn the_language_and_network_menus_tick_the_current_choice() {
    let on_select: crate::ui::features::popovers::language::SelectLangCallback = Rc::new(|_, _, _| {});
    let spec = language_menu_spec(Language::French, &on_select);
    assert_tidy("Language", &spec);
    assert_eq!(spec.items().len(), Language::ALL.len());
    assert_eq!(spec.item("language.fr").and_then(|item| item.checked), Some(true));
    assert!(spec.scrollable, "the language list scrolls on a short window");
    let networks = vec![Network::new("n1", "Stoa", "stoa", ""), Network::new("n2", "Vienna Circle", "vienna", "")];
    let (pick, create): (crate::ui::features::popovers::workspace::SelectNetworkCallback, crate::ui::features::popovers::workspace::CreateNetworkCallback) = (Rc::new(|_, _, _| {}), Rc::new(|_, _| {}));
    let spec = network_menu_spec(&networks, Some(&networks[1].id), px(260.0), &pick, &create);
    assert_tidy("Network", &spec);
    let ticked: Vec<_> = spec.items().into_iter().filter(|(_, item)| item.checked == Some(true)).map(|(path, _)| path.join(" › ")).collect();
    assert_eq!(ticked, vec!["Vienna Circle".to_string()]);
    assert!(spec.reaches("network.create"));
}
