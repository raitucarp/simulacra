//! The built-in ComfyUI text-to-image workflow (API format).

use serde_json::{json, Value};

/// Checkpoint used when the engine's model field is blank.
const DEFAULT_CHECKPOINT: &str = "sd_xl_base_1.0.safetensors";
const NEGATIVE_PROMPT: &str = "blurry, low quality, watermark, text";
const SAMPLER: &str = "euler";
const SCHEDULER: &str = "normal";
const STEPS: u32 = 25;
const CFG: f32 = 7.0;
const OUTPUT_PREFIX: &str = "simulacra";

/// A checkpoint → CLIP encode → KSampler → VAE decode → SaveImage graph.
pub fn comfy_workflow(checkpoint: &str, prompt: &str, width: u32, height: u32) -> Value {
    let checkpoint = if checkpoint.trim().is_empty() { DEFAULT_CHECKPOINT } else { checkpoint.trim() };
    let seed = uuid::Uuid::new_v4().as_u64_pair().0 >> 12;
    json!({
        "1": { "class_type": "CheckpointLoaderSimple", "inputs": { "ckpt_name": checkpoint } },
        "2": { "class_type": "CLIPTextEncode", "inputs": { "text": prompt, "clip": ["1", 1] } },
        "3": { "class_type": "CLIPTextEncode", "inputs": { "text": NEGATIVE_PROMPT, "clip": ["1", 1] } },
        "4": { "class_type": "EmptyLatentImage", "inputs": { "width": width, "height": height, "batch_size": 1 } },
        "5": { "class_type": "KSampler", "inputs": {
            "model": ["1", 0], "positive": ["2", 0], "negative": ["3", 0], "latent_image": ["4", 0],
            "seed": seed, "steps": STEPS, "cfg": CFG, "sampler_name": SAMPLER, "scheduler": SCHEDULER, "denoise": 1.0 } },
        "6": { "class_type": "VAEDecode", "inputs": { "samples": ["5", 0], "vae": ["1", 2] } },
        "7": { "class_type": "SaveImage", "inputs": { "images": ["6", 0], "filename_prefix": OUTPUT_PREFIX } }
    })
}
