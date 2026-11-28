//! Learning on a post (sb C1b): the review items of the post menu's Study section (add a
//! review question or answer to the post, edit or stop its review, learn its thread's
//! connections) and the small indicator a post in review carries, saying when it is due.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{post_ids as ids, MenuItem, MenuText};
use gpui_kit::component::Icon;
use gpui_kit::{div, App, Div, FontWeight, ParentElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// What the learner asked of a post.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearnRequestKind {
    /// Write a question the post answers.
    AddQuestion,
    /// Write the answer the post prompts.
    AddAnswer,
    Edit,
    Stop,
    /// A connection map of the post's thread.
    ThreadConnections,
}

/// Where a post in review stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearnStatus {
    New,
    /// Due today or overdue.
    Due,
    /// Due in this many days.
    Later(u32),
    Suspended,
}

/// Runs a learn request on the card's post.
pub type LearnAction = Rc<dyn Fn(LearnRequestKind, &mut Window, &mut App) + 'static>;

/// The learn controls of one post card.
#[derive(Clone)]
pub struct LearnControls {
    /// The post's review state (none while it is not being learnt).
    pub status: Option<LearnStatus>,
    pub on_learn: LearnAction,
}

impl LearnControls {
    /// Controls that do nothing (cards outside the stream).
    pub fn inert() -> Self {
        Self { status: None, on_learn: Rc::new(|_, _, _| {}) }
    }
}

fn item(id: &'static str, icon: LucideIcon, key: &'static str, kind: LearnRequestKind, action: &LearnAction) -> MenuItem {
    let action = action.clone();
    MenuItem::new(id, MenuText::key(key)).icon(icon).on_click(move |window, cx| action(kind, window, cx))
}

/// The review items of the post menu's Study section: write a review question or answer for
/// the post (or, once it is in review, edit or stop its review), and learn its thread's
/// connections.
pub(crate) fn learn_items(controls: &LearnControls) -> Vec<MenuItem> {
    let action = &controls.on_learn;
    let mut items = if controls.status.is_some() {
        vec![item(ids::LEARN_EDIT, LucideIcon::Edit, "post.learn.edit", LearnRequestKind::Edit, action), item(ids::LEARN_STOP, LucideIcon::CircleStop, "post.learn.stop", LearnRequestKind::Stop, action)]
    } else {
        vec![item(ids::LEARN_ADD_QUESTION, LucideIcon::HelpCircle, "menus.post.review_question", LearnRequestKind::AddQuestion, action), item(ids::LEARN_ADD_ANSWER, LucideIcon::MessageSquareReply, "menus.post.review_answer", LearnRequestKind::AddAnswer, action)]
    };
    items.push(item(ids::LEARN_CONNECTIONS, LucideIcon::Share2, "post.learn.thread_connections", LearnRequestKind::ThreadConnections, action));
    items
}

/// The "learning" indicator of a post in review: a cap and when it is due.
pub(crate) fn render_learn_badge(status: LearnStatus, theme: &ThemeTokens) -> Div {
    let (label, color, background) = match status {
        LearnStatus::Due => (t!("post.learn.due").to_string(), theme.accent_brand, theme.accent_brand_subtle),
        LearnStatus::New => (t!("post.learn.new").to_string(), theme.info, theme.bg_elevated),
        LearnStatus::Later(days) => (t!("post.learn.later", days = days).to_string(), theme.text_muted, theme.bg_elevated),
        LearnStatus::Suspended => (t!("post.learn.paused").to_string(), theme.text_muted, theme.bg_elevated),
    };
    div()
        .flex_none()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .px(theme.spacing_xs)
        .rounded(theme.radius_full)
        .bg(background)
        .text_color(color)
        .child(Icon::new(LucideIcon::GraduationCap).size(theme.icon_size_xs))
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::MEDIUM).child(label))
}
