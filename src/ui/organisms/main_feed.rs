//! Center timeline feed organism featuring:
//! 1. Threads-style Main Dialectic Stream ("For you" / "Dialectic")
//! 2. Focus Thread & Epistemic Inspector (replacing redundant second timeline)
//! 3. Resizable split panels using gpui-kit h_resizable
//! 4. 100% tokenized via ThemeTokens (colors, typography, spacing, radii)

use super::agent_lab_view::render_agent_lab_view;
use super::knowledge_vault_view::render_knowledge_vault_view;
use super::mind_graph_view::render_mind_graph_view;
use crate::domain::{PostId, Profile};
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::molecules::{render_post_item, ThreadlineStyle};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::component::resizable::{h_resizable, resizable_panel};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

pub type LikePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type ReplyPostCallback = Box<dyn Fn(PostId, &str, &mut Window, &mut App) + 'static>;
pub type SelectPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type RepostPostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type QuotePostCallback = Box<dyn Fn(PostId, &mut Window, &mut App) + 'static>;
pub type AddDocCallback = Box<dyn Fn(String, String, &mut Window, &mut App) + 'static>;
pub type DeleteDocCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;
pub type TriggerSimCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type CycleProfileCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type LoadMoreCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Default initial window size for the main timeline stream.
pub const DEFAULT_FEED_WINDOW_SIZE: usize = 8;
/// Number of additional thoughts to load when expanding the timeline stream.
pub const FEED_WINDOW_INCREMENT: usize = 12;
/// Maximum number of direct/indirect replies rendered in the focus thread inspector.
pub const MAX_INSPECTOR_REPLIES: usize = 4;

/// Computes threadline style for a linear sequence of thoughts in the feed stream.
///
/// Supports both reverse-chronological streams (standard feed ordering: `created_at DESC`,
/// where child replies precede parent posts) and chronological streams (e.g. thread inspection).
///
/// Threadline connectivity rules:
/// - Connects upward if the post above is a child replying to `current` (reverse-chronological)
///   or if `current` is a child replying to the post above (chronological).
/// - Connects downward if `current` is a child replying to the post below (reverse-chronological)
///   or if the post below is a child replying to `current` (chronological).
pub fn compute_stream_threadline_style(posts: &[&crate::domain::Post], index: usize) -> ThreadlineStyle {
    if posts.is_empty() || index >= posts.len() {
        return ThreadlineStyle::None;
    }
    let current = posts[index];

    let has_line_above = index > 0 && {
        let prev = posts[index - 1];
        // Reverse-chronological: post above is a reply to current post
        prev.reply_to_id.as_ref() == Some(&current.id)
            // Chronological: current post is a reply to post above
            || current.reply_to_id.as_ref() == Some(&prev.id)
    };

    let has_line_below = index + 1 < posts.len() && {
        let next = posts[index + 1];
        // Reverse-chronological: current post is a reply to post below
        current.reply_to_id.as_ref() == Some(&next.id)
            // Chronological: post below is a reply to current post
            || next.reply_to_id.as_ref() == Some(&current.id)
    };

    match (has_line_above, has_line_below) {
        (false, false) => ThreadlineStyle::None,
        (false, true) => ThreadlineStyle::ParentHasReplies,
        (true, true) => ThreadlineStyle::MiddleReply,
        (true, false) => ThreadlineStyle::LastReply,
    }
}

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
    pub on_trigger_simulation: TriggerSimCallback,
    pub on_cycle_profile: CycleProfileCallback,
}

