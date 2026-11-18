//! Request: the provider's native API, or a hand-built HTTP request (method, URL, headers, a
//! JSON body template with placeholder chips, where the reply and tool calls are read), and
//! "Test call" for either.

use super::super::super::super::common::{card, heading, muted};
use super::headers::HeaderAccess;
use super::kit::{json_line, option_line, CODE_ROWS};
use crate::domain::engine::{CustomRequest, HeaderPair, HttpMethod, RequestKind, RequestMode, REQUEST_PLACEHOLDERS};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{body_error, mutate, JsonProblem};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::state::forge::suggest::EngineParam;
use gpui_kit::*;

const BUILDER_KEY: &str = "forge.ui.engine.builder_title";
const URL_HINT_KEY: &str = "forge.ui.engine.url_hint";
const NO_BODY_KEY: &str = "forge.ui.engine.no_body";
const SECTION: EngineSection = EngineSection::Request;

/// Mutable access to the custom request, when the agent has one.
pub(super) fn custom(engine: &mut EngineConfig) -> Option<&mut CustomRequest> {
    match &mut engine.request {
        RequestMode::Custom(request) => Some(request),
        RequestMode::Native => None,
    }
}

fn request_headers(engine: &mut EngineConfig, _: usize) -> Option<&mut Vec<HeaderPair>> {
    custom(engine).map(|request| &mut request.headers)
}

impl ForgeView {
    pub(super) fn request_section(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let kind = RequestKind::of(&self.engine().request);
        let grid = self.engine_cards((EngineParam::Request, "request"), (Some(kind), &RequestKind::ALL), handle, mutate::set_request_kind);
        let head = card(theme).child(self.engine_field(handle, "engine_request", grid)).children(option_line(kind, theme));
        let builder = match &self.engine().request {
            RequestMode::Custom(request) => Some(self.request_builder(request, handle)),
            RequestMode::Native => None,
        };
        div().flex().flex_col().gap(theme.section_gap).child(head).children(builder).child(self.call_probe_card(handle))
    }

    fn request_builder(&self, request: &CustomRequest, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let methods = self.engine_cards((EngineParam::Method, "method"), (Some(request.method), &HttpMethod::ALL), handle, |e, method| {
            if let Some(request) = custom(e) {
                request.method = method;
            }
        });
        let url = self.engine_input_for((EngineParam::Url, "url".to_string()), &request.url, (SECTION, handle), |e, text| {
            if let Some(request) = custom(e) {
                request.url = text;
            }
        });
        let headers = self.headers_editor("request", &request.headers, (request_headers as HeaderAccess, 0), (SECTION, handle));
        let key = self.credential_picker("request", (request.credential.as_deref(), None), handle, |e, id| {
            if let Some(request) = custom(e) {
                request.credential = id;
            }
        });
        card(theme)
            .gap(theme.section_gap)
            .child(heading(tr(BUILDER_KEY), theme))
            .child(self.engine_field(handle, "engine_method", methods.compact()))
            .child(self.engine_field(handle, "engine_url", url.placeholder(tr(URL_HINT_KEY))))
            .child(self.engine_field(handle, "engine_headers", headers))
            .child(self.engine_field(handle, "engine_body", self.body_editor(request, handle)))
            .child(self.reply_paths(request, handle))
            .child(self.engine_field(handle, "engine_credential", key))
    }

    /// The body template, its placeholder chips and whether it is JSON once filled in.
    fn body_editor(&self, request: &CustomRequest, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        if !request.method.has_body() {
            return muted(tr(NO_BODY_KEY), theme);
        }
        let body = self.engine_input_for((EngineParam::Body, "body".to_string()), &request.body_template, (SECTION, handle), |e, text| {
            if let Some(request) = custom(e) {
                request.body_template = text;
            }
        });
        let chips = self.token_chips("body", &REQUEST_PLACEHOLDERS, &request.body_template, (SECTION, handle), |e| custom(e).map(|r| &mut r.body_template));
        let problem = body_error(&request.body_template, &REQUEST_PLACEHOLDERS).map(JsonProblem::Syntax);
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(body.multiline(CODE_ROWS.0, CODE_ROWS.1))
            .child(json_line(problem, theme))
            .child(chips)
    }

    fn reply_paths(&self, request: &CustomRequest, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let reply = self.engine_input_for((EngineParam::ReplyPath, "reply-path".to_string()), &request.reply_path, (SECTION, handle), |e, text| {
            if let Some(request) = custom(e) {
                request.reply_path = text;
            }
        });
        let tools = self.engine_input_for((EngineParam::ToolCallPath, "tool-call-path".to_string()), &request.tool_call_path, (SECTION, handle), |e, text| {
            if let Some(request) = custom(e) {
                request.tool_call_path = text;
            }
        });
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(self.engine_field(handle, "engine_reply_path", reply))
            .child(self.engine_field(handle, "engine_tool_call_path", tools))
    }
}
