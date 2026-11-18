//! Writes the bundle files into a deflated zip. Copied files that are gone are skipped and
//! counted, never fatal; the zip is written to a temporary name and renamed when complete,
//! so a failed export never leaves half a bundle under the chosen name.

use super::io_err;
use crate::domain::data::export::{BundleFile, FileBody};
use crate::error::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

/// Suffix of the zip while it is being written.
const PARTIAL_SUFFIX: &str = ".partial";

/// What was written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BundleReport {
    pub files: usize,
    /// Copies skipped because their source file is gone.
    pub missing: usize,
    pub bytes: u64,
}

fn add_file(zip: &mut ZipWriter<BufWriter<File>>, file: &BundleFile, report: &mut BundleReport) -> Result<()> {
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated).large_file(true);
    let bytes = match &file.body {
        FileBody::Text(text) => text.as_bytes().to_vec(),
        FileBody::Copy(source) => match std::fs::read(source) {
            Ok(bytes) => bytes,
            Err(_) => {
                report.missing += 1;
                return Ok(());
            }
        },
    };
    zip.start_file(file.path.as_str(), options).map_err(io_err)?;
    zip.write_all(&bytes).map_err(io_err)?;
    report.files += 1;
    report.bytes += bytes.len() as u64;
    Ok(())
}

/// Writes `files` as a zip at `path` (replacing it).
pub fn write_bundle(path: &Path, files: &[BundleFile]) -> Result<BundleReport> {
    let partial = path.with_file_name(format!("{}{PARTIAL_SUFFIX}", path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default()));
    let mut zip = ZipWriter::new(BufWriter::new(File::create(&partial).map_err(io_err)?));
    let mut report = BundleReport::default();
    let written = files.iter().try_for_each(|file| add_file(&mut zip, file, &mut report)).and_then(|()| zip.finish().map_err(io_err)?.flush().map_err(io_err));
    match written.and_then(|()| std::fs::rename(&partial, path).map_err(io_err)) {
        Ok(()) => Ok(report),
        Err(err) => {
            let _ = std::fs::remove_file(&partial);
            Err(err)
        }
    }
}
