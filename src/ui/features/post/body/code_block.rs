//! The code block card: a header with the language label and a copy button, over the code
//! in the mono face, one row per line, syntax-colored from the theme's `code_*` tokens.

use crate::domain::post::highlight::{highlight_lines, CodeHighlight, CodeStyle};
use crate::theme::ThemeTokens;
use super::code_run::RunButton;
use super::run_panel::{render_run_panel, RunPanel};
use crate::ui::features::post::controls::copy_with_toast;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{
    div, relative, AnyElement, Div, HighlightStyle, InteractiveElement, IntoElement, MouseButton, ParentElement, Rgba, Styled,
    StyledText,
};
use rust_i18n::t;
use std::ops::Range;

/// Element id of a block's copy button (suffixed with the block's item index).
const COPY_BUTTON_ID: &str = "code-block-copy";
/// Element id of a block's Run button (P5).
const RUN_BUTTON_ID: &str = "code-block-run";
/// Text shown for an empty code line, so it keeps its height.
const EMPTY_CODE_LINE: &str = " ";
const LINE_BREAK: char = '\n';

/// The theme color of a highlighted run.
pub(super) fn style_color(style: CodeStyle, theme: &ThemeTokens) -> Rgba {
    match style {
        CodeStyle::Keyword => theme.code_keyword,
        CodeStyle::Type => theme.code_type,
        CodeStyle::String => theme.code_string,
        CodeStyle::Number => theme.code_number,
        CodeStyle::Comment => theme.code_comment,
    }
}

/// The label over a block: its language tag, or "plain text" without one.
pub(super) fn language_label(lang: Option<&str>) -> String {
    lang.map_or_else(|| t!("post.code_plain").to_string(), str::to_string)
}

/// One line's highlights as text styles.
fn line_styles(spans: &[CodeHighlight], theme: &ThemeTokens) -> Vec<(Range<usize>, HighlightStyle)> {
    spans
        .iter()
        .map(|(range, style)| (range.clone(), HighlightStyle { color: Some(style_color(*style, theme).into()), ..HighlightStyle::default() }))
        .collect()
}

/// A code block card without Run (file previews, L9).
pub(crate) fn code_block_card(index: usize, lang: Option<&str>, code: &str, theme: &ThemeTokens) -> AnyElement {
    code_block(index, lang, code, theme, BlockExtras::default())
}

/// What a block shows beyond its code: Run (P5) and its output panel (P6).
#[derive(Default)]
pub(super) struct BlockExtras {
    pub run: Option<RunButton>,
    pub panel: Option<RunPanel>,
}

/// A fenced code block card; `index` keeps its buttons' ids unique within the body; `extras` are
/// its Run button and output panel, when it has them.
pub(super) fn code_block(index: usize, lang: Option<&str>, code: &str, theme: &ThemeTokens, extras: BlockExtras) -> AnyElement {
    let lines = code.split(LINE_BREAK).zip(highlight_lines(lang, code)).map(|(line, spans)| {
        let row = if line.is_empty() { StyledText::new(EMPTY_CODE_LINE) } else { StyledText::new(line.to_string()).with_highlights(line_styles(&spans, theme)) };
        div().min_w_0().child(row)
    });
    let body = div().w_full().px(theme.spacing_sm).py(theme.spacing_xs).flex().flex_col().children(lines);
    div()
        .w_full()
        .my(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_code)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_sm)
        .text_color(theme.text_primary)
        .line_height(relative(theme.line_height_body))
        .flex()
        .flex_col()
        .child(header(index, lang, code, extras.run, theme))
        .child(body)
        .children(extras.panel.map(|panel| render_run_panel(index, panel, theme)))
        .into_any_element()
}

/// Language label on the left; Run (when the block may run) and copy on the right.
fn header(index: usize, lang: Option<&str>, code: &str, run: Option<RunButton>, theme: &ThemeTokens) -> Div {
    let code = code.to_string();
    let copy = Button::new((COPY_BUTTON_ID, index))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Copy))
        .tooltip(t!("post.code_copy").to_string())
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            copy_with_toast(code.clone(), t!("post.code_copied").to_string(), window, cx);
        });
    div()
        .w_full()
        .pl(theme.spacing_sm)
        .pr(theme.spacing_2xs)
        .py(theme.spacing_3xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .justify_between()
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(language_label(lang)))
        .child(div().flex().items_center().on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()).children(run.map(|run| run_control(index, run))).child(copy))
}

/// The Run button: hands the block's request to the shell (which asks first when it must).
fn run_control(index: usize, run: RunButton) -> Button {
    let RunButton { request, on_run } = run;
    Button::new((RUN_BUTTON_ID, index))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Play))
        .label(t!("code.run.button").to_string())
        .tooltip(t!("code.run.tooltip", runtime = request.runtime.label()).to_string())
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_run(request.clone(), window, cx);
        })
}
