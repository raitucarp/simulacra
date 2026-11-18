//! The card maker sheet: header, kind switch, sentences, fields, the list and the footer.

use super::fields::MakerKind;
use super::FlashcardMaker;
use crate::ui::atoms::ElevationExt;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::features::review::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const BACKDROP_ID: &str = "maker-backdrop";
const CARD_ID: &str = "maker-card";
const BODY_ID: &str = "maker-body";
const CLOSE_ID: &str = "maker-close";
const KIND_ID_PREFIX: &str = "maker-kind-";
const ESCAPE_KEY: &str = "escape";

fn kind_icon(kind: MakerKind) -> LucideIcon {
    match kind {
        MakerKind::Basic => LucideIcon::MessageSquare,
        MakerKind::Cloze => LucideIcon::Pilcrow,
        MakerKind::Concept => LucideIcon::BookMarked,
    }
}

impl Render for FlashcardMaker {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(request) = self.request.clone() else { return div().into_any_element() };
        let theme = self.theme.clone();
        let card = self.render_sheet(&request, cx);
        div()
            .id(BACKDROP_ID)
            .track_focus(&self.focus)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == ESCAPE_KEY {
                    this.close(cx);
                }
            }))
            .absolute()
            .inset_0()
            .occlude()
            .bg(theme.overlay)
            .flex()
            .flex_col()
            .items_center()
            .px(theme.panel_padding)
            .pt(theme.palette_top_offset)
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| this.close(cx)))
            .child(card)
            .into_any_element()
    }
}

impl FlashcardMaker {
    /// The elevated sheet (clicks inside never reach the scrim).
    fn render_sheet(&self, request: &super::MakerRequest, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = self.theme.clone();
        let body = div()
            .id(BODY_ID)
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .p(theme.card_padding)
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.render_kinds(cx))
            .children((request.sentences.len() > 1).then(|| self.render_sentences(&request.sentences, cx)))
            .child(self.render_fields(cx))
            .children((!self.pending.is_empty()).then(|| self.render_pending(cx)));
        div()
            .id(CARD_ID)
            .w(theme.review_maker_width)
            .max_w_full()
            .max_h(theme.review_maker_max_height)
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(theme.radius_lg)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .elevation_shadow(&theme)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(self.render_top(&request.title, cx))
            .child(body)
            .child(self.render_footer(cx))
    }

    fn render_top(&self, title: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let titles = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("review.maker.title")))
            .when(!title.is_empty(), |titles| titles.child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("review.maker.from", source = title).to_string())));
        let close = Button::new(CLOSE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.maker.close")).on_click(cx.listener(|this, _, _, cx| this.close(cx)));
        div()
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.card_padding)
            .py(theme.spacing_sm)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(Icon::new(LucideIcon::Layers).size(theme.icon_size_sm).text_color(theme.accent_brand))
            .child(titles)
            .child(close)
    }

    fn render_kinds(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pills = MakerKind::ALL.into_iter().map(|kind| {
            let segment = Segment {
                id: ElementId::Name(format!("{KIND_ID_PREFIX}{}", kind.key()).into()),
                icon: kind_icon(kind),
                label: tr(&format!("review.maker.kind.{}", kind.key())),
                active: self.kind == kind,
                active_icon_color: theme.accent_brand,
            };
            let entity = cx.entity().downgrade();
            segment_pill(segment, theme, move |_, cx| {
                let _ = entity.update(cx, |maker, cx| maker.select_kind(kind, cx));
            })
        });
        segmented_track(theme).children(pills)
    }
}