pub fn render_main_feed<C: IntoElement>(
    params: FeedParams<'_, C>,
) -> impl IntoElement {
    let state = params.state;
    let theme = params.theme;

    let _network_name = state
        .current_network()
        .map(|n| n.name.clone())
        .unwrap_or_else(|| t!("feed.general_timeline").to_string());

    let on_like = Rc::new(params.on_like_post);
    let on_reply = Rc::new(params.on_reply_post);
    let on_select = Rc::new(params.on_select_post);
    let on_repost = Rc::new(params.on_repost_post);
    let on_quote = Rc::new(params.on_quote_post);
    let on_load_more = params.on_load_more.map(Rc::new);

    let query_trimmed = params.search_query.trim();
    let requested_window = params.feed_visible_count.unwrap_or(DEFAULT_FEED_WINDOW_SIZE);
    let (total_stream_count, visible_stream): (usize, Vec<&crate::domain::Post>) = if query_trimmed.is_empty() {
        let total = state.current_network_post_count();
        let limit = requested_window.min(total);
        let slice: Vec<_> = state.current_network_posts().take(limit + 1).collect();
        (total, slice)
    } else {
        let all_filtered: Vec<_> = state.filtered_posts(query_trimmed).collect();
        let total = all_filtered.len();
        (total, all_filtered)
    };
    let window_limit = requested_window.min(total_stream_count);

    // Determine currently focused post (or fallback to top/first post in feed) via O(1) lookup
    let focused_post = params
        .selected_post_id
        .as_ref()
        .and_then(|id| state.get_post(id))
        .or_else(|| visible_stream.first().copied())
        .or_else(|| state.posts.first());

    // 1. Render Main Dialectic Stream Post Cards (Windowed progressive rendering)
    let mut general_post_elements = Vec::with_capacity(window_limit + 1);
    for (idx, post) in visible_stream.iter().take(window_limit).enumerate() {
        let post_id = post.id.clone();
        let post_id_for_reply = post.id.clone();
        let post_id_for_select = post.id.clone();
        let post_id_for_repost = post.id.clone();
        let post_id_for_quote = post.id.clone();
        let author_handle = post.author.handle.clone();
        // Stream posts render with seamless flat OLED black background without card boxes (R2)
        let is_focused = false;
        let threadline_style = compute_stream_threadline_style(&visible_stream, idx);

        let cb_like = on_like.clone();
        let cb_reply = on_reply.clone();
        let cb_select = on_select.clone();
        let cb_repost = on_repost.clone();
        let cb_quote = on_quote.clone();

        general_post_elements.push(
            render_post_item(
                post,
                state,
                theme,
                is_focused,
                threadline_style,
                move |window, cx| {
                    cb_select(post_id_for_select.clone(), window, cx);
                },
                move |_, window, cx| {
                    cb_like(post_id.clone(), window, cx);
                },
                move |_, window, cx| {
                    cb_reply(post_id_for_reply.clone(), &author_handle, window, cx);
                },
                move |_, window, cx| {
                    cb_repost(post_id_for_repost.clone(), window, cx);
                },
                move |_, window, cx| {
                    cb_quote(post_id_for_quote.clone(), window, cx);
                },
            )
            .into_any_element(),
        );
    }

    // If more posts exist beyond the window limit, render tokenized Load More trigger
    if window_limit < total_stream_count {
        let remaining = total_stream_count - window_limit;
        let cb_more = on_load_more.clone();
        general_post_elements.push(
            div()
                .id("feed-load-more-container")
                .w_full()
                .py(theme.spacing_lg)
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap(theme.spacing_xs)
                .child(
                    div()
                        .id("btn-load-more-posts")
                        .px(theme.spacing_lg)
                        .py(theme.spacing_sm)
                        .rounded(theme.radius_md)
                        .bg(theme.bg_card)
                        .border(theme.border_width_hairline)
                        .border_color(theme.border_subtle)
                        .hover(move |s| s.bg(theme.hover_bg).border_color(theme.border_strong))
                        .cursor_pointer()
                        .when_some(cb_more, |this, cb| {
                            this.on_click(move |_, window, cx| {
                                cb(window, cx);
                            })
                        })
                        .flex()
                        .items_center()
                        .gap(theme.spacing_sm)
                        .child(
                            div()
                                .font_family(theme.font_family_ui.clone())
                                .text_size(theme.font_size_sm)
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.text_secondary)
                                .child(t!("feed.show_more", count = remaining).to_string()),
                        )
                        .child(
                            div()
                                .w(theme.font_size_sm)
                                .h(theme.font_size_sm)
                                .text_color(theme.text_muted)
                                .child(Icon::new(LucideIcon::ChevronDown)),
                        ),
                )
                .child(
                    div()
                        .font_family(theme.font_family_mono.clone())
                        .text_size(theme.font_size_2xs)
                        .text_color(theme.text_muted)
                        .child(format!("Showing {} of {} thoughts", window_limit, total_stream_count)),
                )
                .into_any_element(),
        );
    }
    let is_general_empty = general_post_elements.is_empty();

    let mut thread_replies_elements = Vec::new();
    let mut total_thread_replies_count = 0;
    if let Some(root) = focused_post {
        // Retrieve direct or linked replies in O(1) time via precomputed index
        let replies_list = state.replies_for_post(&root.id);
        total_thread_replies_count = replies_list.len();
        let rendered_replies_len = total_thread_replies_count.min(MAX_INSPECTOR_REPLIES);

        for (idx, reply) in replies_list.iter().take(MAX_INSPECTOR_REPLIES).enumerate() {
            let post_id = reply.id.clone();
            let post_id_for_reply = reply.id.clone();
            let post_id_for_select = reply.id.clone();
            let post_id_for_repost = reply.id.clone();
            let post_id_for_quote = reply.id.clone();
            let author_handle = reply.author.handle.clone();

            let cb_like = on_like.clone();
            let cb_reply = on_reply.clone();
            let cb_select = on_select.clone();
            let cb_repost = on_repost.clone();
            let cb_quote = on_quote.clone();

            let threadline_style = if idx + 1 < rendered_replies_len {
                ThreadlineStyle::MiddleReply
            } else {
                ThreadlineStyle::LastReply
            };

            thread_replies_elements.push(
                render_post_item(
                    reply,
                    state,
                    theme,
                    false,
                    threadline_style,
                    move |window, cx| {
                        cb_select(post_id_for_select.clone(), window, cx);
                    },
                    move |_, window, cx| {
                        cb_like(post_id.clone(), window, cx);
                    },
                    move |_, window, cx| {
                        cb_reply(post_id_for_reply.clone(), &author_handle, window, cx);
                    },
                    move |_, window, cx| {
                        cb_repost(post_id_for_repost.clone(), window, cx);
                    },
                    move |_, window, cx| {
                        cb_quote(post_id_for_quote.clone(), window, cx);
                    },
                )
                .into_any_element(),
            );
        }
    }
    let has_thread_replies = !thread_replies_elements.is_empty();
    let thread_replies_count = total_thread_replies_count;

    div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        // Sub-view Router
        .when(state.active_tab == NavigationTab::Timeline, |this| {
            this.child(
                // Resizable Dual Feed Layout: Left Stream ("For you") + Right Focus Thread Inspector
                h_resizable("timeline-stream-inspector-resizable")
                    // PANE 1: Main Dialectic Stream ("For you")
                    .child(
                        resizable_panel()
                            .size(px(520.0))
                            .size_range(px(380.0)..px(720.0))
                            .child(
                                div()
                                    .w_full()
                                    .h_full()
                                    .flex()
                                    .flex_col()
                                    .border_r(theme.border_width_hairline)
                                    .border_color(theme.border_subtle)
                                    // Stream Header: Threads-style "For you" / "Dialectic" Tabs + Status
                                    .child(
                                        div()
                                            .w_full()
                                            .px(theme.spacing_lg)
                                            .h(theme.feed_header_height)
                                            .border_b(theme.border_width_hairline)
                                            .border_color(theme.border_subtle)
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(theme.spacing_xl)
                                                    .h_full()
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .h_full()
                                                            .justify_between()
                                                            .pt(theme.spacing_sm)
                                                            .child(
                                                                div()
                                                                    .font_family(theme.font_family_display.clone())
                                                                    .text_size(theme.font_size_base)
                                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                                    .text_color(theme.text_primary)
                                                                    .child(t!("feed.for_you").to_string()),
                                                            )
                                                            .child(
                                                                div()
                                                                    .w(px(48.0))
                                                                    .h(theme.border_width_threadline)
                                                                    .rounded(theme.radius_full)
                                                                    .bg(theme.accent_brand),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .pt(theme.spacing_sm)
                                                            .h_full()
                                                            .child(
                                                                div()
                                                                    .font_family(theme.font_family_display.clone())
                                                                    .text_size(theme.font_size_base)
                                                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                                    .text_color(theme.text_muted)
                                                                    .child(t!("feed.dialectic").to_string()),
                                                            ),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(theme.spacing_xs)
                                                    .px(theme.spacing_sm)
                                                    .py(theme.spacing_2xs)
                                                    .rounded(theme.radius_full)
                                                    .bg(theme.bg_card)
                                                    .border(theme.border_width_hairline)
                                                    .border_color(theme.border_subtle)
                                                    .child(
                                                        div()
                                                            .w(theme.font_size_xs)
                                                            .h(theme.font_size_xs)
                                                            .text_color(theme.accent_success)
                                                            .child(Icon::new(LucideIcon::Activity)),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family(theme.font_family_mono.clone())
                                                            .text_size(theme.font_size_xs)
                                                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                            .text_color(theme.accent_success)
                                                            .child(t!("feed.loop_active").to_string()),
                                                    ),
                                            ),
                                    )
                                    // Pane 1 Composer
                                    .child(params.composer)
                                    // Pane 1 Scrollable Stream
                                    .child(
                                        div()
                                            .id("general-feed-scroll-stream")
                                            .flex_1()
                                            .overflow_y_scrollbar()
                                            .flex()
                                            .flex_col()
                                            .when(is_general_empty, |this| {
                                                this.p(theme.spacing_2xl).items_center().justify_center().child(
                                                    div()
                                                        .text_size(theme.font_size_sm)
                                                        .text_color(theme.text_muted)
                                                        .child(t!("feed.no_posts").to_string()),
                                                )
                                            })
                                            .when(!is_general_empty, |this| this.children(general_post_elements)),
                                    ),
                            ),
                    )
                    // PANE 2: Focus Thread & Epistemic Inspector (Replaces redundant timeline)
                    .child(
                        resizable_panel()
                            .child(
                                div()
                                    .w_full()
                                    .h_full()
                                    .flex()
                                    .flex_col()
                                    .bg(theme.bg_app)
                                    // Inspector Header
                                    .child(
                                        div()
                                            .w_full()
                                            .px(theme.spacing_lg)
                                            .h(theme.feed_header_height)
                                            .border_b(theme.border_width_hairline)
                                            .border_color(theme.border_subtle)
                                            .flex()
                                            .items_center()
                                            .justify_between()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap(theme.spacing_sm)
                                                    .child(
                                                        Icon::new(LucideIcon::MessageSquare)
                                                            .size(theme.font_size_md)
                                                            .text_color(theme.accent_brand),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family(theme.font_family_display.clone())
                                                            .text_size(theme.font_size_base)
                                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(t!("feed.inspector_title").to_string()),
                                                    ),
                                            )
                                            .when_some(focused_post, |this, _root| {
                                                this.child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap(theme.spacing_xs)
                                                        .px(theme.spacing_sm)
                                                        .py(theme.spacing_2xs)
                                                        .rounded(theme.radius_sm)
                                                        .bg(theme.hover_bg)
                                                        .font_family(theme.font_family_mono.clone())
                                                        .text_size(theme.font_size_xs)
                                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                        .text_color(theme.text_secondary)
                                                        .child(t!("feed.replies_count", count = thread_replies_count).to_string()),
                                                )
                                            }),
                                    )
                                    // Inspector Scrollable Thread Body
                                    .child(
                                        div()
                                            .id("focus-thread-scroll-stream")
                                            .flex_1()
                                            .overflow_y_scrollbar()
                                            .flex()
                                            .flex_col()
                                            .when_some(focused_post, |this, root| {
                                                let root_id = root.id.clone();
                                                let root_id_reply = root.id.clone();
                                                let root_id_repost = root.id.clone();
                                                let root_id_quote = root.id.clone();
                                                let root_author = root.author.handle.clone();
                                                let cb_like = on_like.clone();
                                                let cb_reply = on_reply.clone();
                                                let cb_repost = on_repost.clone();
                                                let cb_quote = on_quote.clone();

                                                let root_threadline_style = if has_thread_replies {
                                                    ThreadlineStyle::ParentHasReplies
                                                } else {
                                                    ThreadlineStyle::None
                                                };

                                                this
                                                    // 1. Root Inspected Post (Elevated Card)
                                                    .child(
                                                        div()
                                                            .w_full()
                                                            .bg(theme.bg_elevated)
                                                            .child(
                                                                render_post_item(
                                                                    root,
                                                                    state,
                                                                    theme,
                                                                    true,
                                                                    root_threadline_style,
                                                                    |_w, _cx| {},
                                                                    move |_, window, cx| {
                                                                        cb_like(root_id.clone(), window, cx);
                                                                    },
                                                                    move |_, window, cx| {
                                                                        cb_reply(root_id_reply.clone(), &root_author, window, cx);
                                                                    },
                                                                    move |_, window, cx| {
                                                                        cb_repost(root_id_repost.clone(), window, cx);
                                                                    },
                                                                    move |_, window, cx| {
                                                                        cb_quote(root_id_quote.clone(), window, cx);
                                                                    },
                                                                ),
                                                            ),
                                                    )
                                                    // 2. Linear Connected Thread Replies
                                                    .when(has_thread_replies, |this| {
                                                        this.children(thread_replies_elements)
                                                    })
                                                    // 3. Grounding Context Insight if no replies yet
                                                    .when(!has_thread_replies, |this| {
                                                        this.child(
                                                            div()
                                                                .m(theme.spacing_lg)
                                                                .p(theme.spacing_md)
                                                                .rounded(theme.radius_md)
                                                                .bg(theme.bg_callout)
                                                                .border(theme.border_width_hairline)
                                                                .border_color(theme.border_subtle)
                                                                .flex()
                                                                .flex_col()
                                                                .gap(theme.spacing_xs)
                                                                .child(
                                                                    div()
                                                                        .flex()
                                                                        .items_center()
                                                                        .gap(theme.spacing_xs)
                                                                        .child(
                                                                            Icon::new(LucideIcon::Sparkles)
                                                                                .size(theme.font_size_sm)
                                                                                .text_color(theme.accent_brand),
                                                                        )
                                                                        .child(
                                                                            div()
                                                                                .font_family(theme.font_family_display.clone())
                                                                                .text_size(theme.font_size_xs)
                                                                                .font_weight(gpui_kit::FontWeight::BOLD)
                                                                                .text_color(theme.text_primary)
                                                                                .child(t!("feed.grounding_context_title").to_string()),
                                                                        ),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .font_family(theme.font_family_body.clone())
                                                                        .text_size(theme.font_size_xs)
                                                                        .text_color(theme.text_secondary)
                                                                        .child(t!("feed.grounding_context_desc").to_string()),
                                                                ),
                                                        )
                                                    })
                                            })
                                            .when(focused_post.is_none(), |this| {
                                                this.p(theme.spacing_2xl).items_center().justify_center().child(
                                                    div()
                                                        .text_size(theme.font_size_sm)
                                                        .text_color(theme.text_muted)
                                                        .child(t!("feed.select_to_inspect").to_string()),
                                                )
                                            }),
                                    )
                                    // Pinned Bottom Quick Reply Bar
                                    .when_some(focused_post, |this, root| {
                                        let root_id = root.id.clone();
                                        let author_handle = root.author.handle.clone();
                                        let cb_reply = on_reply.clone();

                                        this.child(
                                            div()
                                                .w_full()
                                                .px(theme.spacing_md)
                                                .py(theme.spacing_sm)
                                                .border_t(theme.border_width_hairline)
                                                .border_color(theme.border_subtle)
                                                .bg(theme.bg_elevated)
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .gap(theme.spacing_sm)
                                                .child(
                                                    div()
                                                        .font_family(theme.font_family_ui.clone())
                                                        .flex()
                                                        .items_center()
                                                        .gap(theme.spacing_xs)
                                                        .text_size(theme.font_size_xs)
                                                        .text_color(theme.text_muted)
                                                        .child(format!("Reply to {}", author_handle)),
                                                )
                                                .child(
                                                    Button::new("btn-quick-reply-thread")
                                                        .ghost()
                                                        .compact()
                                                        .icon(Icon::new(LucideIcon::MessageSquare))
                                                        .label(t!("feed.quote_or_reply").to_string())
                                                        .on_click(move |_, window, cx| {
                                                            cb_reply(root_id.clone(), &author_handle, window, cx);
                                                        }),
                                                ),
                                        )
                                    }),
                            ),
                    )
            )
        })
        .when(state.active_tab == NavigationTab::Knowledge, |this| {
            this.child(render_knowledge_vault_view(
                &state.documents,
                theme,
                params.doc_title_input,
                params.doc_content_input,
                params.on_add_document,
                params.on_delete_document,
            ))
        })
        .when(state.active_tab == NavigationTab::AgentLab, |this| {
            this.child(render_agent_lab_view(
                &state.user_profiles,
                theme,
                params.on_trigger_simulation,
            ))
        })
        .when(state.active_tab == NavigationTab::MindGraph, |this| {
            this.child(render_mind_graph_view(
                &state.posts,
                &state.documents,
                theme,
            ))
        })
}
