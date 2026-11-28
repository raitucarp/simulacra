//! N20: a post shown in the language the user is learning: its kept translation in place of
//! its body, and a line under its author naming the language, with "Show original" and "Send
//! words to review". While the AI is translating it, the line says so. Never on a locked post.

use super::controls::PostCardHandlers;
use super::lookup::PostLookup;
use crate::domain::post::translation::Translation;
use crate::domain::Post;
use crate::i18n::Language;
use crate::theme::ThemeTokens;
use crate::ui::features::data::{DataAction, DataEvent, ThoughtEvent, TranslateEvent};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// The translation `post` shows now (`None`: shown as written, or locked).
pub(crate) fn shown_translation<'a>(post: &'a Post, handlers: &PostCardHandlers) -> Option<&'a Translation> {
    let data = handlers.data.as_ref().filter(|data| !post.is_locked() && data.ideas.translated.contains(&post.id))?;
    post.extras.translation.as_deref().filter(|_| data.ideas.translated.contains(&post.id))
}

/// The native name of the language whose locale code is `code`.
pub(crate) fn language_name(code: &str) -> String {
    Language::ALL.into_iter().find(|language| language.code() == code).map_or_else(|| code.to_string(), |language| language.display_label().into_owned())
}

/// The translation in place of the body; `None` when the post shows as written.
pub(super) fn render_translated_body<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<AnyElement> {
    let kept = shown_translation(post, handlers)?;
    Some(super::body::render_post_body(&kept.text, lookup, theme).into_any_element())
}

fn button(post: &Post, part: &str, icon: LucideIcon, label: String, event: TranslateEvent, on_event: &DataAction) -> Button {
    let (on_event, event) = (on_event.clone(), DataEvent::Thought(ThoughtEvent::Translate(event)));
    Button::new(SharedString::from(format!("translation-{part}-{}", post.id))).ghost().xsmall().icon(Icon::new(icon)).label(label).on_click(move |_, window, cx| on_event(event.clone(), window, cx))
}

/// The line naming the language shown (or that it is being translated); `None` otherwise.
pub(super) fn render_translation_line(post: &Post, theme: &ThemeTokens, handlers: &PostCardHandlers) -> Option<Div> {
    let data = handlers.data.as_ref().filter(|_| !post.is_locked())?;
    let line = div().flex().flex_wrap().items_center().gap(theme.inline_gap).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(Icon::new(LucideIcon::Languages).xsmall().text_color(theme.accent_brand_text));
    if data.ideas.translating.as_ref() == Some(&post.id) {
        return Some(line.child(t!("posts.translation.translating").to_string()));
    }
    let kept = shown_translation(post, handlers)?;
    let shown = t!("posts.translation.shown", language = language_name(&kept.language)).to_string();
    let original = button(post, "original", LucideIcon::RotateCcw, t!("posts.translation.original").to_string(), TranslateEvent::Original(post.id.clone()), &data.on_event);
    let words = button(post, "words", LucideIcon::GraduationCap, t!("posts.translation.words_menu").to_string(), TranslateEvent::Words(post.id.clone()), &data.on_event);
    Some(line.child(shown).child(original).child(words))
}
