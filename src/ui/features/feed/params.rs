//! Main feed configuration parameters and callback types.

use crate::domain::{PostId, Profile};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::{App, Entity, Window};

pub type LikePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type ReplyPostCallback = Box<dyn Fn(PostId, &str, &mut Window, &mut App) + 'static>;
pub type SelectPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type RepostPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type QuotePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type AddDocCallback = Box<dyn Fn(String, String, &mut Window, &mut App) + 'static>;
pub type DeleteDocCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;
pub type CycleProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type LoadMoreCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Configuration parameters for rendering the main feed organism.
pub struct FeedParams<'a, C> {
    pub state: &'a AppState,
    pub custom_profile: Option<&'a Profile>,
    pub selected_post_id: Option<PostId>,
    pub composer: C,
    pub doc_title_input: &'a Entity<InputState>,
    pub doc_content_input: &'a Entity<TextareaState>,
    pub search_query: &'a str,
    pub theme: &'a ThemeTokens,
    pub feed_visible_count: Option<usize>,
    pub on_load_more: Option<LoadMoreCallback>,
    pub on_select_post: SelectPostCallback,
    pub on_like_post: LikePostCallback,
    pub on_reply_post: ReplyPostCallback,
    pub on_repost_post: RepostPostCallback,
    pub on_quote_post: QuotePostCallback,
    pub on_add_document: AddDocCallback,
    pub on_delete_document: DeleteDocCallback,
    pub on_cycle_profile: CycleProfileCallback,
}
