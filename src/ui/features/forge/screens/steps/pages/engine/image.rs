//! Image engine: none, a cloud image model, a local Stable Diffusion server or a custom
//! request; then the model, size, house style, when it draws, its key and "Test image".

use super::super::super::super::common::card;
use super::headers::HeaderAccess;
use super::kit::{engine_id, json_line, option_line, CODE_ROWS, NOTE_ROWS};
use crate::domain::engine::{
    CustomRequest, HeaderPair, ImageEngine, ImageEngineConfig, ImageEngineKind, ImageSize, ImageTrigger, SdFlavor, TriggerKind,
    IMAGE_PLACEHOLDERS,
};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{body_error, image_credential_kind, mutate, JsonProblem};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::NumberSlider;
use crate::state::forge::suggest::{EngineParam, Param};
use gpui_kit::*;

const SECTION: EngineSection = EngineSection::Image;
const CHANCE_KEY: &str = "forge.ui.engine.chance";
const SIZE_HINT_KEY: &str = "forge.ui.engine.size_hint";
const MODEL_HINT_KEY: &str = "forge.ui.engine.image_model_hint";
/// Chance slider: (min, max, step) in percent.
const CHANCE_RANGE: (u16, u16, u16) = (5, 100, 5);

/// Mutable access to the custom image request.
fn custom_image(engine: &mut EngineConfig) -> Option<&mut CustomRequest> {
    match &mut engine.image.engine {
        ImageEngine::Custom(request) => Some(request),
        _ => None,
    }
}

fn image_headers(engine: &mut EngineConfig, _: usize) -> Option<&mut Vec<HeaderPair>> {
    custom_image(engine).map(|request| &mut request.headers)
}

impl ForgeView {
    pub(super) fn image_section(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let image = &self.engine().image;
        let kind = image.engine.kind();
        let grid = self.engine_cards((EngineParam::Image, "image"), (Some(kind), &ImageEngineKind::ALL), handle, mutate::set_image_kind);
        let head = card(theme).child(self.engine_field(handle, "engine_image", grid)).children(option_line(kind, theme));
        let page = div().flex().flex_col().gap(theme.section_gap).child(head);
        if kind == ImageEngineKind::None {
            return page;
        }
        page.children(self.server_card(&image.engine, handle)).child(self.look_card(image, handle)).child(self.when_card(image, handle)).child(self.image_probe_card(handle))
    }

    /// The local server (flavour and URL) or the custom request, when the engine has one.
    fn server_card(&self, engine: &ImageEngine, handle: &ForgeHandle) -> Option<Div> {
        let theme = &self.theme;
        match engine {
            ImageEngine::LocalSd { flavor, url } => {
                let flavors = self.engine_cards((EngineParam::SdFlavor, "sd-flavor"), (Some(*flavor), &SdFlavor::ALL), handle, mutate::set_sd_flavor);
                let url = self.engine_input_for((EngineParam::SdUrl, "sd-url".to_string()), url, (SECTION, handle), |e, text| {
                    if let ImageEngine::LocalSd { url, .. } = &mut e.image.engine {
                        *url = text;
                    }
                });
                Some(card(theme).child(self.engine_field(handle, "engine_sd_flavor", flavors)).children(option_line(*flavor, theme)).child(self.engine_field(handle, "engine_sd_url", url)))
            }
            ImageEngine::Custom(request) => Some(self.custom_image_card(request, handle)),
            _ => None,
        }
    }

