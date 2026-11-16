//! Tests of the document file reader used by the vault import.

use super::{document_title, read_document_file, DocumentImportError, MAX_DOCUMENT_BYTES};
use std::path::{Path, PathBuf};

/// A unique scratch file in the OS temp folder, removed on drop.
struct TempFile(PathBuf);

impl TempFile {
    fn with(name: &str, bytes: &[u8]) -> Self {
        let dir = std::env::temp_dir().join(format!("simulacra-doc-{}", uuid::Uuid::new_v4().simple()));
        std::fs::create_dir_all(&dir).expect("temp dir");
        let path = dir.join(name);
        std::fs::write(&path, bytes).expect("write temp file");
        Self(path)
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        if let Some(dir) = self.0.parent() {
            let _ = std::fs::remove_dir_all(dir);
        }
    }
}

#[test]
fn title_is_the_file_stem() {
    assert_eq!(document_title(Path::new("C:/notes/Spinoza Ethics.md")), "Spinoza Ethics");
    assert_eq!(document_title(Path::new("/tmp/README")), "README");
}

#[test]
fn reads_title_and_content() {
    let file = TempFile::with("Republic.txt", "Justice is the soul's harmony.".as_bytes());
    let (title, content) = read_document_file(&file.0).expect("readable");
    assert_eq!(title, "Republic");
    assert_eq!(content, "Justice is the soul's harmony.");
}

#[test]
fn rejects_empty_binary_oversized_and_missing_files() {
    let blank = TempFile::with("blank.md", b"  \n ");
    assert_eq!(read_document_file(&blank.0), Err(DocumentImportError::Empty));

    let binary = TempFile::with("blob.txt", &[0xff, 0xfe, 0x00, 0x9f]);
    assert!(matches!(read_document_file(&binary.0), Err(DocumentImportError::Unreadable(_))));

    let huge = TempFile::with("huge.txt", &vec![b'a'; MAX_DOCUMENT_BYTES as usize + 1]);
    assert_eq!(read_document_file(&huge.0), Err(DocumentImportError::TooLarge));

    let missing = Path::new("definitely/not/here.md");
    assert!(matches!(read_document_file(missing), Err(DocumentImportError::Unreadable(_))));
}
