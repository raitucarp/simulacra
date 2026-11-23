use super::*;
use crate::domain::post::lock::LockPassword;
use crate::domain::Post;

fn stroke(points: &[(i16, i16)], width: u8, ink: Ink) -> Stroke {
    Stroke { points: points.iter().flat_map(|(x, y)| [*x, *y]).collect(), width, ink }
}

fn colour(ink: Ink) -> String {
    match ink {
        Ink::Text => SVG_TEXT_INK.to_string(),
        Ink::Accent => "#3366ff".to_string(),
        Ink::Danger => "#cc3333".to_string(),
        Ink::Success => "#33aa66".to_string(),
    }
}

#[test]
fn a_straight_stroke_keeps_its_ends_and_a_bend_keeps_its_corner() {
    let line: Vec<(i16, i16)> = (0..50).map(|x| (x, 10)).collect();
    assert_eq!(simplify(&line), vec![(0, 10), (49, 10)]);
    let corner = [(0, 0), (5, 0), (10, 0), (10, 5), (10, 10), (10, 10)];
    assert_eq!(simplify(&corner), vec![(0, 0), (10, 0), (10, 10)]);
    assert_eq!(simplify(&[(3, 3), (3, 3)]), vec![(3, 3)], "a tap is one point");
}

#[test]
fn strokes_become_one_svg_path_each_with_their_ink_and_width() {
    let mut drawing = InkDrawing::blank();
    drawing.strokes = vec![stroke(&[(10, 20), (30, 40), (60, 20)], 4, Ink::Text), stroke(&[(100, 100)], 8, Ink::Accent)];
    let svg = to_svg(&drawing, colour);
    assert!(svg.starts_with("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 480 270\""));
    assert!(svg.contains("<path d=\"M10 20 L30 40 L60 20\" stroke=\"currentColor\" stroke-width=\"4\"/>"));
    assert!(svg.contains("<path d=\"M100 100 l0 0\" stroke=\"#3366ff\" stroke-width=\"8\"/>"), "a dot: {svg}");
    assert!(svg.trim_end().ends_with("</svg>"));
}

#[test]
fn a_finished_drawing_is_clamped_simplified_and_bounded() {
    let mut drawing = InkDrawing::blank();
    drawing.strokes = vec![stroke(&[(-20, 5), (0, 5), (900, 5)], 99, Ink::Danger), Stroke::default()];
    let done = drawing.clone().finished().expect("something drawn");
    assert_eq!(done.strokes.len(), 1, "the empty stroke goes");
    assert_eq!(done.strokes[0].points, vec![0, 5, 480, 5]);
    assert_eq!(done.strokes[0].width, PEN_WIDTHS[PEN_WIDTHS.len() - 1]);
    assert_eq!(InkDrawing::blank().finished(), None, "nothing drawn, nothing kept");
    drawing.strokes = (0..MAX_STROKES + 10).map(|i| stroke(&[(0, 0), ((i % 400) as i16, 200)], 2, Ink::Text)).collect();
    let capped = drawing.finished().expect("kept");
    assert!(capped.strokes.len() <= MAX_STROKES && capped.point_count() <= MAX_POINTS);
}

#[test]
fn a_drawing_is_an_attachment_so_its_post_is_never_locked() {
    let mut post = Post::create_with_files("p1", "n1", "user_main", "", None, None, None, None, None, true).expect("post");
    post.extras.ink = InkDrawing { strokes: vec![stroke(&[(1, 1), (9, 9)], 2, Ink::Text)], ..InkDrawing::blank() }.finished().map(Box::new);
    assert!(post.has_attachments());
    let kdf = crate::domain::crypto::KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
    assert_eq!(post.lock_with(&LockPassword::new("seven tidy lanterns"), kdf), Err(crate::error::AppError::LockedWithAttachments));
    let json = serde_json::to_string(&post.extras).expect("json");
    assert!(json.contains("\"ink\":{\"width\":480,\"height\":270,\"strokes\":[{\"points\":[1,1,9,9],\"width\":2,\"ink\":\"text\"}]}"), "{json}");
    assert_eq!(serde_json::from_str::<crate::domain::post::PostExtras>(&json).expect("back"), post.extras);
}
