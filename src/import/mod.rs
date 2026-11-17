//! Universal import and capture, as a pipeline of small pure stages with side effects only at
//! the end:
//!
//! 1. [`detect`]: the format of a file from its bytes, name and MIME type.
//! 2. [`parse`]: bytes into a [`ParsedDocument`] (metadata plus sections of blocks).
//! 3. [`segment`]: a document into units (chapters, paragraphs or sentences).
//! 4. [`plan`]: selected units into post drafts with citations, and the posts themselves.
//! 5. [`apply`]: the vault document (through [`vault_adapter`]), the original file and the posts.
//!
//! [`capture`] classifies quick-capture input (a note, a link) for the Inbox.

pub mod apply;
pub mod capture;
pub mod capture_posts;
pub mod detect;
pub mod document;
#[cfg(test)]
mod fixtures;
pub mod load;
pub mod markup;
pub mod parse;
pub mod plan;
pub mod segment;
pub mod vault_adapter;

pub use detect::{detect_format, Format, FormatFamily};
pub use load::{load_bytes, load_file, LoadError, LoadedFile};
pub use document::{Block, BlockKind, DocumentMetadata, FileFacts, ParseNote, ParsedDocument, Section};
pub use parse::parse_document;
pub use segment::{segment_document, Granularity, Unit, UnitPosition};
