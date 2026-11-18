//! The right pane: the action's title and hint, its choices, then the answer (or the state
//! of the run) in a scrollable area, and the answer's buttons.

use super::blocks::render_blocks;
use super::choices::render_choices;
use super::footer::render_footer;
use super::hits::render_concept_hits;
use super::labels::{kind_hint, kind_icon, kind_title};
use super::model::{ActionStatus, SheetEvent};
use super::sheet::{SheetAction, SheetInputs};
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::illustration::{EmptyKey, Illustration, NoticeKey};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const SCROLL_ID: &str = "post-actions-answer";
const STOP_ID: &str = "post-actions-stop";

/// The pane right of the rail.
pub(super) fn render_answer_pane(inputs: &SheetInputs<'_>) -> Div {
    let (sheet, theme) = (inputs.sheet, inputs.theme);
    let pane = div().flex_1().min_w_0().h_full().flex().flex_col();
    if sheet.status == ActionStatus::Picking {
        let art = Illustration::empty(EmptyKey::ActionsIdle).width(theme.art_empty_width);
        return pane.items_center().justify_center().p(theme.panel_padding).child(art_state(ArtState::hint(art, t!("post_actions.pick").to_string()), theme));
    }
    let kind = sheet.request.kind;
    let heading = div()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().flex().items_center().gap(theme.inline_gap).text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(Icon::new(kind_icon(kind)).size(theme.icon_size_sm).text_color(theme.accent_brand)).child(kind_title(kind)))
        .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(kind_hint(kind)));
    let body = div()
        .id(SCROLL_ID)
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .px(theme.panel_padding)
        .pb(theme.panel_padding)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .children(sheet.hits.as_ref().map(|hits| render_concept_hits(hits, &inputs.on_event, theme)))
        .child(render_status(&sheet.status, &inputs.on_event, theme));
    pane.child(div().p(theme.panel_padding).flex().flex_col().gap(theme.stack_gap).child(heading).children(render_choices(inputs)))
        .child(body)
        .children(sheet.output().map(|_| render_footer(&inputs.on_event, theme)))
}

/// The answer, or what is happening instead.
fn render_status(status: &ActionStatus, on_event: &SheetAction, theme: &ThemeTokens) -> AnyElement {
    match status {
        ActionStatus::Picking => div().into_any_element(),
        ActionStatus::NeedsChoice => note(LucideIcon::Info, t!("post_actions.needs_choice").to_string(), theme).into_any_element(),
        ActionStatus::Running => running(on_event.clone(), theme).into_any_element(),
        ActionStatus::Done(output) => render_blocks(output, theme).into_any_element(),
        ActionStatus::Failed(reason) => art_state(ArtState::hint(Illustration::notice(NoticeKey::Error).width(theme.art_notice_width), reason.clone()), theme).into_any_element(),
    }
}

fn running(on_event: SheetAction, theme: &ThemeTokens) -> Div {
    let stop = Button::new(STOP_ID)
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::X))
        .label(t!("post_actions.cancel").to_string())
        .on_click(move |_, window, cx| on_event(SheetEvent::Cancel, window, cx));
    let art = Illustration::notice(NoticeKey::Working).width(theme.art_notice_width);
    art_state(ArtState::hint(art, t!("post_actions.running").to_string()), theme).child(stop)
}

/// An icon and a muted line.
fn note(icon: LucideIcon, text: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(Icon::new(icon).size(theme.icon_size_sm))
        .child(div().min_w_0().child(text))
}
