//! Pieces of the Tensions view: one card per pair with both texts side by side (stacked on
//! narrow pages), the AI's verdict and one-sentence reason (or the lexical signals while
//! unchecked), and the actions: open each side, reply to reconcile, mark resolved, dismiss.

use super::super::labels::{tr, verdict_colour, verdict_label};
use super::{RequestAction, TensionRow, ThinkingRequest};
use crate::domain::post::readable_line;
use crate::domain::thinking::tensions::{SideKind, TensionSide};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const SIDE_ID: &str = "tension-side";
const ACTION_ID: &str = "tension-action";
/// Lines of a side's text.
const SIDE_LINES: usize = 6;
/// Buttons per card (element ids).
const ACTIONS_PER_CARD: usize = 3;

fn send(on_request: &RequestAction, request: ThinkingRequest) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_request = on_request.clone();
    move |_, window, cx| on_request(request.clone(), window, cx)
}

/// One side: who wrote it (or which document), the text, and a click that opens it.
fn side(slot: u64, side: &TensionSide, on_request: &RequestAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (icon, request) = match side.kind {
        SideKind::Post => (LucideIcon::MessageSquare, ThinkingRequest::OpenPost(crate::domain::PostId::from(side.id.as_str()))),
        SideKind::Passage => (LucideIcon::BookOpen, ThinkingRequest::OpenDocument(side.id.clone())),
    };
    let label = div().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(Icon::new(icon).size(theme.icon_size_xs)).child(div().truncate().child(side.label.clone()));
    div()
        .id(ElementId::NamedInteger(SIDE_ID.into(), slot))
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| s.border_color(theme.accent_brand_border))
        .on_click(send(on_request, request))
        .child(label)
        .child(div().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).line_clamp(SIDE_LINES).text_ellipsis().child(shown_text(side)))
}

/// A side's text as a reader would say it: a post without links to other posts or to-do
/// markup, a passage without the `#` marks of its headings.
fn shown_text(side: &TensionSide) -> String {
    match side.kind {
        SideKind::Post => readable_line(&side.text),
        SideKind::Passage => side.text.split_whitespace().filter(|word| !word.chars().all(|c| c == '#')).collect::<Vec<_>>().join(" "),
    }
}

/// The verdict line: the AI's verdict and reason, or the signals that made it a candidate.
fn verdict(row: &TensionRow, theme: &ThemeTokens) -> Div {
    let (badge, text) = match row.verdict {
        Some(v) => (verdict_label(v), row.reason.clone()),
        None => (tr("thinking.tensions.unchecked"), row.signals.join(" · ")),
    };
    let tag = div().flex_none().px(theme.spacing_sm).py(theme.spacing_3xs).rounded(theme.radius_full).bg(verdict_colour(row.verdict, theme)).text_size(theme.font_size_2xs).text_color(theme.on_accent).child(badge);
    div().flex().items_start().gap(theme.inline_gap).child(tag).child(div().flex_1().min_w_0().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_secondary).child(text))
}

/// The card of pair `index`; `stacked` puts the two texts one above the other.
pub fn tension_card(index: usize, row: &TensionRow, stacked: bool, on_request: &RequestAction, theme: &ThemeTokens) -> Div {
    let sides = div().w_full().flex().gap(theme.control_gap);
    let sides = if stacked { sides.flex_col() } else { sides.items_start() };
    let slot = (index * 2) as u64;
    let action = |offset: usize, icon: LucideIcon, key: &str, request: ThinkingRequest| {
        Button::new(ElementId::NamedInteger(ACTION_ID.into(), (index * ACTIONS_PER_CARD + offset) as u64)).ghost().xsmall().icon(Icon::new(icon)).label(tr(key)).on_click(send(on_request, request))
    };
    let actions = div()
        .flex()
        .flex_wrap()
        .gap(theme.control_gap)
        .child(action(0, LucideIcon::MessageSquareReply, "thinking.tensions.reconcile", ThinkingRequest::Reconcile(row.pair.clone())))
        .child(action(1, LucideIcon::CircleCheck, "thinking.tensions.resolved", ThinkingRequest::MarkResolved(row.pair.clone())))
        .child(action(2, LucideIcon::EyeOff, "thinking.tensions.dismiss", ThinkingRequest::Dismiss(row.pair.clone())));
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(sides.child(side(slot, &row.left, on_request, theme)).child(side(slot + 1, &row.right, on_request, theme)))
        .child(verdict(row, theme))
        .child(actions)
}
