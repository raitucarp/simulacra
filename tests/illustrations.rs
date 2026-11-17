//! Illustration guard: every trait variant has its option card, and every SVG template under
//! `assets/illustrations/` follows `assets/illustrations/STYLE.md` (palette placeholders only, parseable,
//! correct canvas, within the size budget).

use std::fs;
use std::path::{Path, PathBuf};

use simulacra::domain::traits::*;
use simulacra::agent::llm::LlmProvider;
use simulacra::domain::engine::{
    HttpMethod, ImageEngineKind, ImageSize, OutputKind, RequestKind, SchemaPreset, SdFlavor, SearchBackend, TextSource, ToolKind,
    TriggerKind,
};
use simulacra::domain::TraitOption;

const ASSETS_DIR: &str = "assets";
const ILLUSTRATIONS_DIR: &str = "assets/illustrations";
const SAMPLE_THEME: &str = "assets/themes/simulacra-midnight.json";
const PORTRAIT_FILLS: [(&str, &str); 4] = [("skin", "#e0ac7e"), ("hair", "#5b3a29"), ("primary", "#b8bec6"), ("secondary", "#d9534f")];
const ALLOWED_HEX: [&str; 2] = ["000", "fff"];
const CARD_LIMIT_BYTES: usize = 3 * 1024;
const WIDE_LIMIT_BYTES: usize = 6 * 1024;
const WIDE_KINDS: [&str; 3] = ["classes", "steps", "palettes"];
const FORBIDDEN_TAGS: [&str; 4] = ["<style", "<image", "<text", "<script"];

/// Expected `viewBox` per top-level folder; anything else is a trait option card.
const CANVASES: [(&str, &str); 8] = [
    ("steps", "0 0 320 120"),
    ("classes", "0 0 200 240"),
    ("palettes", "0 0 200 240"),
    ("guide", "0 0 128 128"),
    ("empty", "0 0 240 160"),
    ("portrait", "0 0 256 256"),
    ("forms", "0 0 256 256"),
    ("", "0 0 160 120"),
];

fn missing_cards<T: TraitOption>() -> Vec<String> {
    T::ALL
        .iter()
        .map(|option| option.illustration())
        .filter(|path| !Path::new(ASSETS_DIR).join(path).is_file())
        .collect()
}

macro_rules! missing_for {
    ($($ty:ty),+ $(,)?) => {{
        let mut missing = Vec::new();
        let mut total = 0usize;
        $( missing.extend(missing_cards::<$ty>()); total += <$ty as TraitOption>::ALL.len(); )+
        (missing, total)
    }};
}

#[test]
fn every_trait_variant_has_an_option_card() {
    let (missing, total) = missing_for!(
        CognitiveIntelligenceStyle, CognitivePace, EpistemicStance, HumorStyle,
        EnneagramType, InstinctualVariant, HumanGender,
        CapitalizationStyle, CodeSwitchingStyle, DictionRegister, EmojiUsage, GrammarStyle,
        PunctuationHabit, SentenceLengthPreference,
        IdentityVariant, MbtiType, PrePostReasoningMode, PromptAssemblyStrategy,
        AttachmentStyle, ClassicTemperament, LocusOfControl, MoralFoundation,
        ConflictHandlingStyle, HumanChronotype, MentionAttitude, PostingCadence,
        BodyKind, BodyForm, Material, Scale, Temporality, Perception, Mobility, Anthropomorphism,
        RelationToHumans, Withdrawal, RhetoricalMode, RhetoricalDevice, StylisticSchool, SemioticMode,
        SentenceStructure, PostFormat, LexicalField, Weekday,
        TextSource, LlmProvider, RequestKind, HttpMethod, OutputKind, SchemaPreset, ImageEngineKind, SdFlavor,
        ImageSize, TriggerKind, ToolKind, SearchBackend,
    );
    assert!(total > 100, "expected the full trait catalogue, saw {total} variants");
    assert!(missing.is_empty(), "missing trait illustrations:\n{}", missing.join("\n"));
}

fn collect_svgs(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("illustrations dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_svgs(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "svg") {
            out.push(path);
        }
    }
}

