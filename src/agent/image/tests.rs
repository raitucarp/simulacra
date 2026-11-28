use super::save::{decode_base64, extension_for, save_image, ImageData};
use super::*;
use crate::agent::mock_server::{MockReply, MockServer};
use crate::domain::engine::{ImageEngine, ImageEngineConfig, ImageTrigger, SdFlavor};
use serde_json::json;

const PNG_1X1_BASE64: &str =
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mP8z8BQDwAEhQGAhKmMIQAAAABJRU5ErkJggg==";

fn job(engine: ImageEngine, endpoint: &str) -> ImageJob {
    ImageJob {
        config: ImageEngineConfig { engine, size: "64x32".into(), trigger: ImageTrigger::Always, ..ImageEngineConfig::default() },
        secret: "img-secret".into(),
        endpoint: endpoint.into(),
        media_dir: std::env::temp_dir().join(format!("simulacra-image-test-{}", uuid::Uuid::new_v4().simple())),
    }
}

#[test]
fn triggers_and_prompts() {
    assert!(!should_fire(ImageTrigger::Never, true, 0));
    assert!(should_fire(ImageTrigger::Always, false, 99));
    assert!(should_fire(ImageTrigger::Chance { percent: 30 }, false, 29));
    assert!(!should_fire(ImageTrigger::Chance { percent: 30 }, false, 30));
    assert!(should_fire(ImageTrigger::WhenModelAsks, true, 99) && !should_fire(ImageTrigger::WhenModelAsks, false, 0));
    assert_eq!(compose_prompt(" watercolor ", "a fox"), "watercolor, a fox");
    assert_eq!(compose_prompt("", " a fox "), "a fox");
    assert_eq!(compose_prompt("style", ""), "style");
}

