//! N11: the few WinMM (`winmm.dll`) declarations the recorder uses: the wave-in calls, the
//! two structures they take and their constants, with the recording format.

use crate::domain::post::voice::{RECORD_CHANNELS, RECORD_SAMPLE_RATE};
use std::ffi::c_void;

pub(super) const WAVE_MAPPER: u32 = u32::MAX;
pub(super) const CALLBACK_NULL: u32 = 0;
pub(super) const NO_ERROR: u32 = 0;
pub(super) const WHDR_DONE: u32 = 1;
pub(super) const PCM_FORMAT: u16 = 1;
pub(super) const BITS_PER_SAMPLE: u16 = 16;

/// `WAVEFORMATEX` (its field offsets; the last field is `cbSize`).
#[repr(C)]
#[allow(dead_code)] // read by the system
pub(super) struct WaveFormat {
    pub(super) format_tag: u16,
    pub(super) channels: u16,
    pub(super) samples_per_sec: u32,
    pub(super) avg_bytes_per_sec: u32,
    pub(super) block_align: u16,
    pub(super) bits_per_sample: u16,
    pub(super) extra_size: u16,
}

/// `WAVEHDR`.
#[repr(C)]
#[allow(dead_code)] // read and written by the system
pub(super) struct WaveHeader {
    pub(super) data: *mut u8,
    pub(super) buffer_length: u32,
    pub(super) bytes_recorded: u32,
    pub(super) user: usize,
    pub(super) flags: u32,
    pub(super) loops: u32,
    pub(super) next: *mut WaveHeader,
    pub(super) reserved: usize,
}

pub(super) type WaveIn = *mut c_void;
pub(super) const HEADER_SIZE: u32 = std::mem::size_of::<WaveHeader>() as u32;

#[link(name = "winmm", kind = "raw-dylib")]
extern "system" {
    pub(super) fn waveInOpen(handle: *mut WaveIn, device: u32, format: *const WaveFormat, callback: usize, instance: usize, flags: u32) -> u32;
    pub(super) fn waveInPrepareHeader(handle: WaveIn, header: *mut WaveHeader, size: u32) -> u32;
    pub(super) fn waveInUnprepareHeader(handle: WaveIn, header: *mut WaveHeader, size: u32) -> u32;
    pub(super) fn waveInAddBuffer(handle: WaveIn, header: *mut WaveHeader, size: u32) -> u32;
    pub(super) fn waveInStart(handle: WaveIn) -> u32;
    pub(super) fn waveInReset(handle: WaveIn) -> u32;
    pub(super) fn waveInClose(handle: WaveIn) -> u32;
}

pub(super) fn format() -> WaveFormat {
    let block_align = RECORD_CHANNELS * BITS_PER_SAMPLE / 8;
    let channels = RECORD_CHANNELS;
    WaveFormat { format_tag: PCM_FORMAT, channels, samples_per_sec: RECORD_SAMPLE_RATE, avg_bytes_per_sec: RECORD_SAMPLE_RATE * u32::from(block_align), block_align, bits_per_sample: BITS_PER_SAMPLE, extra_size: 0 }
}

