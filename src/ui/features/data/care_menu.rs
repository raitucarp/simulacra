//! N11–N15: a post's "…" menu items for voice notes, retractions and corrections, and
//! distilling: "Transcribe voice note" (the user's own, once), "Retract…" and "Mark as
//! corrected…" or taking the mark back (the user's own; never a locked post), and "Distill
//! thread to vault" with or without a summary (any post).

use super::correction_sheet::CorrectionEvent;
use super::distill::DistillEvent;
use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::text::tr;
use super::voice_sheet::VoiceEvent;
use crate::domain::post::correction::CorrectionKind;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::menu::PopupMenu;

fn own_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let (post, facts, on_event) = (&controls.post_id, &controls.facts, &controls.on_event);
    let mut menu = menu;
    if facts.transcribable {
        menu = menu.item(item(tr("posts.voice.transcribe"), LucideIcon::Mic, DataEvent::Voice(VoiceEvent::Transcribe(post.clone())), on_event)); // N11
    }
    if controls.locked {
        return menu;
    }
    match facts.correction {
        Some(kind) => menu.item(item(tr(&format!("posts.correction.withdraw_{}", kind.key())), LucideIcon::RotateCcw, DataEvent::Correction(CorrectionEvent::Withdraw(post.clone())), on_event)),
        None => menu
            .item(item(tr("posts.correction.retracted_menu"), LucideIcon::CircleX, DataEvent::Correction(CorrectionEvent::Open(post.clone(), CorrectionKind::Retracted)), on_event))
            .item(item(tr("posts.correction.corrected_menu"), LucideIcon::Pencil, DataEvent::Correction(CorrectionEvent::Open(post.clone(), CorrectionKind::Corrected)), on_event)),
    }
}

/// `menu` with the N11–N15 items of the post.
pub(super) fn with_care_items(menu: PopupMenu, controls: &DataControls) -> PopupMenu {
    let menu = if controls.own { own_items(menu, controls) } else { menu };
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    menu.item(item(tr("posts.distill.menu"), LucideIcon::BookMarked, DataEvent::Distill(DistillEvent::Thread(post.clone())), on_event))
        .item(item(tr("posts.distill.menu_summary"), LucideIcon::Sparkles, DataEvent::Distill(DistillEvent::Summarized(post.clone())), on_event))
}
