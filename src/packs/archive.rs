//! `.simpack` archives: a zip with `pack.json` at its root and every covered file at its
//! manifest path. Reading verifies each file against the SHA-256 its manifest pins, so a
//! [`VerifiedPack`] only ever holds checked bytes. Entries are read with size caps, so a
//! compressed bomb cannot exhaust memory.

use crate::domain::packs::{is_built_in, sha256_hex, PackError, PackManifest, MANIFEST_FILE, MAX_FILE_BYTES, MAX_PACK_BYTES};
use std::collections::BTreeMap;
use std::io::{Cursor, Read, Write};
use zip::result::ZipError;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipArchive, ZipWriter};

/// A pack whose every file matched its pinned checksum: manifest, and file path → bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedPack {
    pub manifest: PackManifest,
    pub files: BTreeMap<String, Vec<u8>>,
}

/// Reads and verifies the `.simpack` archive `bytes`; refuses built-in ids, a missing or
/// invalid manifest, a missing file and any checksum mismatch.
pub fn read_simpack(bytes: &[u8]) -> Result<VerifiedPack, PackError> {
    if bytes.len() as u64 > MAX_PACK_BYTES {
        return Err(PackError::TooLarge);
    }
    let mut zip = ZipArchive::new(Cursor::new(bytes)).map_err(|e| PackError::Archive(e.to_string()))?;
    let json = read_entry(&mut zip, MANIFEST_FILE)?.ok_or_else(|| PackError::Missing(MANIFEST_FILE.into()))?;
    let manifest = PackManifest::parse(&String::from_utf8_lossy(&json))?;
    if is_built_in(&manifest.id) {
        return Err(PackError::BuiltIn(manifest.id));
    }
    let files = verified_files(&manifest, |file| read_entry(&mut zip, file))?;
    Ok(VerifiedPack { manifest, files })
}

/// Every file `manifest` covers, read through `read` and checked against its SHA-256.
pub fn verified_files(
    manifest: &PackManifest,
    mut read: impl FnMut(&str) -> Result<Option<Vec<u8>>, PackError>,
) -> Result<BTreeMap<String, Vec<u8>>, PackError> {
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut total = 0u64;
    for entry in manifest.coverage.values() {
        if !files.contains_key(&entry.file) {
            let data = read(&entry.file)?.ok_or_else(|| PackError::Missing(entry.file.clone()))?;
            total += data.len() as u64;
            if total > MAX_PACK_BYTES {
                return Err(PackError::TooLarge);
            }
            files.insert(entry.file.clone(), data);
        }
        if files.get(&entry.file).is_none_or(|data| sha256_hex(data) != entry.sha256) {
            return Err(PackError::Checksum(entry.file.clone()));
        }
    }
    Ok(files)
}

/// The bytes of entry `name`, or `None` when the archive has no such entry.
fn read_entry(zip: &mut ZipArchive<Cursor<&[u8]>>, name: &str) -> Result<Option<Vec<u8>>, PackError> {
    let entry = match zip.by_name(name) {
        Ok(entry) => entry,
        Err(ZipError::FileNotFound) => return Ok(None),
        Err(err) => return Err(PackError::Archive(err.to_string())),
    };
    if entry.size() > MAX_FILE_BYTES {
        return Err(PackError::TooLarge);
    }
    let mut data = Vec::new();
    entry.take(MAX_FILE_BYTES + 1).read_to_end(&mut data).map_err(|e| PackError::Archive(e.to_string()))?;
    if data.len() as u64 > MAX_FILE_BYTES {
        return Err(PackError::TooLarge);
    }
    Ok(Some(data))
}

/// A `.simpack` of `manifest` and `files` (file path → bytes), deflated.
pub fn write_simpack(manifest: &PackManifest, files: &BTreeMap<String, Vec<u8>>) -> Result<Vec<u8>, PackError> {
    let io = |e: ZipError| PackError::Archive(e.to_string());
    let options = || SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    let entries = std::iter::once((MANIFEST_FILE, manifest.to_json().into_bytes())).chain(files.iter().map(|(path, data)| (path.as_str(), data.clone())));
    for (path, data) in entries {
        writer.start_file(path, options()).map_err(io)?;
        writer.write_all(&data).map_err(|e| PackError::Archive(e.to_string()))?;
    }
    Ok(writer.finish().map_err(io)?.into_inner())
}
