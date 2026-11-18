//! Gallery geometry: where each of 2–4 images sits in a two-column grid, how tall a single
//! image may be relative to its width, and stepping through images in the lightbox.

use super::image::ImageSize;

/// Images shown (and attachable) per post.
pub const MAX_GALLERY_IMAGES: usize = 4;
/// Narrowest and widest a single image is drawn (width / height); beyond these it is cropped.
pub const MIN_SINGLE_ASPECT: f32 = 4.0 / 5.0;
pub const MAX_SINGLE_ASPECT: f32 = 21.0 / 9.0;
/// Shape of a single image whose size is not known yet, and of every 2–4 image grid.
pub const PLACEHOLDER_ASPECT: f32 = 4.0 / 3.0;
pub const GALLERY_ASPECT: f32 = 16.0 / 9.0;
/// Images at most this wide for their height are sized by height (and keep their width).
const SQUARE_ASPECT: f32 = 1.0;
/// Columns of a gallery grid.
const GALLERY_COLUMNS: u16 = 2;

/// One image's cell in the grid (zero-based start, spans of at least one).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub column: u16,
    pub row: u16,
    pub column_span: u16,
    pub row_span: u16,
}

/// Which outer corners of the grid a tile touches: top-left, top-right, bottom-left,
/// bottom-right (those corners of its image are rounded with the frame).
pub type Corners = [bool; 4];

impl Tile {
    /// The grid corners this tile sits in, in a `columns` x `rows` grid.
    pub fn corners(&self, columns: u16, rows: u16) -> Corners {
        let (left, top) = (self.column == 0, self.row == 0);
        let (right, bottom) = (self.column + self.column_span >= columns, self.row + self.row_span >= rows);
        [left && top, right && top, left && bottom, right && bottom]
    }
}

const fn tile(column: u16, row: u16, column_span: u16, row_span: u16) -> Tile {
    Tile { column, row, column_span, row_span }
}

/// The grid of a gallery: its size and one tile per image, in image order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GalleryLayout {
    pub columns: u16,
    pub rows: u16,
    pub tiles: Vec<Tile>,
}

/// The layout of `count` images (extra images beyond the gallery size are not placed).
///
/// One image fills the grid; two sit side by side; three put the first image down the
/// whole left column and stack the others on the right; four make a two-by-two grid.
pub fn gallery_layout(count: usize) -> GalleryLayout {
    let (columns, rows, tiles) = match count.min(MAX_GALLERY_IMAGES) {
        0 => (0, 0, vec![]),
        1 => (1, 1, vec![tile(0, 0, 1, 1)]),
        2 => (GALLERY_COLUMNS, 1, vec![tile(0, 0, 1, 1), tile(1, 0, 1, 1)]),
        3 => (GALLERY_COLUMNS, 2, vec![tile(0, 0, 1, 2), tile(1, 0, 1, 1), tile(1, 1, 1, 1)]),
        _ => (GALLERY_COLUMNS, 2, vec![tile(0, 0, 1, 1), tile(1, 0, 1, 1), tile(0, 1, 1, 1), tile(1, 1, 1, 1)]),
    };
    GalleryLayout { columns, rows, tiles }
}

/// Width over height a single image is drawn at: its own aspect within the allowed range,
/// or the placeholder shape while its size is unknown.
pub fn clamp_aspect(size: Option<ImageSize>) -> f32 {
    size.and_then(ImageSize::aspect).map_or(PLACEHOLDER_ASPECT, |aspect| aspect.clamp(MIN_SINGLE_ASPECT, MAX_SINGLE_ASPECT))
}

/// Whether a single image of `aspect` is sized by height (portrait and square images take the
/// card's maximum height and their own width) rather than by the card's width.
pub fn is_portrait(aspect: f32) -> bool {
    aspect <= SQUARE_ASPECT
}

/// The aspect a post's media block is drawn at for `count` images.
pub fn media_aspect(count: usize, single: Option<ImageSize>) -> f32 {
    if count <= 1 {
        clamp_aspect(single)
    } else {
        GALLERY_ASPECT
    }
}

/// The size `image` is drawn at to fill `room` (width, height) without cropping: scaled up or
/// down by one factor, so it touches the room on one axis ("zoom to fit").
pub fn fit_within(image: ImageSize, room: (f32, f32)) -> (f32, f32) {
    let (room_width, room_height) = (room.0.max(0.0), room.1.max(0.0));
    let Some(aspect) = image.aspect() else { return (room_width, room_height) };
    if room_width / aspect <= room_height {
        (room_width, room_width / aspect)
    } else {
        (room_height * aspect, room_height)
    }
}

/// A move through the lightbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Previous,
    Next,
}

/// The index after moving `step` from `current` through `count` images, wrapping around.
pub fn step_index(current: usize, count: usize, step: Step) -> usize {
    if count == 0 {
        return 0;
    }
    let current = current.min(count - 1);
    match step {
        Step::Next => (current + 1) % count,
        Step::Previous => (current + count - 1) % count,
    }
}
