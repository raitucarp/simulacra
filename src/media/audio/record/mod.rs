//! N11: the microphone recorder. [`Recorder::start`] opens the default microphone on its own
//! thread and records 16 kHz mono 16-bit PCM until [`Recorder::finish`] (or the ten-minute
//! cap); `finish` hands back the take as WAV bytes. Dropping a recorder discards the take.
//!
//! On Windows the system's WinMM wave-in API records (declared here, linked from `winmm.dll`:
//! no new dependency). Other platforms have no recorder yet: `start` says so and the voice
//! sheet offers choosing an audio file instead.

#[cfg(target_os = "windows")]
mod wave_api;
#[cfg(target_os = "windows")]
mod winmm;

use crate::domain::post::voice::{wav_bytes, MAX_RECORDING_MS, RECORD_CHANNELS, RECORD_SAMPLE_RATE};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
use std::time::Instant;

const MS_PER_SECOND: u64 = 1000;

/// Why a recording did not start or did not finish.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordError {
    /// This platform has no recorder (choose an audio file instead).
    Unavailable,
    /// No microphone could be opened (the system's code).
    NoMicrophone(u32),
    /// The recording broke off.
    Failed(String),
}

/// What the recording thread hands back: the samples, or why there are none.
type Take = Result<Vec<i16>, RecordError>;

/// A recording under way.
pub struct Recorder {
    stop: Arc<AtomicBool>,
    worker: Option<JoinHandle<Take>>,
    started: Instant,
}

/// Whether this platform can record at all.
pub const fn can_record() -> bool {
    cfg!(target_os = "windows")
}

/// The most samples one take holds.
fn max_samples() -> usize {
    (MAX_RECORDING_MS * u64::from(RECORD_SAMPLE_RATE) / MS_PER_SECOND) as usize * usize::from(RECORD_CHANNELS)
}

#[cfg(target_os = "windows")]
fn record(stop: &AtomicBool, opened: mpsc::SyncSender<Result<(), RecordError>>) -> Take {
    winmm::capture(stop, opened, max_samples())
}

#[cfg(not(target_os = "windows"))]
fn record(_: &AtomicBool, opened: mpsc::SyncSender<Result<(), RecordError>>) -> Take {
    let _ = opened.send(Err(RecordError::Unavailable));
    let _ = max_samples();
    Err(RecordError::Unavailable)
}

impl Recorder {
    /// Opens the default microphone and starts recording.
    pub fn start() -> Result<Self, RecordError> {
        let stop = Arc::new(AtomicBool::new(false));
        let (opened, ready) = mpsc::sync_channel(1);
        let flag = stop.clone();
        let worker = std::thread::Builder::new().name("voice-recorder".into()).spawn(move || record(&flag, opened)).map_err(|err| RecordError::Failed(err.to_string()))?;
        match ready.recv() {
            Ok(Ok(())) => Ok(Self { stop, worker: Some(worker), started: Instant::now() }),
            Ok(Err(err)) => Err(err),
            Err(_) => Err(worker.join().ok().and_then(Result::err).unwrap_or(RecordError::Unavailable)),
        }
    }

    /// How long it has been recording.
    pub fn elapsed_ms(&self) -> u64 {
        (self.started.elapsed().as_millis() as u64).min(MAX_RECORDING_MS)
    }

    /// Stops and returns the take as a WAV file.
    pub fn finish(mut self) -> Result<Vec<u8>, RecordError> {
        self.stop.store(true, Ordering::SeqCst);
        let worker = self.worker.take().ok_or(RecordError::Unavailable)?;
        let samples = worker.join().map_err(|_| RecordError::Failed("the recorder stopped".into()))??;
        Ok(wav_bytes(&samples, RECORD_SAMPLE_RATE, RECORD_CHANNELS))
    }
}

impl Drop for Recorder {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
    }
}
