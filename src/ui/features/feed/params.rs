//! Main feed configuration parameters and callback types.

use crate::domain::{PostId, Profile};
use crate::state::{AppState, FeedTab};
use crate::theme::ThemeTokens;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::input::{InputState, TextareaState};
use super::stream::StreamInputs;
use crate::ui::features::post::{OpenSourceAction, OpenTextAction};
use gpui_kit::{App, Entity, Window};

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
    pub theme: &'a ThemeTokens,
    pub feed_tab: FeedTab,
    /// Whether the inspector shows every reply instead of the collapsed preview.
    pub inspector_expanded: bool,
    /// Window layout class (below the wide class the inspector stacks under the stream).
    pub layout: LayoutClass,
    /// Virtualized stream state and its cached rows.
    pub stream: StreamInputs,
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
    /// Hashtag / wikilink clicks in post bodies (tag search, open the linked post or doc).
    pub on_open_tag: OpenTextAction,
    pub on_open_wikilink: OpenTextAction,
    /// Source chips of agent posts (open the vault document, post, profile or web page).
    pub on_open_source: OpenSourceAction,
    pub on_add_document: AddDocCallback,
    pub on_delete_document: DeleteDocCallback,
    /// Opens a file picker and adds the chosen text file as a vault document.
    pub on_import_document: ImportDocCallback,
    pub on_cycle_profile: CycleProfileCallback,
}
