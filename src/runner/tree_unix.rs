//! Unix: a run is a process group the program leads; ending the run kills the group.

use std::io;
use std::os::unix::process::CommandExt;
use std::process::{Child, Command};

/// The process group holding a run.
pub struct ProcessTree;

impl ProcessTree {
    pub fn new() -> io::Result<Self> {
        Ok(Self)
    }

    /// The program will lead a new process group (its id is the program's pid).
    pub fn prepare(&self, command: &mut Command) {
        command.process_group(0);
    }

    /// The group is set at the start; nothing to do once it runs.
    pub fn adopt(&self, _child: &Child) -> io::Result<()> {
        Ok(())
    }

    /// Kills every process of the group, and the program itself.
    pub fn kill(&self, child: &mut Child) {
        if let Ok(group) = libc::pid_t::try_from(child.id()) {
            // SAFETY: a plain system call with no memory involved; the group is the one the
            // program was made to lead in `prepare`.
            unsafe { libc::killpg(group, libc::SIGKILL) };
        }
        let _ = child.kill();
    }
}
