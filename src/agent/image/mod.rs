//! Image generation for agent posts: OpenAI, Google, local Stable Diffusion and custom
//! requests. Images land in the vault's media folder; the post stores the file path.
//!
//! - [`providers`]: OpenAI Images, Google Imagen / Gemini image models.
//! - [`local_sd`] / [`comfy`]: AUTOMATIC1111 and ComfyUI.
//! - [`save`]: base64 / URL / bytes → a file in the media folder.

pub mod comfy;
pub mod local_sd;
pub mod providers;
pub mod save;

#[cfg(test)]
mod tests;

use crate::agent::custom_request;
use crate::domain::engine::{value_text, ImageEngine, ImageEngineConfig, ImageTrigger};
use save::ImageData;
use serde_json::Value;
use std::path::PathBuf;
use std::time::Duration;

/// Timeout of one image request (local servers can be slow).
const IMAGE_REQUEST_TIMEOUT: Duration = Duration::from_secs(180);

/// One image engine, resolved with its credential and destination folder.
#[derive(Debug, Clone)]
pub struct ImageJob {
    pub config: ImageEngineConfig,
    pub secret: String,
    /// The credential's endpoint (replaces the provider URL when non-blank).
    pub endpoint: String,
    pub media_dir: PathBuf,
}

impl ImageJob {
    /// The configured model, or `default` when blank.
    pub fn model_or<'a>(&'a self, default: &'a str) -> &'a str {
        let model = self.config.model.trim();
        if model.is_empty() { default } else { model }
    }

    /// The credential endpoint, or `default` when blank.
    pub fn endpoint_or<'a>(&'a self, default: &'a str) -> &'a str {
        let endpoint = self.endpoint.trim();
        if endpoint.is_empty() { default } else { endpoint }
    }
}

/// Whether `trigger` fires for a post. `model_prompt` says whether the model supplied an
/// image prompt; `roll` is a uniform draw in `0..100`.
pub fn should_fire(trigger: ImageTrigger, model_prompt: bool, roll: u8) -> bool {
    match trigger {
        ImageTrigger::Never => false,
        ImageTrigger::Always => true,
        ImageTrigger::Chance { percent } => roll < percent,
        ImageTrigger::WhenModelAsks => model_prompt,
    }
}

/// The house style prefix joined to the prompt.
pub fn compose_prompt(style_prefix: &str, prompt: &str) -> String {
    match (style_prefix.trim(), prompt.trim()) {
        ("", prompt) => prompt.to_string(),
        (style, "") => style.to_string(),
        (style, prompt) => format!("{style}, {prompt}"),
    }
}

/// Renders `prompt` with the job's engine and saves the image; the saved file path.
pub async fn generate(job: &ImageJob, prompt: &str) -> Result<PathBuf, String> {
    let http = reqwest::Client::builder().timeout(IMAGE_REQUEST_TIMEOUT).build().map_err(|e| e.to_string())?;
    let prompt = compose_prompt(&job.config.style_prefix, prompt);
    let data = match &job.config.engine {
        ImageEngine::None => return Err("No image engine is configured".to_string()),
        ImageEngine::OpenAi => providers::openai(&http, job, &prompt).await?,
        ImageEngine::Google => providers::google(&http, job, &prompt).await?,
        ImageEngine::LocalSd { flavor, url } => match flavor {
            crate::domain::engine::SdFlavor::A1111 => local_sd::a1111(&http, url, job, &prompt).await?,
            crate::domain::engine::SdFlavor::ComfyUi => local_sd::comfy(&http, url, job, &prompt).await?,
        },
        ImageEngine::Custom(request) => {
            let vars = [
                ("prompt", Value::String(prompt.clone())),
                ("size", Value::String(job.config.size.clone())),
                ("model", Value::String(job.config.model.clone())),
            ];
            let reply = custom_request::send(&http, request, &job.secret, &job.config.model, &vars).await?;
            let text = value_text(&reply, &request.reply_path).ok_or("The custom image reply has no image at its reply path")?;
            ImageData::from_text(&text)
        }
    };
    let bytes = save::resolve(&http, data).await?;
    save::save_image(&job.media_dir, &bytes)
}
