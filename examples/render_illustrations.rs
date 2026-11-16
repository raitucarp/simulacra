//! Dev tool: renders every illustration template with a theme palette into PNG contact sheets.
//!
//! `cargo run --example render_illustrations -- <glob-dir> <theme-id> <out.png>`
//! e.g. `cargo run --example render_illustrations -- assets/illustrations/guide dracula guide.png`

use std::fs;
use std::path::Path;

const CELL: u32 = 160;
const PAD: u32 = 8;
const COLUMNS: u32 = 6;
const THEMES_DIR: &str = "assets/themes";

fn palette(theme_id: &str) -> Vec<(String, String)> {
    let raw = fs::read_to_string(format!("{THEMES_DIR}/{theme_id}.json")).expect("theme file");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("theme json");
    json["colors"]
        .as_object()
        .expect("colors object")
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
        .collect()
}

fn fill_template(svg: &str, colors: &[(String, String)]) -> String {
    colors.iter().fold(svg.to_string(), |acc, (key, hex)| {
        acc.replace(&format!("{{{{{key}}}}}"), hex)
    })
}

fn parse_hex(hex: &str) -> resvg::tiny_skia::Color {
    let v = u32::from_str_radix(hex.trim_start_matches('#'), 16).unwrap_or(0);
    resvg::tiny_skia::Color::from_rgba8((v >> 16) as u8, (v >> 8) as u8, v as u8, 255)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (dir, theme_id, out) = (&args[0], &args[1], &args[2]);
    let colors = palette(theme_id);
    let mut files: Vec<_> = fs::read_dir(dir)
        .expect("illustration dir")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "svg"))
        .collect();
    files.sort();
    let rows = (files.len() as u32).div_ceil(COLUMNS).max(1);
    let size = |n: u32| n * (CELL + PAD) + PAD;
    let mut sheet = resvg::tiny_skia::Pixmap::new(size(COLUMNS), size(rows)).expect("pixmap");
    let bg = colors.iter().find(|(k, _)| k == "bg").map(|(_, v)| v.as_str()).unwrap_or("#000");
    sheet.fill(parse_hex(bg));
    for (i, path) in files.iter().enumerate() {
        render_cell(&mut sheet, path, &colors, i as u32);
    }
    sheet.save_png(out).expect("write png");
    println!("rendered {} illustrations -> {out}", files.len());
}

fn render_cell(sheet: &mut resvg::tiny_skia::Pixmap, path: &Path, colors: &[(String, String)], i: u32) {
    let svg = fill_template(&fs::read_to_string(path).expect("svg"), colors);
    let tree = resvg::usvg::Tree::from_str(&svg, &resvg::usvg::Options::default())
        .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let scale = CELL as f32 / tree.size().width().max(tree.size().height());
    let (x, y) = ((i % COLUMNS) * (CELL + PAD) + PAD, (i / COLUMNS) * (CELL + PAD) + PAD);
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale).post_translate(x as f32, y as f32);
    resvg::render(&tree, transform, &mut sheet.as_mut());
}
