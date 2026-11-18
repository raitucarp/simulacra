//! Main feed configuration parameters and callback types.

use crate::domain::{PostId, Profile};
use crate::state::{AppState, FeedTab};
use crate::theme::ThemeTokens;
use crate::ui::responsive::LayoutClass;
use super::stream::StreamInputs;
use crate::ui::features::post::{OpenSourceAction, OpenTextAction};
use gpui_kit::{App, Window};

pub type LikePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type ReplyPostCallback = Box<dyn Fn(PostId, &str, &mut Window, &mut App) + 'static>;
pub type SelectPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type RepostPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type QuotePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type DeletePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type FindSimilarCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
/// Post "…" menu › Make flashcard (sb C1).
pub type MakeFlashcardCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type CycleProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type LoadMoreCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectFeedTabCallback = Box<dyn Fn(FeedTab, &mut Window, &mut App) + 'static>;
pub type ToggleRepliesCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
/// Runs a registry command (the stream header's capture and import buttons).
pub type RunCommandCallback = std::rc::Rc<dyn Fn(crate::ui::features::command_palette::CommandId, &mut Window, &mut App) + 'static>;

/// Configuration parameters for rendering the main feed organism.
pub struct FeedParams<'a, C> {
    pub state: &'a AppState,
    pub custom_profile: Option<&'a Profile>,
    pub selected_post_id: Option<PostId>,
    pub composer: C,
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
    /// Post "…" menu › Find similar.
    pub on_find_similar_post: FindSimilarCallback,
    /// Post "…" menu › Make flashcard (sb C1).
    pub on_make_flashcard_post: MakeFlashcardCallback,
    /// Hashtag / wikilink clicks in post bodies (tag search, open the linked post or doc).
    pub on_open_tag: OpenTextAction,
    pub on_open_wikilink: OpenTextAction,
    /// Source chips of agent posts (open the vault document, post, profile or web page).
    pub on_open_source: OpenSourceAction,
    /// Post images open the lightbox (sb C3).
    pub on_open_media: crate::ui::features::media::OpenMediaAction,
    pub on_cycle_profile: CycleProfileCallback,
    pub on_run_command: RunCommandCallback,
}
