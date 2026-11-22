//! Local files attached to posts (epic L), at the edge: reading what a post keeps about a
//! file (its size, date, type and hash), scanning it for its card (its length, a poster frame
//! from the OS) and handing files to the system file manager.

mod moved;
mod open;
pub mod poster;
mod read;
mod scan;
mod source;

pub use moved::find_renamed;
pub use open::{open_file, open_folder, show_in_folder, FILE_MANAGER};
pub use read::read_file_ref;
pub use scan::scan_file;
pub use source::FileBytes;
