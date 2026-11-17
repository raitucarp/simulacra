//! What the engine recorded on an agent post: the author's mood, the tools it called (failed
//! calls dimmed) and the sources it drew on, as chips under the body. Source chips open what
//! they name: the vault document, the post, the profile or the web page.

use crate::domain::engine::ToolKind;
use crate::domain::post::{PostExtras, SourceKind, SourceRef, ToolUse};
use crate::theme::ThemeTokens;
use crate::domain::TraitOption;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, App, Div, FontWeight, InteractiveElement, MouseButton, ParentElement, Rgba, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Opens what a source chip names.
pub type OpenSourceAction = Rc<dyn Fn(SourceRef, &mut Window, &mut App) + 'static>;

/// The icon of a tool, by its model-facing name (custom tools get a bolt).
pub fn tool_icon(name: &str) -> LucideIcon {
    match ToolKind::ALL.into_iter().find(|kind| kind.key() == name) {
        Some(ToolKind::VaultSearch) => LucideIcon::Library,
        Some(ToolKind::ReadNetwork) => LucideIcon::Users,
        Some(ToolKind::WebSearch) => LucideIcon::Search,
        Some(ToolKind::WebFetch) => LucideIcon::Globe,
        Some(ToolKind::CustomHttp) | None => LucideIcon::Zap,
    }
}

/// The display name of a tool: built-ins are translated, custom tools keep their name.
pub fn tool_name(name: &str) -> String {
    ToolKind::ALL.into_iter().find(|kind| kind.key() == name).map_or_else(|| name.to_string(), |kind| {
        let key = kind.label_key();
        t!(key.as_str()).to_string()
    })
}

/// The icon of a source kind.
pub fn source_icon(kind: SourceKind) -> LucideIcon {
    match kind {
        SourceKind::VaultDoc => LucideIcon::FileText,
        SourceKind::Post => LucideIcon::MessageSquare,
        SourceKind::Profile => LucideIcon::User,
        SourceKind::Web => LucideIcon::Link2,
    }
}

/// A small rounded chip: icon, then text.
fn chip(icon: LucideIcon, text: String, color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .bg(theme.hover_bg)
        .text_size(theme.font_size_xs)
        .text_color(color)
        .child(Icon::new(icon).size(theme.icon_size_xs))
        .child(text)
}

/// The author's mood while posting.
pub fn mood_chip(mood: &str, theme: &ThemeTokens) -> Div {
    chip(LucideIcon::Sparkles, mood.to_string(), theme.accent_agent, theme).font_weight(FontWeight::MEDIUM)
}

/// One tool call: its icon and name, with the arguments as a tooltip-like suffix.
pub fn tool_chip(tool: &ToolUse, theme: &ThemeTokens) -> Div {
    let icon = if tool.failed { LucideIcon::CircleAlert } else { tool_icon(&tool.name) };
    chip(icon, tool_name(&tool.name), theme.text_secondary, theme).when(tool.failed, |chip| chip.opacity(theme.post_failed_tool_opacity))
}

/// A source chip; clicking it runs `on_open` (without selecting the card).
pub fn source_chip(source: &SourceRef, theme: &ThemeTokens, on_open: Option<OpenSourceAction>) -> Div {
    let title = if source.title.trim().is_empty() { source.id.clone() } else { source.title.clone() };
    let base = chip(source_icon(source.kind), title, theme.accent_brand, theme);
    let Some(on_open) = on_open else { return base };
    let (source, hover) = (source.clone(), theme.border_strong);
    base.cursor_pointer().border(theme.border_width_hairline).border_color(theme.border_subtle).hover(move |s| s.border_color(hover)).on_mouse_down(
        MouseButton::Left,
        move |_, window, cx| {
            cx.stop_propagation();
            on_open(source.clone(), window, cx);
        },
    )
}

/// A labeled, wrapping row of chips.
fn chip_row(label: String, chips: Vec<Div>, theme: &ThemeTokens) -> Div {
    let caption = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(label);
    div().flex().flex_wrap().items_center().gap(theme.spacing_xs).child(caption).children(chips)
}

/// The mood, "Used" tools and "Sources" rows of a post, or `None` when nothing is recorded.
pub(crate) fn render_post_extras(extras: &PostExtras, theme: &ThemeTokens, on_open: Option<&OpenSourceAction>) -> Option<Div> {
    let has_rows = extras.mood.is_some() || !extras.tools.is_empty() || !extras.sources.is_empty();
    if !has_rows {
        return None;
    }
    let mood = extras.mood.as_deref().map(|mood| div().flex().child(mood_chip(mood, theme)));
    let tools = (!extras.tools.is_empty()).then(|| {
        let chips = extras.tools.iter().map(|tool| tool_chip(tool, theme)).collect();
        chip_row(t!("post.extras_tools").to_string(), chips, theme)
    });
    let sources = (!extras.sources.is_empty()).then(|| {
        let chips = extras.sources.iter().map(|source| source_chip(source, theme, on_open.cloned())).collect();
        chip_row(t!("post.extras_sources").to_string(), chips, theme)
    });
    Some(div().flex().flex_col().gap(theme.spacing_xs).children(mood).children(tools).children(sources))
}