fn all_svgs() -> Vec<(String, String)> {
    let mut files = Vec::new();
    collect_svgs(Path::new(ILLUSTRATIONS_DIR), &mut files);
    files.sort();
    assert!(!files.is_empty(), "no illustrations found");
    files
        .into_iter()
        .map(|path| {
            let rel = path.strip_prefix(ILLUSTRATIONS_DIR).unwrap().to_string_lossy().replace('\\', "/");
            (rel, fs::read_to_string(&path).expect("read svg"))
        })
        .collect()
}

fn sample_fills() -> Vec<(String, String)> {
    let raw = fs::read_to_string(SAMPLE_THEME).expect("sample theme");
    let json: serde_json::Value = serde_json::from_str(&raw).expect("theme json");
    let mut fills: Vec<(String, String)> = json["colors"]
        .as_object()
        .expect("colors")
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_string()))
        .collect();
    fills.extend(PORTRAIT_FILLS.iter().map(|(k, v)| (k.to_string(), v.to_string())));
    fills
}

fn fill(svg: &str, fills: &[(String, String)]) -> String {
    fills.iter().fold(svg.to_string(), |acc, (key, hex)| acc.replace(&format!("{{{{{key}}}}}"), hex))
}

#[test]
fn every_illustration_parses_after_filling_placeholders() {
    let fills = sample_fills();
    let mut failures = Vec::new();
    for (rel, svg) in all_svgs() {
        let filled = fill(&svg, &fills);
        if let Some(at) = filled.find("{{") {
            failures.push(format!("{rel}: unknown placeholder near `{}`", &filled[at..(at + 24).min(filled.len())]));
            continue;
        }
        if let Err(err) = resvg::usvg::Tree::from_str(&filled, &resvg::usvg::Options::default()) {
            failures.push(format!("{rel}: {err}"));
        }
    }
    assert!(failures.is_empty(), "unparseable illustrations:\n{}", failures.join("\n"));
}

/// Returns every `#rgb` / `#rrggbb` / `#rrggbbaa` literal (gradient refs like `url(#g-x)` are not hex).
fn hex_literals(svg: &str) -> Vec<String> {
    let bytes = svg.as_bytes();
    let mut found = Vec::new();
    for (i, _) in svg.match_indices('#') {
        let run = bytes[i + 1..].iter().take_while(|b| b.is_ascii_hexdigit()).count();
        let next = bytes.get(i + 1 + run).copied().unwrap_or(b'"');
        if matches!(run, 3 | 6 | 8) && !next.is_ascii_alphanumeric() && next != b'-' {
            found.push(svg[i + 1..i + 1 + run].to_ascii_lowercase());
        }
    }
    found
}

#[test]
fn illustrations_use_palette_placeholders_only() {
    let mut failures = Vec::new();
    for (rel, svg) in all_svgs() {
        for hex in hex_literals(&svg) {
            if !ALLOWED_HEX.contains(&hex.as_str()) {
                failures.push(format!("{rel}: raw color #{hex}"));
            }
        }
        for tag in FORBIDDEN_TAGS {
            if svg.contains(tag) {
                failures.push(format!("{rel}: forbidden {tag}>"));
            }
        }
    }
    assert!(failures.is_empty(), "style violations:\n{}", failures.join("\n"));
}

#[test]
fn illustrations_respect_canvas_and_size_budget() {
    let mut failures = Vec::new();
    for (rel, svg) in all_svgs() {
        let kind = rel.split('/').next().unwrap_or_default();
        let limit = if WIDE_KINDS.contains(&kind) { WIDE_LIMIT_BYTES } else { CARD_LIMIT_BYTES };
        if svg.len() >= limit {
            failures.push(format!("{rel}: {} bytes (limit {limit})", svg.len()));
        }
        let canvas = CANVASES.iter().find(|(dir, _)| *dir == kind || dir.is_empty()).map(|(_, vb)| *vb).unwrap();
        if !svg.contains(&format!("viewBox=\"{canvas}\"")) {
            failures.push(format!("{rel}: expected viewBox \"{canvas}\""));
        }
    }
    assert!(failures.is_empty(), "budget violations:\n{}", failures.join("\n"));
}
