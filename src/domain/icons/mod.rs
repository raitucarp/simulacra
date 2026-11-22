//! The downloadable icon library (decision #19): its sets are fetched on demand from
//! Settings › Symbols, and the projects behind them are named only on the Licences screen.
//!
//! Pure parts; the network and the files are in [`crate::icons`]:
//! - [`integrity`]: the tarball's SHA-512 in npm's `sha512-…` form (S10).
//! - [`archive`]: the wanted files out of the gzip-compressed tarball (S10).
//! - [`module`] and [`tree`]: one set's module read as text into icon trees, and each tree
//!   converted to a standalone, sanitised SVG (S11).
//! - [`catalog`], [`index`] and [`licence`]: the pinned package's sets, what the store
//!   holds, and the licence text stored with each set (S12).
//! - [`search`]: the downloaded icons searched by name tokens, optionally in one set (S15).

pub mod archive;
pub mod catalog;
pub mod index;
pub mod integrity;
pub mod licence;
pub mod module;
pub mod search;
pub mod tree;
#[cfg(test)]
mod tests;

pub use archive::{read_files, ArchiveError, MAX_ENTRY_BYTES};
pub use catalog::{icon_set, set_number, IconSet, SetStyle, ICON_SETS};
pub use index::{StoreIndex, StoredSet};
pub use licence::set_attribution;
pub use integrity::{matches_integrity, sha512_integrity};
pub use module::{icon_name, parse_module, ModuleIcon};
pub use search::{name_tokens, SymbolIndex, SEARCH_LIMIT, SET_TERM};
pub use tree::{svg_attr_name, to_svg, ConvertError, IconNode};
