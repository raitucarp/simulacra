//! N11: playing a voice note back. A WAV note plays inside the app on Windows (WinMM's
//! `PlaySoundW`, asynchronously, one sound at a time, so a new one stops the last); any other
//! file, or any note elsewhere, opens in the system's player.

use std::path::Path;

/// How a note is being played.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Playback {
    /// In the app: it can be stopped, and ends on its own after its length.
    InApp,
    /// Handed to the system player.
    External,
}

#[cfg(target_os = "windows")]
mod sound {
    const SND_ASYNC: u32 = 0x0001;
    const SND_NODEFAULT: u32 = 0x0002;
    const SND_FILENAME: u32 = 0x0002_0000;

    #[link(name = "winmm", kind = "raw-dylib")]
    extern "system" {
        fn PlaySoundW(sound: *const u16, module: isize, flags: u32) -> i32;
    }

    /// Starts the WAV file at `path`; whether it started.
    pub(super) fn play(path: &std::path::Path) -> bool {
        use std::os::windows::ffi::OsStrExt;
        let wide: Vec<u16> = path.as_os_str().encode_wide().chain(std::iter::once(0)).collect();
        // SAFETY: `wide` is a NUL-terminated path that outlives the call (the system copies it).
        unsafe { PlaySoundW(wide.as_ptr(), 0, SND_FILENAME | SND_ASYNC | SND_NODEFAULT) != 0 }
    }

    /// Stops whatever the app is playing.
    pub(super) fn stop() {
        // SAFETY: a null sound stops the current one.
        unsafe { PlaySoundW(std::ptr::null(), 0, 0) };
    }
}

/// Plays the note at `path` (`wav`: it is a WAV file).
pub fn play(path: &Path, wav: bool) -> Result<Playback, String> {
    if !path.exists() {
        return Err(format!("{} is missing", path.display()));
    }
    #[cfg(target_os = "windows")]
    if wav && sound::play(path) {
        return Ok(Playback::InApp);
    }
    let _ = wav;
    crate::media::files::open_file(path).map(|()| Playback::External)
}

/// Stops a note playing in the app.
pub fn stop() {
    #[cfg(target_os = "windows")]
    sound::stop();
}
