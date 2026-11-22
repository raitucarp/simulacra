//! Local files attached to posts (epic L), at the edge: reading what a post keeps about a
//! file (its size, date, type and hash) and handing files to the system file manager.

mod open;
mod read;

pub use open::{open_file, open_folder, show_in_folder, FILE_MANAGER};
pub use read::read_file_ref;
