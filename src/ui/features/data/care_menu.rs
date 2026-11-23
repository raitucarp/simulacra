//! N11–N15: a post's "…" menu items for its voice note: "Transcribe voice note" (the user's
//! own, once, never on a locked post).

use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use super::voice_sheet::VoiceEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;

fn own_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, facts, on_event) = (&controls.post_id, &controls.facts, &controls.on_event);
    let mut menu = menu;
    if facts.transcribable {
        menu = menu.item(item(tr("posts.voice.transcribe"), LucideIcon::Mic, DataEvent::Voice(VoiceEvent::Transcribe(post.clone())), on_event)); // N11
    }
    menu
}

/// `menu` with the N11–N15 items of the post.
pub(super) fn with_care_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    if controls.own { own_items(menu, controls) } else { menu }
}
