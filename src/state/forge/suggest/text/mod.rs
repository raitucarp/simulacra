//! Example values of the free-text parameters: translated prose examples, values derived from
//! the draft (handles and initials from the name, a tritype around the Enneagram core, pronouns
//! in the order the gender and body suggest) and literal technical values for the Engine.

mod derived;
mod endpoints;
mod templates;

use super::context::SuggestContext;
use super::engine_param::EngineParam;
use super::keys::{keyed_examples, literal_examples, tip_key};
use super::model::{Example, Joiner, Phrase, TextHints};
use super::param::Param;
use crate::state::forge::TextField;

/// Fields whose examples are lists and append with a comma.
const COMMA_FIELDS: [TextField; 5] =
    [TextField::SecondaryLanguages, TextField::CoreValues, TextField::Hobbies, TextField::SignaturePhrases, TextField::AvoidedWords];
/// Fields whose examples append on a new line.
const LINE_FIELDS: [TextField; 2] = [TextField::SystemPrompt, TextField::PromptTemplate];

fn joiner(param: Param) -> Joiner {
    match param {
        Param::Text(field) if COMMA_FIELDS.contains(&field) => Joiner::Comma,
        Param::Text(field) if LINE_FIELDS.contains(&field) => Joiner::Line,
        Param::Engine(EngineParam::Body | EngineParam::Schema | EngineParam::ParamsSchema) => Joiner::Line,
        _ => Joiner::Space,
    }
}

/// Examples of one text field of the profile.
fn field_examples(field: TextField, ctx: &SuggestContext) -> Vec<Example> {
    match field {
        TextField::Name => derived::names(ctx),
        TextField::Handle => derived::handles(ctx),
        TextField::AvatarInitial => derived::initials(&ctx.profile.name),
        TextField::Tritype => derived::tritypes(ctx.config().enneagram),
        TextField::Pronouns => derived::pronouns(ctx.config()),
        TextField::PromptTemplate => templates::prompt_templates(),
        other => keyed_examples(other.path()),
    }
}

/// Examples of one Engine text setting (`None` for settings that are not free text).
fn engine_examples(param: EngineParam, ctx: &SuggestContext) -> Option<Vec<Example>> {
    use EngineParam::*;
    let examples = match param {
        Model => literal_examples(&endpoints::models(&ctx.config().engine)),
        Endpoint => literal_examples(&endpoints::ENDPOINTS),
        Url => literal_examples(&endpoints::REQUEST_URLS),
        Headers => literal_examples(&endpoints::HEADERS),
        ReplyPath => literal_examples(&endpoints::REPLY_PATHS),
        ToolCallPath => literal_examples(&endpoints::TOOL_CALL_PATHS),
        ImageModel => literal_examples(&endpoints::image_models(&ctx.config().engine)),
        ImageReplyPath => literal_examples(&endpoints::IMAGE_REPLY_PATHS),
        SdUrl => literal_examples(&endpoints::SD_URLS),
        ToolName => literal_examples(&endpoints::TOOL_NAMES),
        UrlTemplate => literal_examples(&endpoints::URL_TEMPLATES),
        Body => templates::bodies(),
        Schema => templates::schemas(),
        ParamsSchema => templates::params_schemas(),
        StylePrefix | ToolDescription => keyed_examples(param.key()),
        _ => return None,
    };
    Some(examples)
}

/// The text suggestions of `param`, when it is free text.
pub(super) fn text_hints(param: Param, ctx: &SuggestContext) -> Option<TextHints> {
    let examples = match param {
        Param::Text(field) => field_examples(field, ctx),
        Param::Engine(engine) => engine_examples(engine, ctx)?,
        _ => return None,
    };
    Some(TextHints { examples, tip: Phrase::key(tip_key(param.path())), joiner: joiner(param) })
}
