//! Pack IO (WS5): archives verified file by file, the store of installed packs, and
//! downloads pinned to the archive's SHA-256, served locally. No test touches the network or
//! the real app data folder.

mod archive;
mod fetch;
mod store;

use std::path::PathBuf;

/// The sample pack built by `tools/illustrations` (`process.py` + `make_simpack.py`).
pub(super) const SAMPLE_SIMPACK: &[u8] = include_bytes!("../../../tests/fixtures/packs/sample.simpack");

/// A fresh, empty folder under the system temp dir.
pub(super) fn scratch_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("simulacra-packs-test-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    dir
}
