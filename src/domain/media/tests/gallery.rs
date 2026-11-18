use crate::domain::media::gallery::{media_aspect, GALLERY_ASPECT, MAX_SINGLE_ASPECT, MIN_SINGLE_ASPECT, PLACEHOLDER_ASPECT};
use crate::domain::media::{clamp_aspect, gallery_layout, post_images, step_index, ImageAttachment, ImageSize, Step, Tile, MAX_GALLERY_IMAGES};
use crate::domain::{Post, Profile};

/// Every grid cell covered exactly once by the tiles of `count` images.
fn assert_tiles_cover_grid(count: usize) {
    let layout = gallery_layout(count);
    let mut cells = vec![0u8; usize::from(layout.columns * layout.rows)];
    for Tile { column, row, column_span, row_span } in &layout.tiles {
        for r in *row..row + row_span {
            for c in *column..column + column_span {
                assert!(c < layout.columns && r < layout.rows, "{count}: tile outside the grid");
                cells[usize::from(r * layout.columns + c)] += 1;
            }
        }
    }
    assert!(cells.iter().all(|&n| n == 1), "{count}: cells {cells:?}");
}

#[test]
fn layouts_place_one_to_four_images_without_gaps_or_overlap() {
    for count in 1..=MAX_GALLERY_IMAGES {
        assert_eq!(gallery_layout(count).tiles.len(), count);
        assert_tiles_cover_grid(count);
    }
    assert_eq!(gallery_layout(0).tiles.len(), 0);
}

#[test]
fn three_images_put_the_first_down_the_left_column() {
    let layout = gallery_layout(3);
    assert_eq!((layout.columns, layout.rows), (2, 2));
    assert_eq!(layout.tiles[0], Tile { column: 0, row: 0, column_span: 1, row_span: 2 });
}

#[test]
fn every_outer_corner_is_rounded_by_exactly_one_tile() {
    for count in 1..=MAX_GALLERY_IMAGES {
        let layout = gallery_layout(count);
        let corners: Vec<[bool; 4]> = layout.tiles.iter().map(|tile| tile.corners(layout.columns, layout.rows)).collect();
        for corner in 0..4 {
            assert_eq!(corners.iter().filter(|c| c[corner]).count(), 1, "{count} images, corner {corner}: {corners:?}");
        }
    }
    let three = gallery_layout(3);
    assert_eq!(three.tiles[0].corners(2, 2), [true, false, true, false], "the tall tile takes both left corners");
}

#[test]
fn more_than_four_images_show_the_first_four() {
    assert_eq!(gallery_layout(9), gallery_layout(4));
}

#[test]
fn single_images_keep_their_aspect_within_limits() {
    assert_eq!(clamp_aspect(Some(ImageSize::new(1600, 900))), 1600.0 / 900.0);
    assert_eq!(clamp_aspect(Some(ImageSize::new(400, 2000))), MIN_SINGLE_ASPECT);
    assert_eq!(clamp_aspect(Some(ImageSize::new(5000, 500))), MAX_SINGLE_ASPECT);
    assert_eq!(clamp_aspect(Some(ImageSize::new(0, 500))), PLACEHOLDER_ASPECT);
    assert_eq!(clamp_aspect(None), PLACEHOLDER_ASPECT);
    assert_eq!(media_aspect(3, Some(ImageSize::new(1, 9))), GALLERY_ASPECT);
}

#[test]
fn portrait_and_square_images_are_sized_by_height() {
    use crate::domain::media::is_portrait;
    assert!(is_portrait(clamp_aspect(Some(ImageSize::new(900, 1600)))));
    assert!(is_portrait(clamp_aspect(Some(ImageSize::new(800, 800)))));
    assert!(!is_portrait(clamp_aspect(Some(ImageSize::new(1600, 900)))));
    assert!(!is_portrait(clamp_aspect(None)), "unknown sizes take the full width");
}

#[test]
fn zoom_to_fit_fills_one_axis_and_keeps_the_shape() {
    use crate::domain::media::fit_within;
    assert_eq!(fit_within(ImageSize::new(1000, 1000), (1400.0, 800.0)), (800.0, 800.0));
    assert_eq!(fit_within(ImageSize::new(1600, 900), (800.0, 800.0)), (800.0, 450.0));
    assert_eq!(fit_within(ImageSize::new(100, 50), (1000.0, 1000.0)), (1000.0, 500.0), "small images are zoomed up");
    assert_eq!(fit_within(ImageSize::new(0, 50), (300.0, 200.0)), (300.0, 200.0));
    assert_eq!(fit_within(ImageSize::new(4, 3), (-5.0, 10.0)), (0.0, 0.0));
}

#[test]
fn lightbox_steps_wrap_around() {
    assert_eq!(step_index(0, 3, Step::Next), 1);
    assert_eq!(step_index(2, 3, Step::Next), 0);
    assert_eq!(step_index(0, 3, Step::Previous), 2);
    assert_eq!(step_index(7, 3, Step::Previous), 1, "an out-of-range index is clamped first");
    assert_eq!(step_index(0, 0, Step::Next), 0);
    assert_eq!(step_index(0, 1, Step::Next), 0);
}

#[test]
fn post_images_prefer_attachments_and_fall_back_to_the_image_path() {
    let author = Profile::new_primary("u", "U", "@u");
    let mut post = Post::create("p", "n", author, "text").expect("post");
    assert!(post_images(&post).is_empty());
    post.image_path = Some("  media/agent.png ".into());
    assert_eq!(post_images(&post), vec![ImageAttachment::at("media/agent.png")]);
    post.extras.images = (0..6).map(|i| ImageAttachment { path: format!("{i}.png"), alt: format!("alt {i}"), size: None }).collect();
    post.extras.images[1].path = " ".into();
    let images = post_images(&post);
    assert_eq!(images.len(), MAX_GALLERY_IMAGES);
    assert_eq!(images.iter().map(|i| i.path.as_str()).collect::<Vec<_>>(), ["0.png", "2.png", "3.png", "4.png"]);
}

#[test]
fn image_attachments_round_trip_in_post_extras() {
    let mut extras = crate::domain::post::PostExtras::default();
    assert!(extras.is_empty());
    extras.images.push(ImageAttachment { path: "a.png".into(), alt: "A quiet harbour".into(), size: Some(ImageSize::new(4, 3)) });
    let json = serde_json::to_string(&extras).expect("json");
    assert_eq!(serde_json::from_str::<crate::domain::post::PostExtras>(&json).expect("parse"), extras);
    assert!(!extras.has_details(), "images are shown in the card, not as details");
    let legacy: crate::domain::post::PostExtras = serde_json::from_str("{\"mood\":\"calm\"}").expect("legacy");
    assert!(legacy.images.is_empty());
}
