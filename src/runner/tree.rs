//! A run's process tree: the program and everything it starts, ended together.
//!
//! - Windows: a job object. The program is put in it right after it starts and every process
//!   it starts joins it; the job is ended as a whole, and closing its handle ends anything
//!   left. It also caps the processes and memory of a run.
//! - Unix: a process group. The program leads a new group, and the group is killed as a whole.

#[cfg(windows)]
#[path = "tree_windows.rs"]
mod platform;
#[cfg(unix)]
#[path = "tree_unix.rs"]
mod platform;

pub use platform::ProcessTree;
