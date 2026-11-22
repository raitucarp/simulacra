# L6: video poster frames without bundling a codec

Status: decided (issue #57). Used by the video file card (#59).

## Question

A video file attached to a post (#40) should show a still frame on its card. Decoding a
frame needs a video codec (H.264, HEVC, VP9, AV1 …), and bundling one means megabytes,
patent-encumbered formats and a second licence. Can each desktop OS hand us a poster frame
instead, and what does it cost in crates and code?

## What each OS offers

| | Windows 10 and 11 | macOS 11 and later | Linux desktops |
| --- | --- | --- | --- |
| API | Shell: `IShellItemImageFactory::GetImage`, served by the thumbnail handlers and codecs Windows has installed (the frame File Explorer shows) | Quick Look: `QLThumbnailGenerator`; the `qlmanage -t` tool every macOS ships drives the same generator | The freedesktop.org thumbnail specification: file managers save `$XDG_CACHE_HOME/thumbnails/{normal,large,x-large,xx-large}/<MD5 of the file URI>.png`; a D-Bus thumbnailer (Tumbler, `org.freedesktop.thumbnails.Thumbnailer1`) generates them on request when one runs |
| Formats covered | what Windows plays: MP4 / H.264, MOV, AVI, WMV out of the box; HEVC, VP9, AV1 and WebM with the Store extensions | what AVFoundation plays: MP4, MOV, M4V (not WebM or MKV) | what the installed thumbnailer handles (usually ffmpegthumbnailer or GStreamer: nearly everything) |
| What it takes | the `windows` crate with `Win32_UI_Shell`, `Win32_Graphics_Gdi` and `Win32_System_Com`. Version 0.62 is already in `Cargo.lock` through the UI toolkit's Windows platform crate, **with exactly these features already turned on**, so depending on it adds no crate and rebuilds nothing. The calls are COM and GDI, so they need `unsafe` | no crate: start `qlmanage -t -s <size> -o <dir> <file>` and read the PNG it writes (objc2 crates are in the lock, but not the QuickLookThumbnailing binding) | reading the cache needs only an MD5 of the `file://` URI; `md-5` 0.11 is already in the lock (the PDF reader uses it). Asking Tumbler to generate a missing one needs a D-Bus client (`zbus` is in the lock on Linux) and an asynchronous wait |

## Findings

- **Windows.** The Shell route gives the same frame File Explorer shows, for every format
  Windows can play, and the crate is free: same version and features the toolkit already
  builds. It is the app's first `unsafe` code: COM initialisation (on a thread of its own,
  so it never meets the toolkit's apartment), the image factory, and `GetDIBits` to copy the
  bitmap out. It is about seventy lines, each call commented, every handle released.
- **macOS.** `qlmanage` is a system tool on every supported macOS, like the `open` command
  the app already starts to show folders. It writes `<file name>.png` into the folder it is
  given. It takes a few hundred milliseconds to start, so it runs off the UI thread and its
  result is kept once per file version.
- **Linux.** Reading the cache is pure Rust and costs nothing when there is no thumbnail. It
  finds a poster for any video the user has already seen in a file manager (GNOME Files,
  Dolphin, Thunar and Nemo all write it). Generating a missing one needs D-Bus and is left
  out.
- **Durations** need no OS help: the container headers say how long a video plays (MP4 and
  QuickTime `mvhd`, Matroska and WebM `Info`, AVI `avih`). Small pure parsers read them, and
  the audio card (#62) uses the same ones (WAV, FLAC, Ogg, MP3, M4A).

## Decision

1. **No codec is bundled.** Posters come from the OS; durations from our own header parsers.
2. **Windows:** the Shell thumbnail, through `windows` 0.62 (MIT OR Apache-2.0), in one small
   module that holds all of the app's `unsafe` code.
3. **macOS:** `qlmanage -t`.
4. **Linux:** the freedesktop thumbnail cache, largest size first, named with `md-5` 0.11
   (MIT OR Apache-2.0). Nothing is generated.
5. Posters are saved in the media folder (`posters/`), named by the file's path, size and
   modification time, so a replaced file gets a new one.
6. **The fallback is the file card.** A video without a poster (a format the OS cannot
   thumbnail, no thumbnail in the Linux cache, a missing tool) shows the video frame with
   the video art, the play badge and the duration; opening it hands the file to the system
   player in every case.

## What is left

- Asking a Linux thumbnailer over D-Bus to make a missing thumbnail.
- The macOS and Linux routes are compiled and unit-tested on every platform (the tool's
  arguments and output name, the cache file name from the specification's own example), but
  were not run on those systems for this spike: there was no macOS or Linux machine at hand.
  The Windows route was run here, against a scratch image file.
