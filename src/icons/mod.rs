//! The icon library at the edges (decision #19): downloading the pinned package from the npm
//! registry and verifying it (S10). Everything here blocks and runs on a background thread;
//! reading the package is pure and lives in [`crate::domain::icons`].

pub mod fetch;
#[cfg(test)]
mod tests;

pub use fetch::{fetch_package, fetch_package_blocking, FetchError, PackageSource, Progress, MAX_PACKAGE_BYTES, PACKAGE_BYTES, PACKAGE_INTEGRITY, PACKAGE_URL, PACKAGE_VERSION};
