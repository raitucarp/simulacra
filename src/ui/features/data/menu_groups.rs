//! The post's data entries, sorted into the sections of the post menu (M1): what answers it,
//! what studies it, what keeps and sorts it, how it is seen and shared, how it changed, and
//! what ends it. The post menu places each group; every entry keeps the condition it had.

use super::events::DataEvent;
use super::post_menu::{collections_submenu, item, DataControls};
use super::question::QuestionEvent;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuEntry, MenuItem, MenuText, Submenu};
use rust_i18n::t;

/// Every data entry of one post, by the section it belongs to.
#[derive(Default)]
pub struct DataMenu {
    /// Respond: "Reply in the margin…".
    pub respond: Vec<MenuItem>,
    /// Think with AI: "Fork a what-if…".
    pub think: Vec<MenuItem>,
    /// Study: words to review; the post as a question; its terms.
    pub review: Vec<MenuItem>,
    pub questions: Vec<MenuItem>,
    pub terms: Vec<MenuItem>,
    /// Organize: keeping and sorting it; its lifecycle; deleting it on a timer (destructive).
    pub keep: Vec<MenuEntry>,
    pub lifecycle: Vec<MenuEntry>,
    pub expiry: Option<Submenu>,
    /// View: translation, lens fan, transcript.
    pub view: Vec<MenuItem>,
    /// Share & copy: who can read it.
    pub audience: Option<Submenu>,
    /// Edit & history: edit, versions, corrections, change of mind; retracting (destructive).
    pub edit: Vec<MenuItem>,
    pub retract: Option<MenuItem>,
}

fn bookmark_item(controls: &DataControls) -> MenuItem {
    let (id, key, icon) = if controls.bookmarked { (ids::UNBOOKMARK, "data.menu.unbookmark", LucideIcon::BookmarkCheck) } else { (ids::BOOKMARK, "data.menu.bookmark", LucideIcon::Bookmark) };
    item(id, MenuText::key(key), icon, DataEvent::ToggleBookmark(controls.post_id.clone()), &controls.on_event)
}

/// Edit (the user's own), then History once the post was edited.
fn edit_items(controls: &DataControls) -> Vec<MenuItem> {
    let post = &controls.post_id;
    let edit = controls.own.then(|| item(ids::EDIT, MenuText::key("data.menu.edit"), LucideIcon::Pencil, DataEvent::EditPost(post.clone()), &controls.on_event));
    let history = (controls.revisions > 0).then(|| {
        let label = MenuText::keyed("data.menu.history", t!("data.menu.history", count = controls.revisions).to_string());
        item(ids::HISTORY, label, LucideIcon::RotateCcwClock, DataEvent::OpenPostHistory(post.clone()), &controls.on_event)
    });
    edit.into_iter().chain(history).collect()
}

/// N2 margin reply and N6 what-if fork: never on a locked post (nothing quotes its text).
fn unlocked_item(controls: &DataControls, id: &'static str, key: &'static str, icon: LucideIcon, event: DataEvent) -> Option<MenuItem> {
    (!controls.locked).then(|| item(id, MenuText::key(key), icon, event, &controls.on_event))
}

/// N9: "Ask the room" on the user's own post, once.
fn ask_room_item(controls: &DataControls) -> Option<MenuItem> {
    let open = controls.own && !controls.facts.asked && !controls.locked;
    open.then(|| item(ids::ASK_ROOM, MenuText::key("posts.question.menu"), LucideIcon::Users, DataEvent::Question(QuestionEvent::Ask(controls.post_id.clone())), &controls.on_event))
}

fn keep_entries(controls: &DataControls) -> Vec<MenuEntry> {
    let mut keep: Vec<MenuEntry> = vec![bookmark_item(controls).into(), collections_submenu(controls).into()];
    keep.extend(super::idea_menu::series_entry(controls));
    keep.extend(super::garden_menu::garden_entry(controls));
    keep.extend(super::state_menu::entry_submenu(controls).map(MenuEntry::from));
    keep.extend(super::idea_menu::repin_item(controls).map(MenuEntry::from));
    keep.extend(super::care_menu::distill_items(controls).map(MenuEntry::from));
    keep
}

/// The data entries of the post `controls` describe.
pub fn data_menu(controls: &DataControls) -> DataMenu {
    let post = &controls.post_id;
    let mut edit = edit_items(controls);
    edit.extend(super::care_menu::correction_item(controls));
    edit.extend(super::thought_menu::mind_items(controls));
    let mut questions = super::thought_menu::question_items(controls);
    questions.extend(ask_room_item(controls));
    questions.extend(super::thought_menu::answers_item(controls));
    DataMenu {
        respond: unlocked_item(controls, ids::MARGIN, "post.margin.menu", LucideIcon::MessageSquareReply, DataEvent::ReplyInMargin(post.clone())).into_iter().collect(),
        think: unlocked_item(controls, ids::BRANCH, "posts.branch.menu", LucideIcon::GitBranch, DataEvent::Branch(super::branch_sheet::BranchEvent::Fork(post.clone()))).into_iter().collect(),
        review: super::thought_menu::words_item(controls).into_iter().collect(),
        questions,
        terms: super::thought_menu::glossary_items(controls),
        keep: keep_entries(controls),
        lifecycle: super::lifecycle_menu::lifecycle_entries(controls),
        expiry: super::lifecycle_menu::auto_delete_submenu(controls),
        view: [super::thought_menu::translation_item(controls), super::thought_menu::lens_item(controls), super::care_menu::transcribe_item(controls)].into_iter().flatten().collect(),
        audience: super::idea_menu::audience_submenu(controls),
        edit,
        retract: super::care_menu::retract_item(controls),
    }
}
