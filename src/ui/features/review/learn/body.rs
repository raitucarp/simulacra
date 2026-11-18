//! The learn sheet's body: the source as it reads (the post, or the chosen passage), the
//! question / answer switch saying which face the source is, the passage picker of a
//! document, the written face, the tags, the problem line and the passages to save.

use super::request::{Granularity, LearnSource, Passage};
use super::LearnSheet;
use crate::domain::review::AnchorSide;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::review::render::face::{render_passage_face, render_post_face};
use crate::ui::features::review::text::{passage_place, tr};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const SIDE_ID_PREFIX: &str = "learn-side-";
const GRANULARITY_ID_PREFIX: &str = "learn-granularity-";
const PASSAGES_ID: &str = "learn-passages";
const PASSAGE_ID_PREFIX: &str = "learn-passage-";
const REMOVE_ID_PREFIX: &str = "learn-remove-";

fn side_icon(side: AnchorSide) -> LucideIcon {
    match side {
        AnchorSide::Back => LucideIcon::HelpCircle,
        AnchorSide::Front => LucideIcon::MessageSquareReply,
    }
}

impl LearnSheet {
    fn label(&self, text: String) -> Div {
        div().text_size(self.theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(self.theme.text_secondary).child(text)
    }

    fn hint(&self, text: String) -> Div {
        div().text_size(self.theme.font_size_2xs).text_color(self.theme.text_muted).child(text)
    }

    fn field(&self, label: String, input: &Entity<InputState>) -> Div {
        let theme = &self.theme;
        let frame = div().w_full().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).px(theme.spacing_sm).child(Input::new(input).bordered(false));
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(label)).child(frame)
    }

    pub(super) fn render_body(&self, source: &LearnSource, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let preview = match (source, &self.chosen) {
            (LearnSource::Post(view), _) => Some(render_post_face(view, theme)),
            (LearnSource::Document { title, .. }, Some(passage)) => Some(render_passage_face(title, passage.locator.as_ref(), &passage.text, theme)),
            (LearnSource::Document { .. }, None) => None,
        };
        let problem = self.problem.map(|problem| div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(tr(problem.message_key())));
        let side = self.side.key();
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .children(matches!(source, LearnSource::Document { .. }).then(|| self.render_passages(source, cx)))
            .children(preview)
            .child(self.render_sides(cx))
            .child(self.field(tr(&format!("review.learn.{side}.field")), &self.written))
            .child(self.hint(tr(&format!("review.learn.{side}.tip"))))
            .child(self.field(tr("review.learn.tags"), &self.tags))
            .children(problem)
            .when(!self.pending.is_empty(), |body| body.child(self.render_pending(cx)))
    }

    fn render_sides(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = AnchorSide::ALL.into_iter().map(|side| {
            let segment = Segment {
                id: ElementId::Name(format!("{SIDE_ID_PREFIX}{}", side.key()).into()),
                icon: side_icon(side),
                label: tr(&format!("review.learn.{}.switch", side.key())),
                active: self.side == side,
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |sheet, cx| sheet.select_side(side, cx));
            })
        });
        segmented_track(theme).children(pills)
    }

    fn granularity_switch(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let pills = Granularity::ALL.into_iter().map(|granularity| {
            let segment = Segment {
                id: ElementId::Name(format!("{GRANULARITY_ID_PREFIX}{}", granularity.key()).into()),
                icon: if granularity == Granularity::Sentence { LucideIcon::TextCursorInput } else { LucideIcon::Pilcrow },
                label: tr(&format!("review.learn.{}", granularity.key())),
                active: self.granularity == granularity,
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |sheet, cx| sheet.select_granularity(granularity, cx));
            })
        });
        segmented_track(theme).children(pills)
    }

    fn render_passages(&self, source: &LearnSource, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let passages: Vec<Passage> = self.request.as_ref().map(|r| r.passages(self.granularity).to_vec()).unwrap_or_default();
        let switch = self.granularity_switch(cx);
        let rows = passages.into_iter().enumerate().map(|(index, passage)| {
            let chosen = self.chosen.as_ref() == Some(&passage);
            let place = passage.locator.as_ref().map(passage_place).unwrap_or_default();
            div()
                .id(ElementId::NamedInteger(PASSAGE_ID_PREFIX.into(), index as u64))
                .flex()
                .flex_col()
                .px(theme.spacing_sm)
                .py(theme.spacing_xs)
                .rounded(theme.radius_sm)
                .when(chosen, |row| row.bg(theme.accent_brand_subtle))
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .child(div().text_size(theme.font_size_xs).text_color(if chosen { theme.text_primary } else { theme.text_secondary }).child(passage.text.clone()))
                .child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(place))
                .on_click(cx.listener(move |this, _, _, cx| this.choose_passage(passage.clone(), cx)))
        });
        let list = div().id(PASSAGES_ID).max_h(theme.review_sentence_list_height).overflow_y_scroll().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border_subtle).p(theme.spacing_2xs).flex().flex_col().children(rows);
        let title = match source {
            LearnSource::Document { title, .. } => t!("review.learn.pick_passage", title = title.as_str()).to_string(),
            LearnSource::Post(_) => String::new(),
        };
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(title)).child(switch).child(list)
    }

    fn render_pending(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let rows = self.pending.iter().enumerate().map(|(index, draft)| {
            let remove = Button::new(ElementId::NamedInteger(REMOVE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.learn.remove"))
                .on_click(cx.listener(move |this, _, _, cx| this.remove_pending(index, cx)));
            div()
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .child(Icon::new(side_icon(draft.side)).size(theme.icon_size_xs).text_color(theme.text_muted))
                .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(draft.written.clone()))
                .child(remove)
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(self.label(t!("review.learn.pending", count = self.pending.len()).to_string())).children(rows)
    }
}
