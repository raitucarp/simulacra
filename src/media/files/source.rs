//! A local file as a [`ByteSource`] for the container readers: reads by offset, bounded.

use crate::domain::media::files::ByteSource;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

/// Most bytes one read hands back (the readers ask for headers, never whole files).
const MAX_READ: usize = 16 * 1024 * 1024;

/// An open file and its length.
pub struct FileBytes {
    file: File,
    len: u64,
}

impl FileBytes {
    /// The regular file at `path`, open for reading; `None` when it is not there.
    pub fn open(path: &Path) -> Option<Self> {
        let file = File::open(path).ok()?;
        let meta = file.metadata().ok().filter(std::fs::Metadata::is_file)?;
        Some(Self { file, len: meta.len() })
    }
}

impl ByteSource for FileBytes {
    fn len(&self) -> u64 {
        self.len
    }

    fn read_at(&mut self, offset: u64, len: usize) -> Option<Vec<u8>> {
        self.file.seek(SeekFrom::Start(offset)).ok()?;
        let mut bytes = Vec::new();
        (&mut self.file).take(len.min(MAX_READ) as u64).read_to_end(&mut bytes).ok()?;
        Some(bytes)
    }
}
