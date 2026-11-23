//! N12: ink posts in the store: a drawing is published finished (clamped, simplified), with
//! or without a caption; a blank one is refused.

use crate::domain::post::ink::{Ink, InkDrawing, Stroke};
use crate::state::store::AppState;

#[test]
fn a_drawing_is_posted_finished_and_a_blank_one_is_refused() {
    let mut state = AppState::new_in_memory();
    let drawing = InkDrawing { strokes: vec![Stroke { points: vec![10, 10, 20, 10, 30, 10, 900, 10], width: 4, ink: Ink::Accent }], svg_path: Some("media/ink/a.svg".into()), ..InkDrawing::blank() };
    let id = state.submit_ink_post("", drawing).expect("posted");
    let post = state.get_post(&id).expect("post");
    let kept = post.extras.ink.as_deref().expect("ink");
    assert_eq!(kept.strokes[0].points, vec![10, 10, 480, 10], "clamped to the canvas and simplified");
    assert_eq!(kept.svg_path.as_deref(), Some("media/ink/a.svg"));
    assert!(post.has_attachments() && post.content.is_empty());
    assert!(state.submit_ink_post("Nothing", InkDrawing::blank()).is_err());
}
