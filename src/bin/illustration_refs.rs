//! `illustration-refs` (WS5): renders every Classic illustration to PNG, for use as the
//! reference image when a pack is made (`tools/illustrations/README.md`).
//!
//! `cargo run --features codegen --bin illustration-refs -- <out dir> [--matte RRGGBB] [--scale N] [--only PREFIX]...`
//!
//! Each template is filled by the app's own filler with the default dark palette (layered
//! families in the key colours) and rendered with resvg at `scale`× its canvas (2× by
//! default) to `<out dir>/<key>.png`, transparent, or over a solid `--matte` colour.

use simulacra::domain::packs::key_of_asset_path;
use simulacra::ui::assets::EmbeddedAssets;
use simulacra::ui::illustration::pack::reference_svg;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const USAGE: &str = "usage: illustration-refs <out dir> [--matte RRGGBB] [--scale N] [--only PREFIX]...";
const DEFAULT_SCALE: f32 = 2.0;

/// What to render, and how.
struct Options {
    out: PathBuf,
    matte: Option<[u8; 3]>,
    scale: f32,
    only: Vec<String>,
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Options, String> {
    let out = args.next().filter(|arg| !arg.starts_with("--")).ok_or(USAGE)?;
    let mut options = Options { out: PathBuf::from(out), matte: None, scale: DEFAULT_SCALE, only: Vec::new() };
    while let Some(flag) = args.next() {
        let value = args.next().ok_or_else(|| format!("{flag} needs a value\n{USAGE}"))?;
        match flag.as_str() {
            "--matte" => options.matte = Some(simulacra::domain::packs::tint::parse_hex(&format!("#{}", value.trim_start_matches('#'))).ok_or("--matte takes RRGGBB")?),
            "--scale" => options.scale = value.parse().ok().filter(|scale: &f32| *scale > 0.0).ok_or("--scale takes a positive number")?,
            "--only" => options.only.push(value),
            _ => return Err(format!("unknown flag {flag}\n{USAGE}")),
        }
    }
    Ok(options)
}

/// RGBA pixels of `svg` rendered at `scale`× its canvas, over `matte` when given.
fn render(svg: &str, scale: f32, matte: Option<[u8; 3]>) -> Result<image::RgbaImage, String> {
    let tree = resvg::usvg::Tree::from_str(svg, &resvg::usvg::Options::default()).map_err(|e| e.to_string())?;
    let size = tree.size();
    let (width, height) = ((size.width() * scale).round() as u32, (size.height() * scale).round() as u32);
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height).ok_or("empty canvas")?;
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    let mut out = image::RgbaImage::new(width, height);
    for (pixel, source) in out.pixels_mut().zip(pixmap.pixels()) {
        let color = source.demultiply();
        let rgba = [color.red(), color.green(), color.blue(), color.alpha()];
        pixel.0 = matte.map_or(rgba, |matte| over(rgba, matte));
    }
    Ok(out)
}

/// `rgba` composited over an opaque `matte`.
fn over([r, g, b, a]: [u8; 4], matte: [u8; 3]) -> [u8; 4] {
    let alpha = f32::from(a) / 255.0;
    let mix = |top: u8, bottom: u8| (f32::from(top) * alpha + f32::from(bottom) * (1.0 - alpha)).round() as u8;
    [mix(r, matte[0]), mix(g, matte[1]), mix(b, matte[2]), u8::MAX]
}

/// Renders one Classic key into `out`.
fn write_ref(path: &str, key: &str, options: &Options) -> Result<(), String> {
    let svg = reference_svg(path).ok_or("unreadable template")?;
    let image = render(&svg, options.scale, options.matte)?;
    let file = key.split('/').fold(options.out.clone(), |dir, segment| dir.join(segment)).with_extension("png");
    std::fs::create_dir_all(file.parent().unwrap_or(Path::new("."))).map_err(|e| e.to_string())?;
    image.save(&file).map_err(|e| e.to_string())
}

fn main() -> ExitCode {
    let options = match parse_args(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::FAILURE;
        }
    };
    let (mut written, mut failed) = (0usize, 0usize);
    for path in EmbeddedAssets::iter() {
        let Some(key) = key_of_asset_path(&path) else { continue };
        if !options.only.is_empty() && !options.only.iter().any(|prefix| key.starts_with(prefix.as_str())) {
            continue;
        }
        match write_ref(&path, key, &options) {
            Ok(()) => written += 1,
            Err(err) => {
                eprintln!("illustration-refs: {key}: {err}");
                failed += 1;
            }
        }
    }
    println!("illustration-refs: {written} references written to {} ({failed} failed)", options.out.display());
    if failed == 0 { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