#[test]
fn image_data_decodes_and_saves_with_the_right_extension() {
    assert_eq!(ImageData::from_text(" https://cdn.test/a.png "), ImageData::Url("https://cdn.test/a.png".into()));
    let bytes = decode_base64(&format!("data:image/png;base64,{PNG_1X1_BASE64}")).expect("decodes");
    assert_eq!(extension_for(&bytes), "png");
    assert_eq!(extension_for(&[0xFF, 0xD8, 0xFF, 0xE0]), "jpg");
    assert_eq!(extension_for(b"RIFF\0\0\0\0WEBPVP8 "), "webp");
    assert!(decode_base64("@@not base64@@").is_err());
    let dir = job(ImageEngine::OpenAi, "").media_dir;
    let path = save_image(&dir, &bytes).expect("saved");
    assert_eq!(path.parent(), Some(dir.as_path()));
    assert_eq!(std::fs::read(&path).expect("read back"), bytes);
    assert!(save_image(&dir, &[]).is_err());
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn openai_images_send_model_size_and_key() {
    let server = MockServer::start(vec![MockReply::json(json!({ "data": [{ "b64_json": PNG_1X1_BASE64 }] }))]);
    let job = job(ImageEngine::OpenAi, &format!("{}/v1/images/generations", server.url));
    let path = generate(&job, "a fox").await.expect("image");
    assert!(path.exists());
    let request = &server.requests()[0];
    assert_eq!(request.header("authorization"), Some("Bearer img-secret"));
    assert_eq!(request.json(), json!({ "model": "gpt-image-1", "prompt": "a fox", "size": "64x32", "n": 1 }));
    let _ = std::fs::remove_dir_all(&job.media_dir);
}

#[tokio::test]
async fn google_imagen_and_gemini_image_models() {
    let imagen = MockServer::start(vec![MockReply::json(json!({ "predictions": [{ "bytesBase64Encoded": PNG_1X1_BASE64 }] }))]);
    let mut job_imagen = job(ImageEngine::Google, &imagen.url);
    job_imagen.config.model = "imagen-4.0-generate-001".into();
    assert!(generate(&job_imagen, "a heron").await.is_ok());
    let request = &imagen.requests()[0];
    assert_eq!(request.path, "/imagen-4.0-generate-001:predict");
    assert_eq!(request.header("x-goog-api-key"), Some("img-secret"));
    assert_eq!(request.json()["instances"][0]["prompt"], "a heron");

    let gemini = MockServer::start(vec![MockReply::json(json!({ "candidates": [{ "content": { "parts": [
        { "text": "here" }, { "inlineData": { "mimeType": "image/png", "data": PNG_1X1_BASE64 } } ] } }] }))]);
    let job_gemini = job(ImageEngine::Google, &gemini.url);
    assert!(generate(&job_gemini, "a heron").await.is_ok());
    assert_eq!(gemini.requests()[0].path, "/gemini-nano-banana-2.1:generateContent", "a blank model is the Gemini image default");
    let _ = std::fs::remove_dir_all(&job_imagen.media_dir);
    let _ = std::fs::remove_dir_all(&job_gemini.media_dir);
}

#[tokio::test]
async fn local_stable_diffusion_servers() {
    let a1111 = MockServer::start(vec![MockReply::json(json!({ "images": [PNG_1X1_BASE64] }))]);
    let engine = ImageEngine::LocalSd { flavor: SdFlavor::A1111, url: a1111.url.clone() };
    let job_a = job(engine, "");
    assert!(generate(&job_a, "moss").await.is_ok());
    let request = &a1111.requests()[0];
    assert_eq!(request.path, "/sdapi/v1/txt2img");
    assert_eq!((request.json()["width"].as_u64(), request.json()["height"].as_u64()), (Some(64), Some(32)));

    let png = decode_base64(PNG_1X1_BASE64).expect("png");
    let comfy = MockServer::start(vec![
        MockReply::json(json!({ "prompt_id": "abc" })),
        MockReply::json(json!({})),
        MockReply::json(json!({ "abc": { "outputs": { "7": { "images": [{ "filename": "o.png", "subfolder": "", "type": "output" }] } } } })),
        MockReply::raw(200, "image/png", png.clone()),
    ]);
    let job_c = job(ImageEngine::LocalSd { flavor: SdFlavor::ComfyUi, url: comfy.url.clone() }, "");
    let path = generate(&job_c, "moss").await.expect("comfy image");
    assert_eq!(std::fs::read(path).expect("read"), png);
    let paths: Vec<String> = comfy.requests().iter().map(|r| r.path.clone()).collect();
    assert_eq!(paths, vec!["/prompt", "/history/abc", "/history/abc", "/view?filename=o.png&subfolder=&type=output"]);
    let workflow = comfy.requests()[0].json();
    assert_eq!(workflow["prompt"]["2"]["inputs"]["text"], "moss");
    assert_eq!(workflow["prompt"]["4"]["inputs"]["width"], 64);
    let _ = std::fs::remove_dir_all(&job_a.media_dir);
    let _ = std::fs::remove_dir_all(&job_c.media_dir);
}

#[tokio::test]
async fn engine_errors_are_reported() {
    assert!(generate(&job(ImageEngine::None, ""), "x").await.is_err());
    let server = MockServer::start(vec![MockReply::json(json!({ "data": [] }))]);
    let error = generate(&job(ImageEngine::OpenAi, &server.url), "x").await.expect_err("no image");
    assert_eq!(error, "OpenAI returned no image");
}

#[tokio::test]
async fn a_negative_prompt_reaches_stable_diffusion_only_when_given() {
    let a1111 = MockServer::start(vec![MockReply::json(json!({ "images": [PNG_1X1_BASE64] })), MockReply::json(json!({ "images": [PNG_1X1_BASE64] }))]);
    let job_a = job(ImageEngine::LocalSd { flavor: SdFlavor::A1111, url: a1111.url.clone() }, "");
    assert!(generate_with(&job_a, "moss", Some("text, watermark")).await.is_ok());
    assert!(generate(&job_a, "moss").await.is_ok());
    let requests = a1111.requests();
    assert_eq!(requests[0].json()["negative_prompt"], "text, watermark");
    assert!(requests[1].json().get("negative_prompt").is_none());
    let workflow = comfy::comfy_workflow("", "moss", Some("blur"), 64, 64);
    assert_eq!(workflow["3"]["inputs"]["text"], "blur");
    assert_eq!(comfy::comfy_workflow("", "moss", None, 64, 64)["3"]["inputs"]["text"], "blurry, low quality, watermark, text");
    let _ = std::fs::remove_dir_all(&job_a.media_dir);
}

#[tokio::test]
async fn a_google_reply_without_an_image_says_why() {
    let text_only = json!({ "candidates": [{ "content": { "parts": [{ "text": "I cannot draw that." }] }, "finishReason": "STOP" }] });
    let blocked = json!({ "candidates": [{ "finishReason": "IMAGE_SAFETY" }] });
    let server = MockServer::start(vec![MockReply::json(text_only), MockReply::json(blocked)]);
    let job = job(ImageEngine::Google, &server.url);
    let error = generate(&job, "x").await.expect_err("no image");
    assert_eq!(error, "Google returned no image: the model answered with text only: I cannot draw that.");
    let error = generate(&job, "x").await.expect_err("no image");
    assert_eq!(error, "Google returned no image: the reply was withheld (IMAGE_SAFETY)");
    assert_eq!(providers::no_image_error(&json!({})), "Google returned no image");
}

#[test]
fn image_job_debug_never_prints_the_secret() {
    let mut job = job(ImageEngine::Google, "https://img.test/v1?key=img-secret");
    let shown = format!("{job:?}");
    assert!(!shown.contains("img-secret"), "{shown}");
    assert!(shown.contains("has_secret: true"), "{shown}");
    job.secret = " ".into();
    assert!(format!("{job:#?}").contains("has_secret: false"));
}
