//! L12: what the composer does with dropped and pasted files.

use super::media::split_media;
use std::path::PathBuf;

#[test]
fn images_keep_the_tray_and_every_other_file_becomes_a_chip() {
    let paths = ["photo.JPG", "notes.md", "clip.mp4", "scan.png", "Makefile", "book.epub"].map(PathBuf::from).to_vec();
    let (images, files) = split_media(paths);
    assert_eq!(images, ["photo.JPG", "scan.png"].map(PathBuf::from));
    assert_eq!(files, ["notes.md", "clip.mp4", "Makefile", "book.epub"].map(PathBuf::from));
}