    fn custom_image_card(&self, request: &CustomRequest, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let url = self.engine_input_for((EngineParam::Url, "image-url".to_string()), &request.url, (SECTION, handle), |e, text| {
            if let Some(request) = custom_image(e) {
                request.url = text;
            }
        });
        let body = self.engine_input_for((EngineParam::Body, "image-body".to_string()), &request.body_template, (SECTION, handle), |e, text| {
            if let Some(request) = custom_image(e) {
                request.body_template = text;
            }
        });
        let chips = self.token_chips("image-body", &IMAGE_PLACEHOLDERS, &request.body_template, (SECTION, handle), |e| custom_image(e).map(|r| &mut r.body_template));
        let reply = self.engine_input_for((EngineParam::ImageReplyPath, "image-reply".to_string()), &request.reply_path, (SECTION, handle), |e, text| {
            if let Some(request) = custom_image(e) {
                request.reply_path = text;
            }
        });
        let problem = body_error(&request.body_template, &IMAGE_PLACEHOLDERS).map(JsonProblem::Syntax);
        let body = div().flex().flex_col().gap(theme.spacing_xs).child(body.multiline(CODE_ROWS.0, CODE_ROWS.1)).child(json_line(problem, theme)).child(chips);
        card(theme)
            .gap(theme.section_gap)
            .child(self.engine_field(handle, "engine_url", url))
            .child(self.engine_field(handle, "engine_headers", self.headers_editor("image", &request.headers, (image_headers as HeaderAccess, 0), (SECTION, handle))))
            .child(self.engine_field(handle, "engine_body", body))
            .child(self.engine_field(handle, "engine_image_reply_path", reply))
    }

    /// Model, size and house style.
    fn look_card(&self, image: &ImageEngineConfig, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let model = self.engine_input_for((EngineParam::ImageModel, "image-model".to_string()), &image.model, (SECTION, handle), |e, text| e.image.model = text);
        let sizes = self.engine_cards((EngineParam::ImageSize, "image-size"), (ImageSize::of(image), &ImageSize::ALL), handle, |e, size: ImageSize| e.image.size = size.dimensions().to_string());
        let size = self.engine_input("image-size-text", &image.size, (SECTION, handle), |e, text| e.image.size = text);
        let style = self.engine_input_for((EngineParam::StylePrefix, "style-prefix".to_string()), &image.style_prefix, (SECTION, handle), |e, text| e.image.style_prefix = text);
        let sizes = div().flex().flex_col().gap(theme.spacing_xs).child(sizes.compact().columns(ImageSize::ALL.len()).card_width(theme.forge_compact_card_width)).child(size.placeholder(tr(SIZE_HINT_KEY)));
        card(theme)
            .gap(theme.section_gap)
            .child(self.engine_field(handle, "engine_image_model", model.placeholder(tr(MODEL_HINT_KEY))))
            .child(self.engine_field(handle, "engine_image_size", sizes))
            .child(self.engine_field(handle, "engine_style_prefix", style.multiline(NOTE_ROWS.0, NOTE_ROWS.1)))
    }

    /// When it draws (and how often by chance), and which key it uses.
    fn when_card(&self, image: &ImageEngineConfig, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let trigger = TriggerKind::of(image.trigger);
        let grid = self.engine_cards((EngineParam::Trigger, "trigger"), (Some(trigger), &TriggerKind::ALL), handle, mutate::set_trigger);
        let chance = match image.trigger {
            ImageTrigger::Chance { percent } => {
                let set = handle.engine(|e: &mut EngineConfig, value: u16| e.image.trigger = ImageTrigger::Chance { percent: value as u8 });
                Some(NumberSlider::new(engine_id("chance"), tr(CHANCE_KEY), u16::from(percent), CHANCE_RANGE, theme, move |v, w, cx| set(v, w, cx)).format(|v| format!("{v}%")).hint(self.slider_hint(Param::Engine(EngineParam::ImageChance), &tr(CHANCE_KEY), handle)))
            }
            _ => None,
        };
        let key = self.credential_picker("image", (image.credential.as_deref(), image_credential_kind(&image.engine)), handle, |e, id| e.image.credential = id);
        card(theme)
            .gap(theme.section_gap)
            .child(self.engine_field(handle, "engine_trigger", grid.compact()))
            .children(option_line(trigger, theme))
            .children(chance)
            .child(self.engine_field(handle, "engine_credential", key))
    }
}
