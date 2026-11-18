use super::labels::{failure_art, fixed_in_image_settings, use_label};
use super::*;
use crate::domain::image_prompt::{ImageAvoid, ImageChoices, ImageFraming, ImageMood, ImageTarget};
use crate::ui::illustration::NoticeKey;

#[test]
fn picks_apply_and_report_themselves() {
    let choices = ImageChoices::for_target(ImageTarget::Avatar);
    let moody = IdeasPick::Mood(ImageMood::Mysterious).apply(choices.clone());
    assert!(IdeasPick::Mood(ImageMood::Mysterious).is_on(&moody) && !IdeasPick::Mood(ImageMood::Mysterious).is_on(&choices));
    let framed = IdeasPick::Framing(ImageFraming::Emblem).apply(moody);
    assert!(IdeasPick::Framing(ImageFraming::Emblem).is_on(&framed));
    let text_allowed = IdeasPick::Avoid(ImageAvoid::Text).apply(framed);
    assert!(!IdeasPick::Avoid(ImageAvoid::Text).is_on(&text_allowed));
}

#[test]
fn failures_pick_their_art_and_fix() {
    assert_eq!(failure_art(&IdeasFailure::NoImageKey), NoticeKey::NoKey);
    assert_eq!(failure_art(&IdeasFailure::Offline), NoticeKey::Offline);
    assert_eq!(failure_art(&IdeasFailure::Failed("x".into())), NoticeKey::Error);
    assert!(fixed_in_image_settings(&IdeasFailure::NoEngine) && !fixed_in_image_settings(&IdeasFailure::NoTextKey));
    assert!(!failure_text(&IdeasFailure::Budget).is_empty());
}

#[test]
fn results_go_to_the_avatar_or_the_draft() {
    assert_ne!(use_label(&IdeasDestination::Composer), use_label(&IdeasDestination::Forge));
}

#[test]
fn runner_and_engine_failures_map_to_the_sheet() {
    use crate::agent::post_actions::ActionFailure;
    use crate::domain::post_actions::ActionFailureKind;
    assert_eq!(from_action(ActionFailure::local(ActionFailureKind::Cancelled)), None);
    assert_eq!(from_action(ActionFailure::local(ActionFailureKind::NoKey)), Some(IdeasFailure::NoTextKey));
    assert_eq!(from_action(ActionFailure::local(ActionFailureKind::Timeout)), Some(IdeasFailure::Offline));
    assert_eq!(from_image_error("OpenAI image request error: refused".into()), IdeasFailure::Offline);
    assert_eq!(from_image_error("OpenAI returned no image".into()), IdeasFailure::Failed("OpenAI returned no image".into()));
}
