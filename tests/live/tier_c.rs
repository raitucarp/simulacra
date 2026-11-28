//! Tier C, media (opt-in: `SIMULACRA_LIVE_TIERS=a,b,c,d`): an image saved with real
//! PNG/JPEG bytes, an engine post illustrated on every post, and an optional transcription
//! of the audio file named by `SIMULACRA_LIVE_AUDIO`.

use super::fixtures::plan;
use super::harness::{block_on, ensure, pass, quote, run, skip, Key, Model, Tier, AUDIO_VAR};
use super::report::media_dir;
use simulacra::agent::image::{self, ImageJob};
use simulacra::agent::llm::transcribe::AudioClip;
use simulacra::agent::runtime::run_engine;
use simulacra::domain::engine::{EngineConfig, ImageEngine, ImageEngineConfig, ImageTrigger};
use std::path::Path;

const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G'];
const JPEG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];
const IMAGE_SIZE: &str = "1024x1024";

/// The image engine on the live image model, firing on every post.
fn image_config(model: &str) -> ImageEngineConfig {
    ImageEngineConfig { engine: ImageEngine::Google, model: model.into(), size: IMAGE_SIZE.into(), trigger: ImageTrigger::Always, ..ImageEngineConfig::default() }
}

/// `path` holds a PNG or JPEG; a note with its kind, size and place.
fn check_image(path: &Path) -> Result<String, String> {
    let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    let kind = if bytes.starts_with(PNG_MAGIC) {
        "PNG"
    } else if bytes.starts_with(JPEG_MAGIC) {
        "JPEG"
    } else {
        return Err(format!("{} bytes that are neither PNG nor JPEG (starts {:02X?})", bytes.len(), &bytes[..bytes.len().min(4)]));
    };
    Ok(format!("{kind}, {} KiB at {}", bytes.len() / 1024, path.display()))
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn c01_image_generation_saves_a_real_image() {
    run(Tier::C, "image generation (Gemini image model)", Model::Image, Key::Needed, |live| {
        let job = ImageJob { config: image_config(&live.image_model), secret: live.key().to_string(), endpoint: String::new(), media_dir: media_dir() };
        let path = block_on(image::generate(&job, "A small watercolor fox reading a book under a lamp"))?;
        pass(check_image(&path)?)
    });
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn c02_engine_post_illustrated_every_time() {
    run(Tier::C, "engine post with image trigger Always", Model::Image, Key::Needed, |live| {
        let config = EngineConfig { image: image_config(&live.image_model), ..EngineConfig::default() };
        let result = block_on(run_engine(&plan(live, &config), "You are a nature diarist.", "Post one sentence about a heron at dawn.", None, 0))?;
        if let Some(error) = &result.image_error {
            return Err(format!("the post was kept but the image failed: {error}"));
        }
        let path = result.image_path.as_deref().ok_or("no image path on the post")?;
        pass(format!("{}; post {}", check_image(Path::new(path))?, quote(&result.draft.content)))
    });
}

/// The MIME type of an audio file, by extension.
fn audio_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).map(str::to_lowercase).as_deref() {
        Some("mp3") => "audio/mpeg",
        Some("m4a" | "mp4") => "audio/mp4",
        Some("ogg" | "oga") => "audio/ogg",
        Some("flac") => "audio/flac",
        Some("webm") => "audio/webm",
        Some("aac") => "audio/aac",
        _ => "audio/wav",
    }
}

#[test]
#[ignore = "live: calls the real Gemini API (cargo live)"]
fn c03_transcription_of_an_audio_file() {
    run(Tier::C, "transcription (SIMULACRA_LIVE_AUDIO)", Model::Text, Key::Needed, |live| {
        let Some(path) = std::env::var_os(AUDIO_VAR).map(std::path::PathBuf::from) else {
            return skip(format!("skipped: {AUDIO_VAR} not set"));
        };
        let bytes = std::fs::read(&path).map_err(|e| format!("cannot read {AUDIO_VAR}: {e}"))?;
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("audio").to_string();
        let clip = AudioClip { mime: audio_mime(&path).into(), bytes, file_name };
        let text = block_on(live.client().transcribe(&clip))?;
        ensure(!text.trim().is_empty(), || "empty transcript".to_string())?;
        pass(format!("{} words: {}", text.split_whitespace().count(), quote(&text)))
    });
}
