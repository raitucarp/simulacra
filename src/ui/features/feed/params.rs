//! Main feed configuration parameters and callback types.

use crate::domain::{PostId, Profile};
use crate::state::{AppState, FeedTab};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::{App, Entity, ScrollHandle, Window};

pub type LikePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type ReplyPostCallback = Box<dyn Fn(PostId, &str, &mut Window, &mut App) + 'static>;
pub type SelectPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type RepostPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type QuotePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type DeletePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type ImportDocCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type AddDocCallback = Box<dyn Fn(String, String, &mut Window, &mut App) + 'static>;
pub type DeleteDocCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;
pub type CycleProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type LoadMoreCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectFeedTabCallback = Box<dyn Fn(FeedTab, &mut Window, &mut App) + 'static>;
pub type ToggleRepliesCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

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
    pub feed_tab: FeedTab,
    /// Whether the inspector shows every reply instead of the collapsed preview.
    pub inspector_expanded: bool,
    pub stream_scroll: &'a ScrollHandle,
    pub on_load_more: Option<LoadMoreCallback>,
    pub on_select_feed_tab: SelectFeedTabCallback,
    pub on_toggle_replies: ToggleRepliesCallback,
    pub on_select_post: SelectPostCallback,
    pub on_like_post: LikePostCallback,
    pub on_reply_post: ReplyPostCallback,
    pub on_repost_post: RepostPostCallback,
    pub on_quote_post: QuotePostCallback,
    /// Deletes one of the user's own thoughts (after the card's confirmation dialog).
    pub on_delete_post: DeletePostCallback,
    pub on_add_document: AddDocCallback,
    pub on_delete_document: DeleteDocCallback,
    /// Opens a file picker and adds the chosen text file as a vault document.
    pub on_import_document: ImportDocCallback,
    pub on_cycle_profile: CycleProfileCallback,
}
