//! The body of a locked post (X4): a placeholder with "Unlock…" while it is locked, and the
//! opened text with "Lock again" once the user unlocked it this session. The opened text
//! comes from the shell's in-memory cache only; it is rendered without link previews.

use super::body::{render_body_tokens, BodyLinks};
use super::lookup::PostLookup;
use super::controls::PostAction;
use crate::domain::post::tokenize_post_body;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, AnyElement, Div, IntoElement, ParentElement, SharedString, Styled};
use rust_i18n::t;
use crate::domain::PostId;
use crate::ui::features::data::{UnlockAction, UnlockEvent};
use std::collections::HashMap;
use std::rc::Rc;
use zeroize::Zeroizing;

const UNLOCK_ID: &str = "post-unlock-";
const RELOCK_ID: &str = "post-relock-";

/// An opened text, zeroized once the last card holding it is gone.
pub type OpenedText = Rc<Zeroizing<String>>;

/// What the feed knows to build every locked card's controls.
#[derive(Clone)]
pub struct LockSource {
    /// The opened text of every post unlocked this session.
    pub texts: Rc<HashMap<PostId, OpenedText>>,
    pub on_event: UnlockAction,
}

impl LockSource {
    /// The controls of the locked post `post`'s card.
    pub fn controls(&self, post: &PostId) -> LockControls {
        let send = |event: UnlockEvent| -> PostAction {
            let on_event = self.on_event.clone();
            Rc::new(move |window, cx| on_event(event.clone(), window, cx))
        };
        LockControls { post_id: post.to_string(), text: self.texts.get(post).cloned(), on_unlock: send(UnlockEvent::Open(post.clone())), on_relock: send(UnlockEvent::Relock(post.clone())) }
    }
}

/// A locked card's unlock state and actions.
#[derive(Clone)]
pub struct LockControls {
    pub post_id: String,
    /// The opened text while unlocked this session.
    pub text: Option<OpenedText>,
    /// Opens the unlock sheet.
    pub on_unlock: PostAction,
    /// Locks the post again.
    pub on_relock: PostAction,
}

fn note(icon: LucideIcon, text: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(Icon::new(icon).xsmall().text_color(theme.text_muted))
        .child(text)
}

/// The placeholder of a locked post, with "Unlock…" when the card offers it.
pub fn render_locked_placeholder(controls: Option<&LockControls>, theme: &ThemeTokens) -> AnyElement {
    let unlock = controls.map(|controls| {
        let on_unlock = controls.on_unlock.clone();
        Button::new(SharedString::from(format!("{UNLOCK_ID}{}", controls.post_id))).ghost().xsmall().icon(Icon::new(LucideIcon::LockOpen)).label(t!("posts.lock.unlock").to_string()).on_click(move |_, window, cx| on_unlock(window, cx))
    });
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.control_gap)
        .px(theme.spacing_md)
        .py(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_callout)
        .child(note(LucideIcon::LockKeyhole, t!("posts.lock.locked").to_string(), theme))
        .children(unlock)
        .into_any_element()
}

/// A locked card's body: the opened text and "Lock again", or the placeholder.
pub fn render_locked_body<L: PostLookup>(controls: &LockControls, lookup: L, theme: &ThemeTokens, links: &BodyLinks) -> AnyElement {
    let Some(text) = controls.text.as_ref() else { return render_locked_placeholder(Some(controls), theme) };
    let on_relock = controls.on_relock.clone();
    let relock = Button::new(SharedString::from(format!("{RELOCK_ID}{}", controls.post_id))).ghost().xsmall().icon(Icon::new(LucideIcon::LockKeyhole)).label(t!("posts.lock.relock").to_string()).on_click(move |_, window, cx| on_relock(window, cx));
    let bar = div().flex().items_center().justify_between().gap(theme.control_gap).child(note(LucideIcon::LockOpen, t!("posts.lock.unlocked").to_string(), theme)).child(relock);
    let links = BodyLinks { code: None, ..links.clone() }; // P5: a locked post's code never runs (its output would be stored in the open)
    div().w_full().flex().flex_col().gap(theme.spacing_xs).child(render_body_tokens(&tokenize_post_body(text), lookup, theme, &links)).child(bar).into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_unlock_text_is_translated() {
        let keys = ["locked", "unlock", "relock", "unlocked", "title", "subtitle", "explain", "password", "password_placeholder", "unlocking", "wrong_password", "damaged", "wait"];
        for key in keys.map(|key| format!("posts.lock.{key}")) {
            assert_ne!(rust_i18n::t!(&key, locale = "en"), key);
        }
    }
}
