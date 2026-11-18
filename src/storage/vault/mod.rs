//! The vault worker: a dedicated thread owning the SQLite connection.
//!
//! Writes are queued and executed off the UI thread in submission order; a failed write is
//! logged and delivered on the failure channel ([`Vault::take_failures`]) so the shell can
//! toast it. Reads are synchronous round trips through the same queue, so they always observe
//! every earlier write. Dropping the last handle flushes the queue and joins the worker.

mod catalogue_ops;
mod embeddings;
mod journal;
mod media;
mod ops;
mod review;
mod thinking;
#[cfg(test)]
mod tests;

use super::Database;
pub use thinking::ThinkingRows;
use crate::error::{AppError, Result};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;
use std::thread::JoinHandle;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

/// Name of the vault worker OS thread.
const VAULT_THREAD_NAME: &str = "simulacra-vault";

/// A unit of work run on the worker thread against the connection.
type Job = Box<dyn FnOnce(&Database) + Send>;

/// Error returned when the worker thread is gone (it never is while a handle lives).
fn worker_gone() -> AppError {
    AppError::DatabaseError(VAULT_THREAD_NAME.to_string())
}

struct Worker {
    jobs: Option<mpsc::Sender<Job>>,
    thread: Option<JoinHandle<()>>,
    path: String,
    failure_tx: UnboundedSender<AppError>,
    failures: RefCell<Option<UnboundedReceiver<AppError>>>,
}

impl Drop for Worker {
    /// Closes the queue and waits until every pending write reached SQLite.
    fn drop(&mut self) {
        drop(self.jobs.take());
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

/// Cloneable handle to the vault worker.
#[derive(Clone)]
pub struct Vault(Rc<Worker>);

impl Vault {
    /// Moves `db` onto a dedicated worker thread.
    pub fn spawn(db: Database) -> Result<Self> {
        let path = db.path();
        let (jobs, queue) = mpsc::channel::<Job>();
        let thread = std::thread::Builder::new()
            .name(VAULT_THREAD_NAME.into())
            .spawn(move || queue.into_iter().for_each(|job| job(&db)))
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        let (failure_tx, failures) = unbounded_channel();
        let worker = Worker {
            jobs: Some(jobs),
            thread: Some(thread),
            path,
            failure_tx,
            failures: RefCell::new(Some(failures)),
        };
        Ok(Self(Rc::new(worker)))
    }

    /// File path of the vault (empty for an in-memory vault).
    pub fn path(&self) -> String {
        self.0.path.clone()
    }

    /// The receiver of failed writes; `None` once taken.
    pub fn take_failures(&self) -> Option<UnboundedReceiver<AppError>> {
        self.0.failures.borrow_mut().take()
    }

    fn submit(&self, job: Job) -> Result<()> {
        let jobs = self.0.jobs.as_ref().ok_or_else(worker_gone)?;
        jobs.send(job).map_err(|_| worker_gone())
    }

    /// Queues `write`; its error, if any, is logged and sent to the failure channel.
    pub fn write(&self, write: impl FnOnce(&Database) -> Result<()> + Send + 'static) -> Result<()> {
        let failures = self.0.failure_tx.clone();
        self.submit(Box::new(move |db| {
            if let Err(err) = write(db) {
                eprintln!("simulacra: vault write failed: {err}");
                let _ = failures.send(err);
            }
        }))
    }

    /// Runs `read` on the worker after every queued write and returns its result.
    pub fn read<T: Send + 'static>(&self, read: impl FnOnce(&Database) -> Result<T> + Send + 'static) -> Result<T> {
        let (reply, answer) = mpsc::channel();
        self.submit(Box::new(move |db| {
            let _ = reply.send(read(db));
        }))?;
        answer.recv().map_err(|_| worker_gone())?
    }

    /// Blocks until every write queued so far has been applied.
    pub fn flush(&self) -> Result<()> {
        self.read(|_| Ok(()))
    }
}
