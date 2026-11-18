//! The "find similar" sheet: the items closest in meaning to a post or document, each with
//! its score and best passage, and a button to keep exploring from that item.

use super::row::render_scored_row;
use crate::state::semantic::{HitTarget, SemanticHit};
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "similar-sheet-backdrop";
const CARD_ID: &str = "similar-sheet-card";
const LIST_ID: &str = "similar-sheet-list";
const CLOSE_ID: &str = "similar-sheet-close";
const AGAIN_ID_PREFIX: &str = "similar-again-";
const ESCAPE_KEY: &str = "escape";

/// What the sheet shows.
#[derive(Debug, Clone, PartialEq)]
pub struct SimilarView {
    /// Short excerpt of the item the results are similar to.
    pub source: String,
    /// Why these are word matches (none when found by meaning).
    pub note: Option<String>,
    pub hits: Vec<SemanticHit>,
}

/// A click on one hit.
pub type HitAction = Rc<dyn Fn(HitTarget, &mut Window, &mut App) + 'static>;

/// The sheet's callbacks.
#[derive(Clone)]
pub struct SheetActions {
    pub on_open: HitAction,
    pub on_again: HitAction,
    pub on_close: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
}

/// The icon of a hit's kind.
pub fn hit_icon(target: &HitTarget) -> LucideIcon {
    match target {
        HitTarget::Post(_) => LucideIcon::MessageSquare,
        HitTarget::Document(_) => LucideIcon::FileText,
    }
}

/// The overlay: scrim, card with a header, the note and the rows.
pub fn render_similar_sheet(view: &SimilarView, actions: SheetActions, focus: &FocusHandle, theme: &ThemeTokens) -> Stateful<Div> {
    let (close, escape, dismiss) = (actions.on_close.clone(), actions.on_close.clone(), actions.on_close.clone());
    let rows: Vec<AnyElement> = view.hits.iter().enumerate().map(|(index, hit)| render_hit(index, hit, &actions, theme)).collect();
    let art = || crate::ui::illustration::Illustration::spot(crate::ui::illustration::SpotKey::Connections).width(theme.art_notice_width);
    let empty_state = || crate::ui::components::art_state(crate::ui::components::ArtState::hint(art(), t!("semantic.similar.empty").to_string()), theme);
    let empty = rows.is_empty().then(|| div().p(theme.panel_padding).flex().justify_center().child(empty_state()));
    let note = view.note.clone().map(|note| div().px(theme.card_padding).pt(theme.spacing_xs).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(note));
    let list = div().id(LIST_ID).max_h(theme.similar_sheet_max_height).overflow_y_scroll().p(theme.spacing_xs).flex().flex_col().children(rows);
    let card = render_card(theme)
        .child(render_header(&view.source, move |window, cx| close(window, cx), theme))
        .children(note)
        .children(empty)
        .child(list);
    div()
        .id(BACKDROP_ID)
        .track_focus(focus)
        .on_key_down(move |event, window, cx| {
            if event.keystroke.key == ESCAPE_KEY {
                escape(window, cx);
            }
        })
        .absolute()
        .inset_0()
        .occlude()
        .bg(theme.overlay)
        .flex()
        .flex_col()
        .items_center()
        .pt(theme.palette_top_offset)
        .on_mouse_down(MouseButton::Left, move |_, window, cx| dismiss(window, cx))
        .child(card)
}

/// The elevated card (clicks inside never reach the scrim).
fn render_card(theme: &ThemeTokens) -> Stateful<Div> {
    div()
        .id(CARD_ID)
        .w(theme.similar_sheet_width)
        .flex()
        .flex_col()
        .overflow_hidden()
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .elevation_shadow(theme)
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
}

fn render_header(source: &str, on_close: impl Fn(&mut Window, &mut App) + 'static, theme: &ThemeTokens) -> Div {
    let titles = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("semantic.similar.title").to_string()))
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("semantic.similar.subtitle", source = source).to_string()));
    let close = Button::new(CLOSE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(t!("semantic.similar.close").to_string()).on_click(move |_, window, cx| on_close(window, cx));
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.card_padding)
        .py(theme.spacing_sm)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(Icon::new(LucideIcon::Compass).size(theme.icon_size_sm).text_color(theme.accent_brand))
        .child(titles)
        .child(close)
}

fn render_hit(index: usize, hit: &SemanticHit, actions: &SheetActions, theme: &ThemeTokens) -> AnyElement {
    let (open, again) = (actions.on_open.clone(), actions.on_again.clone());
    let (open_target, again_target) = (hit.target.clone(), hit.target.clone());
    let row = render_scored_row(index, hit_icon(&hit.target), &hit.title, &hit.detail, &hit.excerpt, hit.score, false, theme)
        .flex_1()
        .min_w_0()
        .on_click(move |_, window, cx| open(open_target.clone(), window, cx));
    let more = Button::new(SharedString::from(format!("{AGAIN_ID_PREFIX}{index}")))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Compass))
        .tooltip(t!("semantic.similar.again").to_string())
        .on_click(move |_, window, cx| again(again_target.clone(), window, cx));
    div().flex().items_center().gap(theme.spacing_xs).child(row).child(more).into_any_element()
}
