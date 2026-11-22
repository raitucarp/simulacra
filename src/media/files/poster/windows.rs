//! L6 on Windows: the Shell's thumbnail of a file (`IShellItemImageFactory`), the frame File
//! Explorer shows, copied out as RGBA. COM and GDI are only reachable through `unsafe` calls;
//! each one is commented, and every handle is released before returning.

use image::RgbaImage;
use std::path::Path;
use windows::core::HSTRING;
use windows::Win32::Foundation::SIZE;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HGDIOBJ,
};
use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::{IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_BIGGERSIZEOK, SIIGBF_THUMBNAILONLY};

/// Bytes per pixel of the 32-bit bitmap copied out.
const BYTES_PER_PIXEL: usize = 4;
const BITS_PER_PIXEL: u16 = 32;

/// The Shell's thumbnail of the file at `path`, about `edge` pixels on its longer side; `None`
/// when no thumbnail handler knows the file. Call it on a thread of its own (it joins COM).
pub(super) fn shell_thumbnail(path: &Path, edge: u32) -> Option<RgbaImage> {
    // SAFETY: joining a single-threaded COM apartment has no preconditions; it is left below
    // only when this call joined it (a thread already in an apartment keeps it).
    let joined = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) }.is_ok();
    let image = thumbnail(path, edge);
    if joined {
        // SAFETY: balances the successful `CoInitializeEx` above on the same thread.
        unsafe { CoUninitialize() };
    }
    image
}

fn thumbnail(path: &Path, edge: u32) -> Option<RgbaImage> {
    let side = i32::try_from(edge).ok()?;
    // SAFETY: the path is a valid wide string for the call; the factory is reference-counted.
    let factory: IShellItemImageFactory = unsafe { SHCreateItemFromParsingName(&HSTRING::from(path.as_os_str()), None) }.ok()?;
    // SAFETY: `GetImage` returns a bitmap we own, released right after it is copied.
    let bitmap = unsafe { factory.GetImage(SIZE { cx: side, cy: side }, SIIGBF_THUMBNAILONLY | SIIGBF_BIGGERSIZEOK) }.ok()?;
    let pixels = copy_out(bitmap);
    // SAFETY: the bitmap came from `GetImage` and is not used after this.
    unsafe {
        let _ = DeleteObject(HGDIOBJ(bitmap.0));
    }
    pixels
}

/// The bitmap's pixels as top-down RGBA.
fn copy_out(bitmap: HBITMAP) -> Option<RgbaImage> {
    let mut facts = BITMAP::default();
    // SAFETY: `facts` is a `BITMAP` of the size passed.
    let read = unsafe { GetObjectW(HGDIOBJ(bitmap.0), std::mem::size_of::<BITMAP>() as i32, Some((&raw mut facts).cast())) };
    let (width, height) = (u32::try_from(facts.bmWidth).ok()?, facts.bmHeight.unsigned_abs());
    if read == 0 || width == 0 || height == 0 {
        return None;
    }
    let header = BITMAPINFOHEADER {
        biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: facts.bmWidth,
        biHeight: -(height as i32),
        biPlanes: 1,
        biBitCount: BITS_PER_PIXEL,
        biCompression: BI_RGB.0,
        ..Default::default()
    };
    let mut info = BITMAPINFO { bmiHeader: header, ..Default::default() };
    let mut bgra = vec![0u8; width as usize * height as usize * BYTES_PER_PIXEL];
    // SAFETY: a memory device context, deleted below; `bgra` holds `height` rows of 32-bit
    // pixels, the format `info` asks for.
    let copied = unsafe {
        let dc = CreateCompatibleDC(None);
        let lines = GetDIBits(dc, bitmap, 0, height, Some(bgra.as_mut_ptr().cast()), &mut info, DIB_RGB_COLORS);
        let _ = DeleteDC(dc);
        lines
    };
    (copied == height as i32).then(|| RgbaImage::from_raw(width, height, to_rgba(bgra))).flatten()
}

/// BGRA rows as RGBA; a bitmap with no alpha at all is opaque.
fn to_rgba(mut pixels: Vec<u8>) -> Vec<u8> {
    let opaque = pixels.chunks_exact(BYTES_PER_PIXEL).all(|pixel| pixel[3] == 0);
    for pixel in pixels.chunks_exact_mut(BYTES_PER_PIXEL) {
        pixel.swap(0, 2);
        if opaque {
            pixel[3] = u8::MAX;
        }
    }
    pixels
}
