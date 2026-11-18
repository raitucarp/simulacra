//! The post picker's buttons in the Learn tab's modes: talk about a post (or its whole
//! thread) with the tutor, place a post at the palace locus being filled, or link a post or
//! thread to the open goal (a thread link is resolved to its first post by the shell).

use super::super::data::PostView;
use super::super::events::PickerMode;
use super::super::panel::ReviewPanel;
use super::super::text::tr;
use crate::domain::learning::{GoalLink, TutorSource};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const POST_ID_PREFIX: &str = "learning-picker-post-";
const THREAD_ID_PREFIX: &str = "learning-picker-thread-";
const PLACE_ID_PREFIX: &str = "learning-picker-place-";
const LINK_ID_PREFIX: &str = "learning-picker-link-";

impl ReviewPanel {
    pub(in crate::ui::features::review) fn learning_picker_buttons(&self, mode: PickerMode, index: usize, view: &PostView, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let id = view.post.id.as_str().to_string();
        let button = |prefix: &'static str, icon: LucideIcon, label: &str| Button::new(ElementId::NamedInteger(prefix.into(), index as u64)).ghost().xsmall().icon(Icon::new(icon)).label(tr(label));
        match mode {
            PickerMode::Place => {
                let place = button(PLACE_ID_PREFIX, LucideIcon::MapPin, "learning.picker.place_here").on_click(cx.listener(move |this, _, _, cx| this.place_post(id.clone(), cx)));
                div().flex_none().child(place)
            }
            PickerMode::Link => {
                let (post, thread) = (id.clone(), id);
                let link = button(LINK_ID_PREFIX, LucideIcon::Link2, "learning.picker.link_post").on_click(cx.listener(move |this, _, _, cx| this.link_to_goal(GoalLink::Post { id: post.clone() }, cx)));
                let whole = button(THREAD_ID_PREFIX, LucideIcon::ListTree, "learning.picker.whole_thread").on_click(cx.listener(move |this, _, _, cx| this.link_to_goal(GoalLink::Thread { root_id: thread.clone() }, cx)));
                div().flex_none().flex().gap(theme.spacing_xs).child(link).child(whole)
            }
            _ => {
                let (post, thread) = (id.clone(), id);
                let on_post = button(POST_ID_PREFIX, LucideIcon::MessageSquare, "learning.picker.this_post").on_click(cx.listener(move |this, _, _, cx| this.start_tutor(TutorSource::Post { id: post.clone() }, cx)));
                let on_thread = button(THREAD_ID_PREFIX, LucideIcon::ListTree, "learning.picker.whole_thread").on_click(cx.listener(move |this, _, _, cx| this.start_tutor(TutorSource::Thread { root_id: thread.clone() }, cx)));
                div().flex_none().flex().gap(theme.spacing_xs).child(on_post).child(on_thread)
            }
        }
    }

    /// Fills the picker's search with `query` (posts on a concept).
    pub fn filter_picker(&mut self, query: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.fields.search.update(cx, |input, cx| input.set_value(query.to_string(), window, cx));
        cx.notify();
    }
}
