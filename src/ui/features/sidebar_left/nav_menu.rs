//! M1: the right-click menu of a place in the navigation: open it, then the commands that
//! start something there (a jump list). Every entry is a registry command, so it shows its
//! shortcut and runs exactly as from the palette.

use super::SidebarCallbacks;
use crate::ui::features::command_palette::{spec, CommandId};
use crate::ui::menus::{command_item, MenuItem, MenuSection, MenuSpec};

use CommandId as C;

/// The commands a place offers beyond opening it.
pub(super) fn jump_list(open: CommandId) -> &'static [CommandId] {
    match open {
        C::GoTimeline => &[C::NewThread, C::NewNote, C::OpenInbox, C::ScheduledPosts],
        C::GoKnowledge => &[C::ImportFile, C::OpenCollection, C::ExportBundle],
        C::GoAgents => &[C::NewAgent, C::SwitchPersona],
        C::GoReview => &[C::OpenQuestions, C::NewConnectionMap],
        C::GoInsights => &[C::InsightsOverview, C::InsightsNetwork, C::InsightsActivity, C::InsightsCalibration, C::InsightsTensions, C::InsightsTimeline],
        C::GoJournal => &[C::NewJournalEntry, C::LogMood, C::StartFocus, C::NewTimeCapsule],
        C::GoArena => &[C::NewDebate, C::NewThoughtExperiment],
        C::GoTasks => &[C::MigrationReview],
        C::GoSearch => &[C::SemanticSearch],
        _ => &[],
    }
}

/// `item` running its command through the sidebar (its shortcut still shows).
fn through(item: MenuItem, id: CommandId, callbacks: &SidebarCallbacks) -> MenuItem {
    let on_command = callbacks.on_command.clone();
    item.on_click(move |window, cx| on_command(id, window, cx))
}

/// The menu of the place `open` opens.
pub(super) fn nav_menu_spec(open: CommandId, callbacks: &SidebarCallbacks) -> MenuSpec {
    let jumps = jump_list(open).iter().copied().filter(|id| spec(*id).is_listed()).map(|id| through(command_item(id), id, callbacks));
    MenuSpec::new().section(MenuSection::new().entry(through(command_item(open), open, callbacks))).section(MenuSection::new().entries(jumps))
}
