//! M1 (U3.5): the title bar's ☰ app menu: File, Post, View, Go, Tools, Insights and Help,
//! every entry a registry command dispatched to the shell (so each shows its shortcut). It
//! gathers the palette's commands where a desktop app keeps them, for people who browse
//! rather than type.

use crate::ui::features::command_palette::CommandId;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{command_item, command_items, render_menu, MenuSection, MenuSpec, MenuText, Submenu};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::component::Icon;
use gpui_kit::{FocusHandle, IntoElement};
use rust_i18n::t;

use CommandId as C;

const APP_MENU_ID: &str = "title-bar-app-menu";

fn section(label: Option<&'static str>, ids: &[CommandId]) -> MenuSection {
    let section = label.map_or_else(MenuSection::new, MenuSection::labelled);
    section.entries(command_items(ids))
}

fn submenu(id: &'static str, key: &'static str, icon: LucideIcon, spec: MenuSpec) -> Submenu {
    Submenu::new(id, MenuText::key(key), spec).icon(icon)
}

fn file() -> Submenu {
    let more = submenu("app.file.more", "menus.new.more", LucideIcon::Plus, MenuSpec::of(command_items(&[C::NewAgent, C::CreateNetwork, C::NewJournalEntry, C::NewTimeCapsule, C::NewDebate, C::NewThoughtExperiment, C::NewConnectionMap])));
    let new = section(None, &[C::NewThread, C::NewNote, C::NewVoiceNote, C::NewDrawing]).entry(more);
    let spec = MenuSpec::new().section(new).section(section(None, &[C::ImportFile])).section(section(None, &[C::ExportBundle, C::EncryptedSnapshot, C::RestoreSnapshot]));
    submenu("app.file", "menus.app.file", LucideIcon::FileText, spec)
}

fn go() -> Submenu {
    let places = [C::GoTimeline, C::GoKnowledge, C::GoAgents, C::GoPersonas, C::GoMindGraph, C::GoReview, C::GoInsights, C::GoJournal, C::GoArena, C::GoTasks];
    let spec = MenuSpec::new()
        .section(section(Some("menus.app.places"), &places))
        .section(section(Some("menus.app.lists"), &[C::OpenInbox, C::OpenCollection, C::ScheduledPosts, C::OpenQuestions]))
        .section(section(Some("menus.app.switch"), &[C::SwitchNetwork, C::SwitchPersona]));
    submenu("app.go", "menus.app.go", LucideIcon::Compass, spec)
}

fn tools() -> Submenu {
    let search = section(Some("menus.app.search"), &[C::GoSearch, C::SemanticSearch]).entry(command_item(C::OpenPalette));
    let spec = MenuSpec::new()
        .section(search)
        .section(section(Some("palette.category.thinking"), &[C::ResolveDuePredictions, C::SymbolKey, C::ImageIdeas]))
        .section(section(Some("palette.category.practice"), &[C::LogMood, C::StartFocus, C::MigrationReview]));
    submenu("app.tools", "menus.app.tools", LucideIcon::Puzzle, spec)
}

fn insights() -> Submenu {
    let network = [C::InsightsOverview, C::InsightsNetwork, C::InsightsSets, C::InsightsSymmetry, C::InsightsCategories, C::InsightsEmbedding, C::InsightsActivity];
    let thinking = [C::InsightsCalibration, C::InsightsTensions, C::InsightsTimeline, C::InsightsDiversity, C::InsightsReadability];
    let spec = MenuSpec::new().section(section(Some("menus.app.insights_network"), &network)).section(section(Some("menus.app.insights_thinking"), &thinking));
    submenu("app.insights", "palette.category.insights", LucideIcon::ChartColumn, spec)
}

/// The whole app menu (its actions go to the shell once the button sets the context).
pub fn app_menu_spec() -> MenuSpec {
    let post = submenu("app.post", "menus.app.post", LucideIcon::MessageSquare, MenuSpec::of(command_items(&[C::PostActions, C::FindSimilar, C::LearnPost, C::BookmarkPost, C::PinThread, C::DebateOpenThought, C::ExperimentOpenThought])));
    let view = MenuSpec::new().section(section(None, &[C::ToggleSidebar, C::ToggleRightSidebar])).section(section(None, &[C::ToggleOutline, C::ShowArgumentMap])).section(section(None, &[C::OpenAppearance]));
    let help = MenuSpec::of(command_items(&[C::ShowShortcuts, C::ReplayWelcome, C::OpenSettings]));
    let top = MenuSection::new().entry(file()).entry(post).entry(submenu("app.view", "menus.post.view", LucideIcon::Eye, view)).entry(go()).entry(tools()).entry(insights());
    MenuSpec::new().section(top).section(MenuSection::new().entry(submenu("app.help", "menus.app.help", LucideIcon::HelpCircle, help)))
}

/// The ☰ button opening the app menu, its commands run from `shell_focus`.
pub(super) fn render_app_menu(shell_focus: FocusHandle) -> impl IntoElement {
    Button::new(APP_MENU_ID)
        .ghost()
        .compact()
        .icon(Icon::new(LucideIcon::Menu))
        .tooltip(t!("menus.app.tooltip").to_string())
        .dropdown_menu(move |menu, window, cx| render_menu(menu, &app_menu_spec().action_context(shell_focus.clone()), window, cx))
}
