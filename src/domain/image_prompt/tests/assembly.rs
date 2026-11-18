use super::super::*;
use crate::domain::engine::{ImageEngine, ImageEngineConfig, ImageEngineKind, SdFlavor};

fn context(target: ImageTarget, noun: &str) -> ImageContext {
    let subject = ImageSubject { noun: noun.into(), details: vec!["made of glass".into()], palette_hint: None };
    ImageContext { target, subject, theme_colours: vec!["violet".into(), "gold".into(), "near-black".into(), "teal".into()] }
}

fn engine(engine: ImageEngine, prefix: &str) -> PromptEngine {
    PromptEngine::of(&ImageEngineConfig { engine, style_prefix: prefix.into(), size: "1024x1024".into(), ..ImageEngineConfig::default() }, "")
}

#[test]
fn hosted_engines_get_the_hints_said_positively() {
    let choices = ImageChoices::for_target(ImageTarget::Avatar);
    let prompt = assemble_prompt(&choices, &context(ImageTarget::Avatar, "a crow"), &engine(ImageEngine::OpenAi, "house style"));
    assert!(prompt.text.starts_with("house style, clean flat vector illustration"), "{}", prompt.text);
    assert!(prompt.text.contains("a crow, made of glass"));
    assert!(prompt.text.contains("colour palette of violet, gold and near-black"), "three theme colours at most: {}", prompt.text);
    assert!(prompt.text.contains("no text or lettering") && prompt.text.contains("no watermark"));
    assert_eq!(prompt.negative, None);
    assert_eq!(prompt.size, "1024x1024");
}

#[test]
fn stable_diffusion_gets_a_negative_prompt() {
    let sd = ImageEngine::LocalSd { flavor: SdFlavor::A1111, url: "http://127.0.0.1:7860".into() };
    let choices = ImageChoices::for_target(ImageTarget::Post).with_palette(ImagePalette::Warm).toggle_avoid(ImageAvoid::Blur);
    let prompt = assemble_prompt(&choices, &context(ImageTarget::Post, "Freedom is understanding"), &engine(sd, ""));
    assert!(prompt.text.contains("visual metaphor for: Freedom is understanding"));
    assert!(prompt.text.contains("warm palette") && !prompt.text.contains("no text"));
    assert_eq!(prompt.negative.as_deref(), Some("text, letters, captions, watermark, signature, logo, blurry, out of focus, low quality"));
}

#[test]
fn the_preference_prefix_and_aspect_fill_in() {
    let config = ImageEngineConfig { engine: ImageEngine::Google, ..ImageEngineConfig::default() };
    let engine = PromptEngine::of(&config, " soft grain ").with_aspect("landscape");
    assert_eq!((engine.style_prefix.as_str(), engine.size.as_str()), ("soft grain", "1536x1024"));
    assert_eq!(engine.clone().with_aspect("unknown").size, "1536x1024");
    assert_eq!(size_of_aspect("small"), Some("512x512"));
    assert!(!engine.supports_negative() && is_configured(&ImageEngine::Google) && !is_configured(&ImageEngine::None));
}

#[test]
fn theme_match_falls_back_to_the_agent_palette_and_long_prompts_are_clipped() {
    let mut ctx = context(ImageTarget::Avatar, "an owl");
    ctx.theme_colours.clear();
    ctx.subject.palette_hint = Some("ember_glow".into());
    let picks = ImageChoices::for_target(ImageTarget::Avatar);
    let prompt = assemble_prompt(&picks, &ctx, &engine(ImageEngine::OpenAi, ""));
    assert!(prompt.text.contains("colours inspired by the ember glow palette"), "{}", prompt.text);
    ctx.subject.noun = "word ".repeat(400);
    let long = assemble_prompt(&picks, &ctx, &engine(ImageEngine::OpenAi, ""));
    assert!(long.text.chars().count() <= MAX_PROMPT_CHARS);
}

#[test]
fn an_agent_post_without_a_prompt_gets_a_better_default() {
    let agent = ImageSubject { noun: "a crow".into(), details: vec!["feeling restless".into()], palette_hint: Some("ember".into()) };
    let prompt = default_post_prompt("Memory is a river. It keeps moving.", &agent, ImageEngineKind::OpenAi);
    assert!(prompt.text.contains("visual metaphor for: Memory is a river"), "{}", prompt.text);
    assert!(prompt.text.contains("as imagined by a crow") && prompt.text.contains("feeling restless"));
    assert!(prompt.text.contains("colours inspired by the ember palette") && prompt.text.contains("no text"));
    assert!(!prompt.text.contains("It keeps moving"), "only the first sentence");
    let sd = default_post_prompt("Memory is a river.", &ImageSubject::default(), ImageEngineKind::LocalSd);
    assert!(sd.negative.is_some() && !sd.text.contains("imagined by"));
}
