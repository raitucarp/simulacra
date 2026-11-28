//! The post picker: search the stream, then learn a post (write its question or answer) or
//! gather posts for a connection map.

use super::super::data::PostView;
use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::{Picker, ReviewPanel, Screen};
use super::super::text::tr;
use super::common::{block_title, caption, control_row, surface};
use crate::domain::post::readable_line;
use crate::domain::review::quiz::piece_text;
use crate::domain::review::AnchorSide;
use crate::i18n::t_count;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const ROW_ID_PREFIX: &str = "review-picker-row-";
const QUESTION_ID_PREFIX: &str = "review-picker-question-";
const ANSWER_ID_PREFIX: &str = "review-picker-answer-";
const BACK_ID: &str = "review-picker-back";
const CREATE_ID: &str = "review-picker-create";
/// Characters of a post shown in a row.
const ROW_CHARS: usize = 200;
const ROW_LINES: usize = 2;
/// Fewest posts that seed a map.
const MIN_MAP_POSTS: usize = 2;

impl ReviewPanel {
    pub(super) fn render_picker(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(picker) = &self.picker else { return surface(theme) };
        let key = match picker.mode {
            PickerMode::Learn => "review.picker.learn",
            PickerMode::Map => "review.picker.map",
            PickerMode::Tutor => "learning.picker.tutor",
            PickerMode::Place => "learning.picker.place",
            PickerMode::Link => "learning.picker.link",
        };
        let query = self.picker_query(cx);
        let visible = picker.visible(&query);
        let rows: Vec<AnyElement> = visible.iter().enumerate().map(|(index, view)| self.picker_row(picker, index, view, cx).into_any_element()).collect();
        let empty = rows.is_empty().then(|| caption(tr("review.picker.none"), theme));
        surface(theme)
            .child(block_title(LucideIcon::ListChecks, theme.accent_brand, tr(&format!("{key}_title")), theme))
            .child(caption(tr(&format!("{key}_hint")), theme))
            .child(control_row(theme).child(self.field(tr("review.picker.search_label"), &self.fields.search)))
            .child(div().flex().flex_col().children(rows).children(empty))
            .child(self.picker_footer(picker, cx))
    }

    fn picker_row(&self, picker: &Picker, index: usize, view: &PostView, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let id = view.post.id.clone();
        let picked = picker.picked.contains(&id);
        let text = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().truncate().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(format!("{} {}", view.author.name, view.author.handle)))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).line_clamp(ROW_LINES).text_ellipsis().child(piece_text(&readable_line(&view.post.content), ROW_CHARS)));
        let row = div()
            .id(ElementId::NamedInteger(ROW_ID_PREFIX.into(), index as u64))
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .py(theme.spacing_xs)
            .px(theme.spacing_xs)
            .rounded(theme.radius_sm)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .when(picked, |row| row.bg(theme.accent_brand_subtle))
            .child(render_profile_avatar(&view.author, theme.avatar_size_sm, theme))
            .child(text);
        match picker.mode {
            PickerMode::Map => row
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .on_click(cx.listener(move |this, _, _, cx| this.toggle_picked(id.clone(), cx)))
                .when(picked, |row| row.child(Icon::new(LucideIcon::Check).size(theme.icon_size_sm).text_color(theme.accent_brand_text))),
            PickerMode::Learn => row.child(self.learn_buttons(index, view, cx)),
            PickerMode::Tutor | PickerMode::Place | PickerMode::Link => row.child(self.learning_picker_buttons(picker.mode, index, view, cx)),
        }
    }

    /// "Question" and "Answer" (or "Edit" once the post is being learnt).
    fn learn_buttons(&self, index: usize, view: &PostView, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let existing = self.data.items.iter().find(|item| !item.retired && item.is_on_post(view.post.id.as_str())).map(|item| item.id.clone());
        let button = |prefix: &'static str, icon: LucideIcon, label: &str| Button::new(ElementId::NamedInteger(prefix.into(), index as u64)).ghost().xsmall().icon(Icon::new(icon)).label(tr(label));
        if let Some(item) = existing {
            let edit = button(QUESTION_ID_PREFIX, LucideIcon::GraduationCap, "review.picker.learning").on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::EditItem(item.clone()))));
            return div().flex_none().child(edit);
        }
        let (question_post, answer_post) = (view.post.id.clone(), view.post.id.clone());
        let question = button(QUESTION_ID_PREFIX, LucideIcon::HelpCircle, "post.learn.add_question")
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::LearnPost(question_post.clone(), AnchorSide::Back))));
        let answer = button(ANSWER_ID_PREFIX, LucideIcon::MessageSquareReply, "post.learn.add_answer")
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::LearnPost(answer_post.clone(), AnchorSide::Front))));
        div().flex_none().flex().gap(theme.spacing_xs).child(question).child(answer)
    }

    fn picker_footer(&self, picker: &Picker, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let back_to = picker.from;
        let back_label = match back_to {
            Screen::Today => "review.back_today",
            Screen::Learn => "learning.back",
            Screen::Stats => "review.back_stats",
            _ => "review.back_items",
        };
        let back = Button::new(BACK_ID).ghost().small().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr(back_label))
            .on_click(cx.listener(move |this, _, _, cx| this.open_tab(back_to, cx)));
        let row = control_row(theme).child(back).child(div().flex_1());
        match picker.mode {
            PickerMode::Learn | PickerMode::Tutor | PickerMode::Place | PickerMode::Link => row,
            PickerMode::Map => row.child(
                Button::new(CREATE_ID).primary().small().icon(Icon::new(LucideIcon::Share2)).label(t_count("review.picker.create_map", picker.picked.len()))
                    .disabled(picker.picked.len() < MIN_MAP_POSTS)
                    .on_click(cx.listener(|this, _, _, cx| this.map_from_picked(cx))),
            ),
        }
    }
}
