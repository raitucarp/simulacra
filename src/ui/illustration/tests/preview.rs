//! Manual preview: `SIMULACRA_PREVIEW_DIR=<dir> cargo test --lib render_avatar_previews -- --ignored`
//! writes composed portraits and emblems as PNGs for eyeballing.

use super::super::portrait::compose_portrait_template;
use super::super::*;
use super::{default_colors, parse};
use crate::domain::portrait::{EmblemSpec, PortraitSpec};
use crate::domain::traits::{ClassicTemperament, EnneagramType, MbtiType};
use std::path::{Path, PathBuf};

const PREVIEW_ENV: &str = "SIMULACRA_PREVIEW_DIR";
const PREVIEW_SIZE: u32 = 256;
const PREVIEW_COUNT: u64 = 6;

fn save(svg: &str, path: PathBuf) {
    let tree = parse(svg, "preview");
    let mut pixmap = resvg::tiny_skia::Pixmap::new(PREVIEW_SIZE, PREVIEW_SIZE).unwrap();
    let scale = PREVIEW_SIZE as f32 / tree.size().width();
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    pixmap.save_png(path).unwrap();
}

fn save_emblems(dir: &Path) {
    let personalities = [
        (MbtiType::Entj, EnneagramType::Type8w7, ClassicTemperament::Choleric),
        (MbtiType::Infp, EnneagramType::Type4w5, ClassicTemperament::Melancholic),
        (MbtiType::Esfp, EnneagramType::Type7w6, ClassicTemperament::Sanguine),
        (MbtiType::Istj, EnneagramType::Type1w9, ClassicTemperament::Phlegmatic),
        (MbtiType::Intp, EnneagramType::Type5w4, ClassicTemperament::MelancholicCholeric),
        (MbtiType::Enfj, EnneagramType::Type2w3, ClassicTemperament::PhlegmaticSanguine),
    ];
    for (i, (mbti, enneagram, temperament)) in personalities.into_iter().enumerate() {
        let svg = EmblemSpec::from_traits(mbti, enneagram, temperament).to_svg_template();
        save(&fill_template(&svg, &default_colors(), &TemplateExtras::none()), dir.join(format!("emblem_{i}.png")));
    }
}

#[test]
#[ignore = "writes preview PNGs; run manually"]
fn render_avatar_previews() {
    let Ok(dir) = std::env::var(PREVIEW_ENV) else { return };
    let dir = PathBuf::from(dir);
    std::fs::create_dir_all(&dir).unwrap();
    for seed in 0..PREVIEW_COUNT {
        let spec = PortraitSpec::random(seed);
        let extras = TemplateExtras::portrait(spec.skin, spec.hair_color);
        let svg = fill_template(&compose_portrait_template(&spec), &default_colors(), &extras);
        save(&svg, dir.join(format!("portrait_{seed}.png")));
    }
    save_emblems(&dir);
}
