//! N11–N15: a post's menu entries for voice notes, retractions and corrections, and
//! distilling: "Transcribe voice note" (the user's own, once), "Mark as corrected…" and
//! "Retract…" or taking the mark back (the user's own; never a locked post), and "Distill
//! thread to vault" with or without a summary (any post).

use super::correction_sheet::CorrectionEvent;
use super::distill::DistillEvent;
use super::events::DataEvent;
use super::post_menu::{item, DataControls};
use super::voice_sheet::VoiceEvent;
use crate::domain::post::correction::CorrectionKind;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuItem, MenuText};

/// N11: "Transcribe voice note" on the user's own post whose voice note has no transcript.
pub(super) fn transcribe_item(controls: &DataControls) -> Option<MenuItem> {
    (controls.own && controls.facts.transcribable).then(|| item(ids::TRANSCRIBE, MenuText::key("posts.voice.transcribe"), LucideIcon::Mic, DataEvent::Voice(VoiceEvent::Transcribe(controls.post_id.clone())), &controls.on_event))
}

/// Whether the correction items apply: the user's own post, not locked.
fn correctable(controls: &DataControls) -> bool {
    controls.own && !controls.locked
}

/// N14: "Mark as corrected…", or taking back the retraction or correction the post carries.
pub(super) fn correction_item(controls: &DataControls) -> Option<MenuItem> {
    if !correctable(controls) {
        return None;
    }
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    Some(match controls.facts.correction {
        Some(kind) => item(ids::WITHDRAW_CORRECTION, MenuText::key(format!("posts.correction.withdraw_{}", kind.key())), LucideIcon::RotateCcw, DataEvent::Correction(CorrectionEvent::Withdraw(post.clone())), on_event),
        None => item(ids::MARK_CORRECTED, MenuText::key("posts.correction.corrected_menu"), LucideIcon::Pencil, DataEvent::Correction(CorrectionEvent::Open(post.clone(), CorrectionKind::Corrected)), on_event),
    })
}

/// N14: "Retract…" (destructive) while the post carries no mark yet.
pub(super) fn retract_item(controls: &DataControls) -> Option<MenuItem> {
    let open = correctable(controls) && controls.facts.correction.is_none();
    open.then(|| item(ids::RETRACT, MenuText::key("posts.correction.retracted_menu"), LucideIcon::CircleX, DataEvent::Correction(CorrectionEvent::Open(controls.post_id.clone(), CorrectionKind::Retracted)), &controls.on_event).destructive())
}

/// N15: distill the thread to the vault, as it is or with a summary.
pub(super) fn distill_items(controls: &DataControls) -> [MenuItem; 2] {
    let (post, on_event) = (&controls.post_id, &controls.on_event);
    [
        item(ids::DISTILL_THREAD, MenuText::key("posts.distill.menu"), LucideIcon::BookMarked, DataEvent::Distill(DistillEvent::Thread(post.clone())), on_event),
        item(ids::DISTILL_SUMMARY, MenuText::key("posts.distill.menu_summary"), LucideIcon::Sparkles, DataEvent::Distill(DistillEvent::Summarized(post.clone())), on_event),
    ]
}
