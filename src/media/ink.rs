//! N12: an ink drawing's SVG file in the media folder (`ink/<hash>.svg`, once per content).

use crate::domain::post::ink::{to_svg, Ink, InkDrawing};
use crate::import::parse::sha256_hex;
use std::path::Path;

const INK_DIR: &str = "ink";
/// Hex characters of the content hash used as a file name.
const NAME_HASH_CHARS: usize = 24;

/// Writes `drawing` as SVG (inks painted as `colour_of` says) and returns the file's path.
pub fn save_ink_svg(drawing: &InkDrawing, colour_of: impl Fn(Ink) -> String, media_dir: &Path) -> Result<String, String> {
    let svg = to_svg(drawing, colour_of);
    let name: String = sha256_hex(svg.as_bytes()).chars().take(NAME_HASH_CHARS).collect();
    let folder = media_dir.join(INK_DIR);
    let path = folder.join(format!("{name}.svg"));
    if !path.exists() {
        std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        std::fs::write(&path, svg).map_err(|e| e.to_string())?;
    }
    Ok(path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::post::ink::Stroke;

    #[test]
    fn a_drawing_is_saved_once_as_svg() {
        let media = std::env::temp_dir().join(format!("simulacra-ink-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&media);
        let drawing = InkDrawing { strokes: vec![Stroke { points: vec![1, 2, 30, 40], width: 4, ink: Ink::Accent }], ..InkDrawing::blank() };
        let path = save_ink_svg(&drawing, |_| "#123456".into(), &media).expect("saved");
        assert_eq!(save_ink_svg(&drawing, |_| "#123456".into(), &media).expect("again"), path);
        let svg = std::fs::read_to_string(&path).expect("file");
        assert!(path.ends_with(".svg") && svg.contains("stroke=\"#123456\"") && svg.contains("M1 2 L30 40"));
        let _ = std::fs::remove_dir_all(&media);
    }
}
