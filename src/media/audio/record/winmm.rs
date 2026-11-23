//! N11: recording through WinMM's wave-in API (`winmm.dll`, part of every Windows install). A
//! few buffers of a quarter second are queued on the default microphone; the recording thread
//! polls them in order, keeps what each recorded and queues it again, until it is told to
//! stop. The declarations are in `wave_api`.

use super::wave_api::*;
use super::RecordError;
use crate::domain::post::voice::{RECORD_CHANNELS, RECORD_SAMPLE_RATE};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::SyncSender;
use std::time::Duration;

const BUFFERS: usize = 4;
/// A buffer holds a quarter of a second.
const BUFFERS_PER_SECOND: usize = 4;
const POLL: Duration = Duration::from_millis(30);

/// An open microphone with its queued buffers (they stay put while queued).
struct Capture {
    handle: WaveIn,
    buffers: Vec<Vec<u8>>,
    headers: Vec<WaveHeader>,
    next: usize,
    samples: Vec<i16>,
}

impl Capture {
    fn open() -> Result<Self, RecordError> {
        let mut handle: WaveIn = std::ptr::null_mut();
        // SAFETY: `handle` and the format outlive the call; no callback is registered.
        let status = unsafe { waveInOpen(&mut handle, WAVE_MAPPER, &format(), 0, 0, CALLBACK_NULL) };
        if status != NO_ERROR {
            return Err(RecordError::NoMicrophone(status));
        }
        let length = (RECORD_SAMPLE_RATE as usize / BUFFERS_PER_SECOND) * usize::from(RECORD_CHANNELS) * 2;
        let mut capture = Self { handle, buffers: vec![vec![0u8; length]; BUFFERS], headers: Vec::new(), next: 0, samples: Vec::new() };
        for buffer in &mut capture.buffers {
            let header = WaveHeader { data: buffer.as_mut_ptr(), buffer_length: length as u32, bytes_recorded: 0, user: 0, flags: 0, loops: 0, next: std::ptr::null_mut(), reserved: 0 };
            capture.headers.push(header);
        }
        match capture.queue_all() {
            Ok(()) => Ok(capture),
            Err(err) => {
                capture.close();
                Err(err)
            }
        }
    }

    /// Prepares and queues every buffer, then starts recording.
    fn queue_all(&mut self) -> Result<(), RecordError> {
        for index in 0..BUFFERS {
            let header: *mut WaveHeader = &mut self.headers[index];
            // SAFETY: the header and its buffer live in `self` until it is closed.
            let status = unsafe { waveInPrepareHeader(self.handle, header, HEADER_SIZE) };
            // SAFETY: as above, and the header is prepared.
            let status = if status == NO_ERROR { unsafe { waveInAddBuffer(self.handle, header, HEADER_SIZE) } } else { status };
            if status != NO_ERROR {
                return Err(RecordError::Failed(format!("wave-in buffer {status}")));
            }
        }
        // SAFETY: the device is open with its buffers queued.
        match unsafe { waveInStart(self.handle) } {
            NO_ERROR => Ok(()),
            status => Err(RecordError::Failed(format!("wave-in start {status}"))),
        }
    }

    /// Keeps what the finished buffers recorded, in order; queues each again when `requeue`.
    fn drain(&mut self, requeue: bool) {
        for _ in 0..BUFFERS {
            let header: *mut WaveHeader = &mut self.headers[self.next];
            // SAFETY: the driver writes these fields; they are read without caching.
            let (flags, recorded) = unsafe { (std::ptr::read_volatile(&raw const (*header).flags), std::ptr::read_volatile(&raw const (*header).bytes_recorded)) };
            if flags & WHDR_DONE == 0 {
                return;
            }
            let bytes = &self.buffers[self.next][..(recorded as usize).min(self.buffers[self.next].len())];
            self.samples.extend(bytes.chunks_exact(2).map(|pair| i16::from_le_bytes([pair[0], pair[1]])));
            // SAFETY: the buffer is done, so it is the program's again until it is queued.
            unsafe {
                std::ptr::write_volatile(&raw mut (*header).flags, flags & !WHDR_DONE);
                if requeue {
                    waveInAddBuffer(self.handle, header, HEADER_SIZE);
                }
            }
            self.next = (self.next + 1) % BUFFERS;
        }
    }

    /// Stops, keeps what is left and closes the device.
    fn close(mut self) -> Vec<i16> {
        // SAFETY: reset returns every queued buffer as done; then they are unprepared and freed after close.
        unsafe { waveInReset(self.handle) };
        self.drain(false);
        for header in &mut self.headers {
            let header: *mut WaveHeader = header;
            // SAFETY: no buffer is queued after the reset.
            unsafe { waveInUnprepareHeader(self.handle, header, HEADER_SIZE) };
        }
        // SAFETY: nothing refers to the device any more.
        unsafe { waveInClose(self.handle) };
        std::mem::take(&mut self.samples)
    }
}

/// Records until `stop` is set or `max_samples` are in; says on `opened` whether it started.
pub(super) fn capture(stop: &AtomicBool, opened: SyncSender<Result<(), RecordError>>, max_samples: usize) -> Result<Vec<i16>, RecordError> {
    let mut capture = match Capture::open() {
        Ok(capture) => capture,
        Err(err) => {
            let _ = opened.send(Err(err.clone()));
            return Err(err);
        }
    };
    let _ = opened.send(Ok(()));
    while !stop.load(Ordering::SeqCst) && capture.samples.len() < max_samples {
        std::thread::sleep(POLL);
        capture.drain(true);
    }
    let mut samples = capture.close();
    samples.truncate(max_samples);
    Ok(samples)
}
