//! Local Stable Diffusion servers: AUTOMATIC1111 (`/sdapi/v1/txt2img`) and ComfyUI
//! (`/prompt`, `/history/{id}`, `/view`).

use super::comfy::comfy_workflow;
use super::save::ImageData;
use super::ImageJob;
use crate::agent::llm::send_json;
use crate::agent::llm::wire::ErrorLabels;
use serde_json::{json, Value};
use std::time::Duration;

const A1111_TXT2IMG_PATH: &str = "/sdapi/v1/txt2img";
const A1111_STEPS: u32 = 25;
const A1111_NEGATIVE_KEY: &str = "negative_prompt";
const COMFY_PROMPT_PATH: &str = "/prompt";
const COMFY_HISTORY_PATH: &str = "/history/";
const COMFY_VIEW_PATH: &str = "/view";
/// ComfyUI polling: interval and attempts (about two minutes).
const COMFY_POLL_INTERVAL: Duration = Duration::from_secs(1);
const COMFY_MAX_POLLS: u32 = 120;
/// Default render size when the configured one is malformed.
const DEFAULT_SIDE: u32 = 1024;

const SD_ERRORS: ErrorLabels = ErrorLabels {
    network: "Stable Diffusion request error",
    api: "Stable Diffusion server error",
    parse: "Failed to parse Stable Diffusion response",
    empty: "Stable Diffusion returned no image",
};

/// The A1111 txt2img body (with the negative prompt when one is given).
pub fn a1111_body(job: &ImageJob, prompt: &str, negative: Option<&str>) -> Value {
    let (width, height) = job.config.dimensions().unwrap_or((DEFAULT_SIDE, DEFAULT_SIDE));
    let mut body = json!({ "prompt": prompt, "width": width, "height": height, "steps": A1111_STEPS });
    if let Some(negative) = negative.filter(|text| !text.trim().is_empty()) {
        body[A1111_NEGATIVE_KEY] = Value::String(negative.to_string());
    }
    body
}

/// Generates with AUTOMATIC1111 (`images[0]` is base64).
pub async fn a1111(http: &reqwest::Client, base: &str, job: &ImageJob, prompt: &str, negative: Option<&str>) -> Result<ImageData, String> {
    let url = format!("{}{A1111_TXT2IMG_PATH}", base.trim_end_matches('/'));
    let reply = send_json(http.post(url).json(&a1111_body(job, prompt, negative)), &SD_ERRORS).await?;
    reply["images"][0].as_str().map(|data| ImageData::Base64(data.to_string())).ok_or_else(|| SD_ERRORS.empty.to_string())
}

/// Queues the workflow on ComfyUI, waits for its first output image and downloads it.
pub async fn comfy(http: &reqwest::Client, base: &str, job: &ImageJob, prompt: &str, negative: Option<&str>) -> Result<ImageData, String> {
    let base = base.trim_end_matches('/');
    let (width, height) = job.config.dimensions().unwrap_or((DEFAULT_SIDE, DEFAULT_SIDE));
    let workflow = comfy_workflow(&job.config.model, prompt, negative, width, height);
    let queued = send_json(http.post(format!("{base}{COMFY_PROMPT_PATH}")).json(&json!({ "prompt": workflow })), &SD_ERRORS).await?;
    let id = queued["prompt_id"].as_str().ok_or_else(|| SD_ERRORS.empty.to_string())?.to_string();
    for _ in 0..COMFY_MAX_POLLS {
        let history = send_json(http.get(format!("{base}{COMFY_HISTORY_PATH}{id}")), &SD_ERRORS).await?;
        if let Some(image) = first_output_image(&history[&id]) {
            let query = [("filename", image.0), ("subfolder", image.1), ("type", image.2)];
            let resp = http.get(format!("{base}{COMFY_VIEW_PATH}")).query(&query).send().await;
            let resp = resp.map_err(|e| format!("{}: {e}", SD_ERRORS.network))?;
            let (bytes, _) = crate::agent::tools::http::read_capped(resp, super::save::MAX_IMAGE_BYTES).await?;
            return Ok(ImageData::Bytes(bytes));
        }
        tokio::time::sleep(COMFY_POLL_INTERVAL).await;
    }
    Err("ComfyUI did not finish the image in time".to_string())
}

/// `(filename, subfolder, type)` of the first image in a ComfyUI history entry.
pub fn first_output_image(entry: &Value) -> Option<(String, String, String)> {
    let outputs = entry["outputs"].as_object()?;
    outputs.values().find_map(|node| {
        let image = node["images"].get(0)?;
        let field = |key: &str| image[key].as_str().unwrap_or_default().to_string();
        Some((field("filename"), field("subfolder"), field("type")))
    })
}
