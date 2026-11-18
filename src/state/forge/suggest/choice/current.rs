//! The option keys a choice parameter currently has on the draft.

use super::options::{AVATAR_FORM, AVATAR_PORTRAIT, OFF, ON};
use super::super::engine_param::EngineParam;
use super::super::param::Param;
use crate::domain::engine::{
    EngineConfig, ImageEngine, ImageSize, OutputKind, RequestKind, RequestMode, TextModel, TextSource, ToolGrant, TriggerKind,
};
use crate::domain::portrait::AvatarMode;
use crate::domain::{Profile, TraitOption};
use crate::state::forge::engine::mutate::granted;

const AVATAR_EMBLEM: &str = "emblem";
const AVATAR_PHOTO: &str = "photo";

fn avatar_key(avatar: Option<&AvatarMode>) -> Option<&'static str> {
    avatar.map(|mode| match mode {
        AvatarMode::Pcg(_) => AVATAR_PORTRAIT,
        AvatarMode::Form(_) => AVATAR_FORM,
        AvatarMode::Emblem(_) => AVATAR_EMBLEM,
        AvatarMode::Photo { .. } => AVATAR_PHOTO,
    })
}

fn engine_keys(param: EngineParam, engine: &EngineConfig) -> Vec<&'static str> {
    use EngineParam as E;
    let one = |key: Option<&'static str>| key.into_iter().collect::<Vec<_>>();
    match param {
        E::Source => one(Some(TextSource::of(&engine.text).key())),
        E::Provider => one(match &engine.text {
            TextModel::Own(own) => Some(own.provider.key()),
            TextModel::UseGlobal => None,
        }),
        E::Request => one(Some(RequestKind::of(&engine.request).key())),
        E::Method => one(match &engine.request {
            RequestMode::Custom(custom) => Some(custom.method.key()),
            RequestMode::Native => None,
        }),
        E::Output => one(Some(OutputKind::of(&engine.output).key())),
        E::SchemaPreset => one(engine.structured().map(|structured| structured.preset.key())),
        E::Image => one(Some(engine.image.engine.kind().key())),
        E::ImageSize => one(ImageSize::of(&engine.image).map(ImageSize::key)),
        E::SdFlavor => one(match &engine.image.engine {
            ImageEngine::LocalSd { flavor, .. } => Some(flavor.key()),
            _ => None,
        }),
        E::Trigger => one(Some(TriggerKind::of(engine.image.trigger).key())),
        E::Tools => granted(engine).iter().map(TraitOption::key).collect(),
        E::SearchBackend => one(engine.tools.iter().find_map(|tool| match tool {
            ToolGrant::WebSearch(search) => Some(search.backend.key()),
            _ => None,
        })),
        _ => Vec::new(),
    }
}

/// The current option keys of `param` on `profile` (several for a multi-select, none when
/// the parameter is not a choice or does not apply right now).
pub fn current_keys(param: Param, profile: &Profile) -> Vec<&'static str> {
    let cfg = &profile.config;
    match param {
        Param::Trait(field) => vec![field.read(cfg).key()],
        Param::Set(field) => field.read(cfg).keys(),
        Param::Toggle(field) => vec![if *field.get(profile) { ON } else { OFF }],
        Param::Avatar(field) => avatar_key(field.get(profile).as_ref()).into_iter().collect(),
        Param::Engine(engine) => engine_keys(engine, &cfg.engine),
        _ => Vec::new(),
    }
}
