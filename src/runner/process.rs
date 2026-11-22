//! Running one program to its end, its output captured: stdout and stderr read together into
//! one buffer up to a cap, stopped when the time runs out, the cap is passed or the caller
//! cancels. Programs start with their arguments as a list, never through a shell.

use std::io::Read;
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::time::{Duration, Instant};

/// How often the child is checked while it runs.
const POLL: Duration = Duration::from_millis(10);
/// How long the output readers get after the child ends (a pipe held open by something else
/// is given up on, not waited for).
const READ_GRACE: Duration = Duration::from_secs(2);
/// Bytes read at a time.
const CHUNK: usize = 8 * 1024;

/// The bounds of a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub timeout: Duration,
    /// Most output bytes kept; passing it stops the run.
    pub output_cap: usize,
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum End {
    Exited,
    TimedOut,
    Cancelled,
    OutputCapped,
}

/// What a run printed and how it ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Captured {
    /// stdout and stderr as they arrived (lossy UTF-8), at most the cap.
    pub output: String,
    /// The exit code, when the program exited by itself with one.
    pub exit_code: Option<i32>,
    pub end: End,
    pub duration: Duration,
}

/// Output shared by the two readers.
#[derive(Default)]
struct Sink {
    bytes: Vec<u8>,
    capped: bool,
}

/// Runs `command` within `limits` (stdin is empty, stdout and stderr are captured); `cancel`
/// stops it early. `stop` ends the child and everything it started.
pub fn run(mut command: Command, limits: Limits, cancel: &AtomicBool, stop: impl Fn(&mut Child)) -> std::io::Result<Captured> {
    let started = Instant::now();
    let mut child = command.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let sink = Arc::new(Mutex::new(Sink::default()));
    let (done_tx, done_rx) = mpsc::channel();
    let readers = [child.stdout.take().map(|out| Box::new(out) as Box<dyn Read + Send>), child.stderr.take().map(|err| Box::new(err) as Box<dyn Read + Send>)];
    let count = readers.iter().flatten().count();
    for reader in readers.into_iter().flatten() {
        let (sink, done) = (sink.clone(), done_tx.clone());
        std::thread::spawn(move || {
            drain(reader, &sink, limits.output_cap);
            let _ = done.send(());
        });
    }
    let (end, status) = watch(&mut child, limits, cancel, &sink, &stop);
    let grace = Instant::now() + READ_GRACE;
    for _ in 0..count {
        let _ = done_rx.recv_timeout(grace.saturating_duration_since(Instant::now()));
    }
    let output = String::from_utf8_lossy(&sink.lock().map(|sink| sink.bytes.clone()).unwrap_or_default()).into_owned();
    let exit_code = status.filter(|_| end == End::Exited);
    Ok(Captured { output, exit_code, end, duration: started.elapsed() })
}

/// Waits for the child, stopping it on time-out, cancel or a full sink. Returns how it ended
/// and its exit code, when it had one.
fn watch(child: &mut Child, limits: Limits, cancel: &AtomicBool, sink: &Mutex<Sink>, stop: &impl Fn(&mut Child)) -> (End, Option<i32>) {
    let deadline = Instant::now() + limits.timeout;
    loop {
        if let Ok(Some(status)) = child.try_wait() {
            return (End::Exited, status.code());
        }
        let capped = sink.lock().is_ok_and(|sink| sink.capped);
        let end = if capped { Some(End::OutputCapped) } else if cancel.load(Ordering::Relaxed) { Some(End::Cancelled) } else { (Instant::now() >= deadline).then_some(End::TimedOut) };
        if let Some(end) = end {
            stop(child);
            let _ = child.wait();
            return (end, None);
        }
        std::thread::sleep(POLL);
    }
}

/// Reads `reader` into `sink` until it closes; past `cap` bytes it marks the sink and stops.
fn drain(mut reader: Box<dyn Read + Send>, sink: &Mutex<Sink>, cap: usize) {
    let mut chunk = vec![0; CHUNK];
    while let Ok(read) = reader.read(&mut chunk) {
        if read == 0 {
            return;
        }
        let Ok(mut sink) = sink.lock() else { return };
        let room = cap.saturating_sub(sink.bytes.len());
        sink.bytes.extend_from_slice(&chunk[..read.min(room)]);
        if read > room {
            sink.capped = true;
            return;
        }
    }
}
