//! Windows: a run is a job object. Win32 is only reachable through `unsafe` calls; each is
//! commented, and the job's handle is closed when the tree is dropped (which also ends any
//! process still in it).

use std::io;
use std::os::windows::io::AsRawHandle;
use std::process::{Child, Command};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation, SetInformationJobObject, TerminateJobObject, JOBOBJECT_EXTENDED_LIMIT_INFORMATION,
    JOB_OBJECT_LIMIT_ACTIVE_PROCESS, JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION, JOB_OBJECT_LIMIT_JOB_MEMORY, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
};

/// Processes a run may have at once: a compiler starts a few, a fork bomb is stopped.
const MAX_PROCESSES: u32 = 64;
/// Memory all processes of a run may commit together (2 GiB).
const MAX_MEMORY: usize = 2 << 30;
/// The exit code of processes ended with the job.
const KILLED_EXIT_CODE: u32 = 1;

/// The job object holding a run.
pub struct ProcessTree {
    job: HANDLE,
}

fn io_error(err: windows::core::Error) -> io::Error {
    io::Error::other(err)
}

impl ProcessTree {
    /// A new job that ends every process in it when closed, with the run's limits.
    pub fn new() -> io::Result<Self> {
        // SAFETY: no security attributes and no name: a fresh, unnamed job this value owns.
        let job = unsafe { CreateJobObjectW(None, PCWSTR::null()) }.map_err(io_error)?;
        let tree = Self { job };
        let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
        limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION | JOB_OBJECT_LIMIT_ACTIVE_PROCESS | JOB_OBJECT_LIMIT_JOB_MEMORY;
        limits.BasicLimitInformation.ActiveProcessLimit = MAX_PROCESSES;
        limits.JobMemoryLimit = MAX_MEMORY;
        let size = std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32;
        // SAFETY: `limits` is the struct of the class passed, of the size passed, alive for the call.
        unsafe { SetInformationJobObject(tree.job, JobObjectExtendedLimitInformation, (&raw const limits).cast(), size) }.map_err(io_error)?;
        Ok(tree)
    }

    /// Nothing to set before the start on Windows (the job is joined right after).
    pub fn prepare(&self, _command: &mut Command) {}

    /// Puts the started program in the job; what it starts from then on joins it too.
    pub fn adopt(&self, child: &Child) -> io::Result<()> {
        // SAFETY: the job handle is open while `self` lives; the process handle is the
        // child's own, open while `child` lives.
        unsafe { AssignProcessToJobObject(self.job, HANDLE(child.as_raw_handle())) }.map_err(io_error)
    }

    /// Ends every process of the run, the program first if it is somehow outside the job.
    pub fn kill(&self, child: &mut Child) {
        // SAFETY: the job handle is open while `self` lives.
        let _ = unsafe { TerminateJobObject(self.job, KILLED_EXIT_CODE) };
        let _ = child.kill();
    }
}

impl Drop for ProcessTree {
    fn drop(&mut self) {
        // SAFETY: the handle came from `CreateJobObjectW` and is closed only here; closing it
        // ends whatever is left in the job (`KILL_ON_JOB_CLOSE`).
        let _ = unsafe { CloseHandle(self.job) };
    }
}
