//! Threads-style single thought post item molecule with contemporary design system foundations.
//! Features tokenized typography hierarchy, continuous threadline, dialectic callouts,
//! human-readable wikilink excerpts, and click-to-focus thread inspection.

use crate::domain::{Post, ProfileRole};
use crate::theme::ThemeTokens;
use crate::ui::atoms::{render_avatar, render_hashtag_span, render_obsidian_pill, render_profile_avatar, render_role_badge};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, img, AnyElement, InteractiveElement, IntoElement, MouseButton, ObjectFit, ParentElement,
    Rgba, StatefulInteractiveElement, Styled, StyledImage, Window,
};
use std::path::PathBuf;

/// Trait abstracting post entity lookup and wikilink excerpt resolution.
/// Enables zero-cost polymorphism between raw borrowed slices (&[Post]) and indexed AppState.
pub trait PostLookup: Copy {
    fn find_post(&self, id: &str) -> Option<&Post>;
    fn resolve_wikilink(&self, target_id: &str) -> String;
}

impl PostLookup for &[Post] {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.iter().find(|p| p.id.as_str() == id)
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        resolve_wikilink_excerpt(target_id, self)
    }
}

impl<const N: usize> PostLookup for &[Post; N] {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.iter().find(|p| p.id.as_str() == id)
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        resolve_wikilink_excerpt(target_id, self.as_slice())
    }
}

impl PostLookup for &Vec<Post> {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.iter().find(|p| p.id.as_str() == id)
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        resolve_wikilink_excerpt(target_id, self.as_slice())
    }
}

impl PostLookup for &[&Post] {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.iter().find(|p| p.id.as_str() == id).copied()
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        let clean = target_id.trim();
        if clean.starts_with("post_") {
            if let Some(linked) = self.find_post(clean) {
                let words: Vec<&str> = linked
                    .content
                    .split_whitespace()
                    .filter(|w| !w.starts_with("[[") && !w.starts_with('#'))
                    .take(3)
                    .collect();
                if !words.is_empty() {
                    return format!("{}...", words.join(" "));
                }
            }
            let slug = clean.strip_prefix("post_").unwrap_or(clean);
            if slug.chars().count() > 6 {
                let short_slug: String = slug.chars().take(4).collect();
                return format!("#{}", short_slug);
            }
        }
        clean.to_string()
    }
}

impl PostLookup for &crate::state::AppState {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.get_post_by_id(id)
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        self.resolve_wikilink_fast(target_id)
    }
}

/// Resolves an Obsidian `[[post_id]]` into a human-readable 3-word excerpt of the referenced thought
/// or a clean short slug.
pub fn resolve_wikilink_excerpt(target_id: &str, all_posts: &[Post]) -> String {
    let clean = target_id.trim();
    if clean.starts_with("post_") {
        if let Some(linked) = all_posts.iter().find(|p| p.id.as_str() == clean) {
            let words: Vec<&str> = linked
                .content
                .split_whitespace()
                .filter(|w| !w.starts_with("[[") && !w.starts_with('#'))
                .take(3)
                .collect();
            if !words.is_empty() {
                return format!("{}...", words.join(" "));
            }
        }
        let slug = clean.strip_prefix("post_").unwrap_or(clean);
        let char_count = slug.chars().count();
        if char_count > 6 {
            let short_slug: String = slug.chars().take(4).collect();
            return format!("#{}", short_slug);
        }
    }
    clean.to_string()
}

/// Returns true if the character is an ASCII or CJK trailing punctuation mark.
#[inline]
pub(crate) fn is_trailing_punctuation(c: char) -> bool {
    matches!(
        c,
        ',' | '.'
            | '!'
            | '?'
            | ';'
            | ':'
            | '，' // U+FF0C fullwidth comma
            | '。' // U+3002 ideographic full stop
            | '！' // U+FF01 fullwidth exclamation
            | '？' // U+FF1F fullwidth question
            | '；' // U+FF1B fullwidth semicolon
            | '：' // U+FF1A fullwidth colon
            | '…'  // U+2026 horizontal ellipsis
    )
}

/// Parses post content into structured inline elements (tokenized words, obsidian links, hashtags)
/// with seamless word-level flex wrapping preventing text clipping at panel edges.
pub fn render_post_body<L: PostLookup>(content: &str, lookup: L, theme: &ThemeTokens) -> impl IntoElement {
    if !content.contains("[[") && !content.contains('#') && !content.contains('\n') {
        return div()
            .w_full()
            .font_family(theme.font_family_body.clone())
            .text_size(theme.font_size_md)
            .text_color(theme.text_primary)
            .child(content.to_string())
            .into_any_element();
    }

    let mut elements = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut current_text = String::new();

    let flush_words = |text: &str, elements: &mut Vec<AnyElement>| {
        if text.is_empty() {
            return;
        }
        let lines = text.split('\n');
        let mut first_line = true;
        for line in lines {
            let clean_line = line.trim_end_matches('\r');
            if !first_line {
                if clean_line.trim().is_empty() {
                    elements.push(div().w_full().h(theme.spacing_sm).into_any_element());
                } else {
                    elements.push(div().w_full().into_any_element());
                }
            }
            first_line = false;

            if !clean_line.is_empty() {
                elements.push(
                    div()
                        .font_family(theme.font_family_body.clone())
                        .text_size(theme.font_size_md)
                        .text_color(theme.text_primary)
                        .child(clean_line.to_string())
                        .into_any_element(),
                );
            }
        }
    };

    while i < len {
        // 1. Check for Obsidian Wikilink `[[...]]`
        if i + 1 < len && chars[i] == '[' && chars[i + 1] == '[' {
            let search_start = i + 2;
            let mut close_idx = None;
            let mut j = search_start;
            while j + 1 < len {
                if chars[j] == ']' && chars[j + 1] == ']' {
                    close_idx = Some(j);
                    break;
                }
                j += 1;
            }

            if let Some(end_bracket) = close_idx {
                flush_words(&current_text, &mut elements);
                current_text.clear();

                let inner: String = chars[search_start..end_bracket].iter().collect();
                let display_title = lookup.resolve_wikilink(inner.trim());
                let mut cursor = end_bracket + 2;
                let mut trailing_punct = String::new();
                while cursor < len && is_trailing_punctuation(chars[cursor]) {
                    trailing_punct.push(chars[cursor]);
                    cursor += 1;
                }

                if trailing_punct.is_empty() {
                    elements.push(render_obsidian_pill(&display_title, theme).into_any_element());
                } else {
                    elements.push(
                        div()
                            .flex()
                            .items_center()
                            .gap_0()
                            .child(render_obsidian_pill(&display_title, theme))
                            .child(
                                div()
                                    .font_family(theme.font_family_body.clone())
                                    .text_size(theme.font_size_md)
                                    .text_color(theme.text_primary)
                                    .child(trailing_punct),
                            )
                            .into_any_element(),
                    );
                }

                i = cursor;
                continue;
            }
        }

        // 2. Check for Hashtag `#Tag`
        if chars[i] == '#' && (i == 0 || chars[i - 1].is_whitespace()) {
            let mut end = i + 1;
            while end < len
                && !chars[end].is_whitespace()
                && chars[end] != '['
                && chars[end] != ']'
                && !is_trailing_punctuation(chars[end])
            {
                end += 1;
            }

            if end > i + 1 {
                flush_words(&current_text, &mut elements);
                current_text.clear();

                let tag_str: String = chars[i..end].iter().collect();
                let mut cursor = end;
                let mut trailing_punct = String::new();
                while cursor < len && is_trailing_punctuation(chars[cursor]) {
                    trailing_punct.push(chars[cursor]);
                    cursor += 1;
                }

                if trailing_punct.is_empty() {
                    elements.push(render_hashtag_span(&tag_str, theme).into_any_element());
                } else {
                    elements.push(
                        div()
                            .flex()
                            .items_center()
                            .gap_0()
                            .child(render_hashtag_span(&tag_str, theme))
                            .child(
                                div()
                                    .font_family(theme.font_family_body.clone())
                                    .text_size(theme.font_size_md)
                                    .text_color(theme.text_primary)
                                    .child(trailing_punct),
                            )
                            .into_any_element(),
                    );
                }

                i = cursor;
                continue;
            }
        }

        current_text.push(chars[i]);
        i += 1;
    }

    flush_words(&current_text, &mut elements);

    div()
        .w_full()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.spacing_xs)
        .gap_y(theme.spacing_2xs)
        .children(elements)
        .into_any_element()
}

pub type ActionClickCallback = Box<dyn Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static>;

/// Atomic Threads-style action button helper with tokenized typography and hover states.
fn render_action_btn(
    id: impl Into<gpui_kit::ElementId>,
    icon_name: LucideIcon,
    count: usize,
    is_active: bool,
    active_color: Rgba,
    theme: &ThemeTokens,
    on_click: Option<ActionClickCallback>,
) -> impl IntoElement {
    let base_color = if is_active {
        active_color
    } else {
        theme.text_muted
    };

    let hover_bg = theme.hover_bg;

    let mut btn = div()
        .id(id)
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(gpui_kit::FontWeight::MEDIUM)
        .text_color(base_color)
        .hover(move |s| s.bg(hover_bg).text_color(active_color))
        .child(Icon::new(icon_name).size(theme.font_size_md));

    if count > 0 {
        btn = btn.child(count.to_string());
    }

    if let Some(cb) = on_click {
        btn.on_click(move |e, w, cx| cb(e, w, cx))
    } else {
        btn
    }
}

/// Context-aware dialectic threadline rendering style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThreadlineStyle {
    /// Standalone thought with no threadline connections.
    #[default]
    None,
    /// Root/parent thought in a thread; line extends from below avatar down to bottom of card.
    ParentHasReplies,
    /// Reply in middle of a thread; continuous vertical line entering from top, passing avatar, and extending to bottom.
    MiddleReply,
    /// Final reply in a thread; line enters from top of card and terminates at the avatar.
    LastReply,
}

impl ThreadlineStyle {
    #[inline]
    pub fn has_top_line(&self) -> bool {
        matches!(self, Self::MiddleReply | Self::LastReply)
    }

    #[inline]
    pub fn has_bottom_line(&self) -> bool {
        matches!(self, Self::ParentHasReplies | Self::MiddleReply)
    }
}

/// Extracts a clean host domain slug from an arbitrary URL.
pub fn extract_host_domain(url: &str) -> String {
    let trimmed = url.trim();
    let lower = trimmed.to_ascii_lowercase();
    let without_scheme = if let Some(stripped) = lower.strip_prefix("https://") {
        stripped
    } else if let Some(stripped) = lower.strip_prefix("http://") {
        stripped
    } else {
        &lower
    };

    let host_part = without_scheme
        .split(['/', '?', '#'])
        .next()
        .unwrap_or(without_scheme);

    host_part.strip_prefix("www.").unwrap_or(host_part).to_string()
}

pub type PostSelectCallback = Box<dyn Fn(&mut Window, &mut gpui_kit::App) + 'static>;

/// Comprehensive Threads-style Post Molecule with tokenized layout, context-aware threadline,
/// attachment embeds (image & link), quote cards, repost header, and click-to-focus inspection.
#[allow(clippy::too_many_arguments)]
pub fn render_post_item<L: PostLookup>(
    post: &Post,
    lookup: L,
    theme: &ThemeTokens,
    is_focused: bool,
    threadline_style: ThreadlineStyle,
    on_select: impl Fn(&mut Window, &mut gpui_kit::App) + 'static,
    on_like: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
    on_reply: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
    on_repost: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
    on_quote: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
) -> impl IntoElement {
    // Resolve instant repost target if post content is empty
    let (effective_post, is_instant_repost) = if post.repost_id.is_some() && post.content.trim().is_empty() {
        let rep_id = post.repost_id.as_deref().unwrap_or("");
        if let Some(target) = lookup.find_post(rep_id) {
            (target, true)
        } else {
            (post, false)
        }
    } else {
        (post, false)
    };

    let time_str = effective_post.created_at.format("%H:%M").to_string();
    let is_liked = effective_post.is_liked;
    let is_agent = matches!(effective_post.author.role, ProfileRole::Agent { .. });

    let like_color = if is_liked {
        theme.accent_brand
    } else {
        theme.text_secondary
    };

    let repost_color = if effective_post.is_reposted {
        theme.accent_success
    } else {
        theme.text_secondary
    };

    let bg_color = if is_focused {
        theme.bg_elevated
    } else {
        theme.bg_feed_post
    };

    let hover_bg = if is_focused {
        theme.bg_elevated
    } else {
        theme.hover_bg
    };

    let on_select_rc = std::rc::Rc::new(on_select);
    let on_select_click = on_select_rc.clone();

    // Top & bottom line colors for context-aware threadline
    let top_line_color = if threadline_style.has_top_line() {
        theme.border_subtle
    } else {
        theme.bg_transparent
    };

    let bottom_line_color = if threadline_style.has_bottom_line() {
        theme.border_subtle
    } else {
        theme.bg_transparent
    };

    let has_repost_header = post.repost_id.is_some();
    let reposter_name = post.author.name.clone();

    div()
        .id(format!("post-item-{}", post.id))
        .w_full()
        .px(theme.spacing_lg)
        .bg(bg_color)
        .hover(move |s| s.bg(hover_bg))
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_default()
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            (on_select_click)(window, cx);
        })
        .flex()
        .items_start()
        .gap(theme.spacing_md)
        // LEFT COLUMN: Avatar + Continuous Threadline (Zero-gap card boundary connection)
        .child(
            div()
                .flex_shrink_0()
                .w(theme.post_avatar_col_width)
                .h_full()
                .flex()
                .flex_col()
                .items_center()
                // Top line segment entering from card top boundary
                .child(
                    div()
                        .w(theme.border_width_threadline)
                        .h(if has_repost_header {
                            theme.spacing_md + theme.font_size_xs + theme.spacing_2xs
                        } else {
                            theme.spacing_md
                        })
                        .bg(top_line_color),
                )
                // Author Avatar
                .child(render_profile_avatar(
                    &effective_post.author,
                    theme.avatar_size_lg,
                    theme,
                ))
                // Bottom line segment extending to card bottom boundary
                .child(
                    div()
                        .w(theme.border_width_threadline)
                        .flex_1()
                        .min_h(theme.threadline_min_height)
                        .mt(theme.spacing_xs)
                        .bg(bottom_line_color),
                ),
        )
        // RIGHT COLUMN: Repost Header, Author Info, Body, Attachments, Quotes, Action Bar
        .child(
            div()
                .flex_1()
                .min_w_0()
                .py(theme.spacing_md)
                .flex()
                .flex_col()
                .gap(theme.spacing_xs)
                // 1. Repost Header
                .when(has_repost_header, |this| {
                    this.child(
                        div()
                            .flex()
                            .items_center()
                            .gap(theme.spacing_xs)
                            .pb(theme.spacing_2xs)
                            .child(
                                div()
                                    .w(theme.font_size_xs)
                                    .h(theme.font_size_xs)
                                    .text_color(theme.text_muted)
                                    .child(Icon::new(LucideIcon::Repeat)),
                            )
                            .child(
                                div()
                                    .font_family(theme.font_family_ui.clone())
                                    .text_size(theme.font_size_xs)
                                    .font_weight(gpui_kit::FontWeight::MEDIUM)
                                    .text_color(theme.text_muted)
                                    .child(format!("{} reposted", reposter_name)),
                            ),
                    )
                })
                // Top Row: Author Display Name + Handle + Role Badge + Grounding Pill + Timestamp
                .child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .flex_wrap()
                                .gap(theme.spacing_sm)
                                // Author Display Name (prominent, crisp white)
                                .child(
                                    div()
                                        .font_family(theme.font_family_display.clone())
                                        .text_size(theme.font_size_sm)
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.text_primary)
                                        .child(effective_post.author.name.clone()),
                                )
                                // Author Handle (secondary muted)
                                .child(
                                    div()
                                        .font_family(theme.font_family_ui.clone())
                                        .text_size(theme.font_size_xs)
                                        .font_weight(gpui_kit::FontWeight::NORMAL)
                                        .text_color(theme.text_muted)
                                        .child(effective_post.author.handle.clone()),
                                )
                                // Agent Role Badge
                                .when(is_agent, |this| {
                                    this.child(render_role_badge(&effective_post.author.role, theme))
                                })
                                // Subtle Grounding Pill
                                .when(is_agent, |this| {
                                    this.child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(theme.spacing_2xs)
                                            .px(theme.spacing_xs)
                                            .py(theme.spacing_2xs)
                                            .rounded(theme.radius_sm)
                                            .bg(theme.hover_bg)
                                            .border(theme.border_width_hairline)
                                            .border_color(theme.border_subtle)
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_2xs)
                                            .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                            .text_color(theme.accent_dialectic)
                                            .child(Icon::new(LucideIcon::Library).size(theme.font_size_xs))
                                            .child("Grounding 94%"),
                                    )
                                })
                                .child(
                                    div()
                                        .font_family(theme.font_family_mono.clone())
                                        .text_size(theme.font_size_xs)
                                        .text_color(theme.text_muted)
                                        .child(format!("· {}", time_str)),
                                )
                                .when(is_instant_repost, |this| {
                                    this.child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_2xs)
                                            .text_color(theme.accent_agent)
                                            .child("(original)"),
                                    )
                                }),
                        )
                        .child(
                            div()
                                .text_color(theme.text_muted)
                                .cursor_pointer()
                                .hover(move |s| s.text_color(theme.text_primary))
                                .child(Icon::new(LucideIcon::Ellipsis).size(theme.font_size_md)),
                        ),
                )
                // Post Body / Epistemic Dialectic Callout
                .child(
                    div()
                        .w_full()
                        .min_w_0()
                        .py(theme.spacing_2xs)
                        .child(render_post_body(&effective_post.content, lookup, theme)),
                )
                // 4. Local Image Attachment Embed (Zero synchronous filesystem syscalls)
                .when_some(effective_post.image_path.as_ref(), |this, img_path| {
                    let trimmed = img_path.trim();
                    if !trimmed.is_empty() {
                        this.child(
                            div()
                                .w_full()
                                .max_h(theme.post_image_max_height)
                                .rounded(theme.radius_md)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .overflow_hidden()
                                .child(
                                    img(PathBuf::from(trimmed))
                                        .w_full()
                                        .h_full()
                                        .object_fit(ObjectFit::Cover),
                                ),
                        )
                    } else {
                        this
                    }
                })
                // 5. Rich Link Preview Card
                .when_some(effective_post.link_url.as_ref(), |this, link_url| {
                    let domain = extract_host_domain(link_url);
                    let url_to_open = link_url.clone();
                    this.child(
                        div()
                            .id(format!("link-preview-{}", effective_post.id))
                            .w_full()
                            .p(theme.spacing_md)
                            .rounded(theme.radius_md)
                            .bg(theme.bg_card)
                            .border(theme.border_width_hairline)
                            .border_color(theme.border_subtle)
                            .hover(move |s| s.bg(theme.hover_bg).border_color(theme.border_strong))
                            .cursor_pointer()
                            .on_click(move |_ev, _window, cx| {
                                cx.open_url(&url_to_open);
                            })
                            .flex()
                            .flex_col()
                            .gap(theme.spacing_2xs)
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(theme.spacing_xs)
                                    .child(
                                        div()
                                            .w(theme.font_size_xs)
                                            .h(theme.font_size_xs)
                                            .text_color(theme.text_muted)
                                            .child(Icon::new(LucideIcon::ExternalLink)),
                                    )
                                    .child(
                                        div()
                                            .font_family(theme.font_family_mono.clone())
                                            .text_size(theme.font_size_xs)
                                            .text_color(theme.text_muted)
                                            .child(domain),
                                    ),
                            )
                            .child(
                                div()
                                    .font_family(theme.font_family_ui.clone())
                                    .text_size(theme.font_size_sm)
                                    .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                    .text_color(theme.text_primary)
                                    .child(link_url.clone()),
                            ),
                    )
                })
                // 6. Quoted Thought Card Embed
                .when_some(effective_post.quote_id.as_ref(), |this, quote_id| {
                    if let Some(quoted) = lookup.find_post(quote_id.as_str()) {
                        let quoted_time = quoted.created_at.format("%H:%M").to_string();
                        let quoted_is_agent = matches!(quoted.author.role, ProfileRole::Agent { .. });
                        this.child(
                            div()
                                .w_full()
                                .p(theme.spacing_md)
                                .rounded(theme.radius_md)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .hover(move |s| s.bg(theme.hover_bg).border_color(theme.border_strong))
                                .flex()
                                .flex_col()
                                .gap(theme.spacing_xs)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap(theme.spacing_sm)
                                        .child(render_avatar(
                                            &quoted.author.avatar_initial,
                                            &quoted.author.role,
                                            theme.avatar_size_sm,
                                            theme,
                                        ))
                                        .child(
                                            div()
                                                .font_family(theme.font_family_display.clone())
                                                .text_size(theme.font_size_xs)
                                                .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                .text_color(theme.text_primary)
                                                .child(quoted.author.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .font_family(theme.font_family_ui.clone())
                                                .text_size(theme.font_size_2xs)
                                                .text_color(theme.text_muted)
                                                .child(quoted.author.handle.clone()),
                                        )
                                        .when(quoted_is_agent, |this| {
                                            this.child(render_role_badge(&quoted.author.role, theme))
                                        })
                                        .child(
                                            div()
                                                .font_family(theme.font_family_mono.clone())
                                                .text_size(theme.font_size_2xs)
                                                .text_color(theme.text_muted)
                                                .child("·"),
                                        )
                                        .child(
                                            div()
                                                .font_family(theme.font_family_mono.clone())
                                                .text_size(theme.font_size_2xs)
                                                .text_color(theme.text_muted)
                                                .child(quoted_time),
                                        ),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_body.clone())
                                        .text_size(theme.font_size_xs)
                                        .text_color(theme.text_secondary)
                                        .child(render_post_body(&quoted.content, lookup, theme)),
                                ),
                        )
                    } else {
                        this.child(
                            div()
                                .w_full()
                                .p(theme.spacing_sm)
                                .rounded(theme.radius_md)
                                .bg(theme.bg_card)
                                .border(theme.border_width_hairline)
                                .border_color(theme.border_subtle)
                                .flex()
                                .items_center()
                                .gap(theme.spacing_xs)
                                .child(
                                    div()
                                        .w(theme.font_size_xs)
                                        .h(theme.font_size_xs)
                                        .text_color(theme.text_muted)
                                        .child(Icon::new(LucideIcon::Info)),
                                )
                                .child(
                                    div()
                                        .font_family(theme.font_family_mono.clone())
                                        .text_size(theme.font_size_xs)
                                        .text_color(theme.text_muted)
                                        .child(format!("Quoted thought [{}] unavailable", quote_id)),
                                ),
                        )
                    }
                })
                // 7. Full Threads Action Bar (Heart, Reply, Repost, Quote, Refs, Share)
                .child(
                    div()
                        .w_full()
                        .flex()
                        .items_center()
                        .gap(theme.spacing_lg)
                        .pt(theme.spacing_xs)
                        // Like Button
                        .child(render_action_btn(
                            format!("like-btn-{}", effective_post.id),
                            LucideIcon::Heart,
                            effective_post.likes,
                            is_liked,
                            like_color,
                            theme,
                            Some(Box::new(on_like)),
                        ))
                        // Reply Button -> opens ComposerModal in Reply mode
                        .child(render_action_btn(
                            format!("reply-btn-{}", effective_post.id),
                            LucideIcon::MessageSquare,
                            effective_post.replies_count,
                            false,
                            theme.accent_brand,
                            theme,
                            Some(Box::new(on_reply)),
                        ))
                        // Repost Button -> triggers instant repost
                        .child(render_action_btn(
                            format!("repost-btn-{}", effective_post.id),
                            LucideIcon::Repeat,
                            effective_post.reposts_count,
                            effective_post.is_reposted,
                            repost_color,
                            theme,
                            Some(Box::new(on_repost)),
                        ))
                        // Quote Button -> opens ComposerModal in Quote mode
                        .child(render_action_btn(
                            format!("quote-btn-{}", effective_post.id),
                            LucideIcon::Edit,
                            0,
                            false,
                            theme.accent_dialectic,
                            theme,
                            Some(Box::new(on_quote)),
                        ))
                        // Refs Button -> shows count of linked thoughts
                        .child(render_action_btn(
                            format!("refs-btn-{}", effective_post.id),
                            LucideIcon::GitBranch,
                            effective_post.linked_posts.len(),
                            !effective_post.linked_posts.is_empty(),
                            theme.accent_user,
                            theme,
                            None,
                        ))
                        // Share Button
                        .child(
                            div()
                                .id(format!("share-btn-{}", effective_post.id))
                                .flex()
                                .items_center()
                                .py(theme.spacing_2xs)
                                .cursor_pointer()
                                .text_color(theme.text_muted)
                                .hover(move |s| s.text_color(theme.text_primary))
                                .child(Icon::new(LucideIcon::Share2).size(theme.font_size_md)),
                        ),
                ),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::Profile;

    #[test]
    fn test_resolve_wikilink_excerpt_found() {
        let author = Profile::new_primary("u1", "Agent 1", "@agent1");
        let post = Post::create(
            "post_123456",
            "net1",
            author,
            "Epistemic dialectic simulation enables autonomous debate",
        )
        .unwrap();
        let posts = vec![post];

        let excerpt = resolve_wikilink_excerpt("post_123456", &posts);
        assert_eq!(excerpt, "Epistemic dialectic simulation...");
    }

    #[test]
    fn test_resolve_wikilink_excerpt_not_found_fallback() {
        let posts: Vec<Post> = Vec::new();
        let excerpt = resolve_wikilink_excerpt("post_abcdef123", &posts);
        assert_eq!(excerpt, "#abcd");
    }

    #[test]
    fn test_resolve_wikilink_excerpt_plain_text() {
        let posts: Vec<Post> = Vec::new();
        let excerpt = resolve_wikilink_excerpt("non_post_reference", &posts);
        assert_eq!(excerpt, "non_post_reference");
    }

    #[test]
    fn test_resolve_wikilink_excerpt_unicode_slugs() {
        let posts: Vec<Post> = Vec::new();

        // 1. Accented Latin characters (> 6 chars, non-char-boundary at byte 4)
        let excerpt_accent = resolve_wikilink_excerpt("post_café_latte", &posts);
        assert_eq!(excerpt_accent, "#café");

        let excerpt_uber = resolve_wikilink_excerpt("post_über_mensch", &posts);
        assert_eq!(excerpt_uber, "#über");

        // 2. CJK characters (3 bytes per character)
        // Exactly 6 chars: remains full string
        let excerpt_cjk_6 = resolve_wikilink_excerpt("post_你好世界测试", &posts);
        assert_eq!(excerpt_cjk_6, "post_你好世界测试");

        // 8 chars (> 6 chars): cleanly truncates to first 4 characters
        let excerpt_cjk_long = resolve_wikilink_excerpt("post_你好世界测试哲学", &posts);
        assert_eq!(excerpt_cjk_long, "#你好世界");

        // Japanese
        let excerpt_jp = resolve_wikilink_excerpt("post_東京_tokyo_spark", &posts);
        assert_eq!(excerpt_jp, "#東京_t");

        // Korean (5 chars <= 6)
        let excerpt_kr = resolve_wikilink_excerpt("post_소크拉테斯", &posts);
        assert_eq!(excerpt_kr, "post_소크拉테斯");

        // Cyrillic (2 bytes per character)
        let excerpt_ru = resolve_wikilink_excerpt("post_сократ_диалог", &posts);
        assert_eq!(excerpt_ru, "#сокр");

        // Greek (2 bytes per character)
        let excerpt_el = resolve_wikilink_excerpt("post_σωκράτης_νόμος", &posts);
        assert_eq!(excerpt_el, "#σωκρ");

        // Emoji with leading ASCII
        let excerpt_emoji = resolve_wikilink_excerpt("post_a🚀_spark_node", &posts);
        assert_eq!(excerpt_emoji, "#a🚀_s");
    }

    #[test]
    fn test_resolve_wikilink_excerpt_boundary_cases() {
        let posts: Vec<Post> = Vec::new();

        // Exactly 6 ASCII chars -> remains full
        assert_eq!(resolve_wikilink_excerpt("post_123456", &posts), "post_123456");

        // Exactly 7 ASCII chars -> truncates to first 4
        assert_eq!(resolve_wikilink_excerpt("post_1234567", &posts), "#1234");

        // Exactly 6 CJK chars -> remains full
        assert_eq!(resolve_wikilink_excerpt("post_一二三四五六", &posts), "post_一二三四五六");

        // Exactly 7 CJK chars -> truncates to first 4
        assert_eq!(resolve_wikilink_excerpt("post_一二三四五六七", &posts), "#一二三四");

        // Empty slug
        assert_eq!(resolve_wikilink_excerpt("post_", &posts), "post_");

        // Whitespace trimmed
        assert_eq!(resolve_wikilink_excerpt("  post_abcdefgh  ", &posts), "#abcd");
    }

    #[test]
    fn test_resolve_wikilink_excerpt_cjk_content() {
        let author = Profile::new_primary("u2", "Spinoza", "@spinoza");
        let post = Post::create(
            "post_cjk_1",
            "net1",
            author,
            "知之为知之不知为不知是知也 仁义礼智信",
        )
        .unwrap();
        let posts = vec![post];

        let excerpt = resolve_wikilink_excerpt("post_cjk_1", &posts);
        assert_eq!(excerpt, "知之为知之不知为不知是知也 仁义礼智信...");
    }

    #[test]
    fn test_render_post_body_trailing_punctuation_multibyte() {
        let theme = crate::theme::minimalist_dark();
        let posts = Vec::new();

        // CJK Ideographic period and fullwidth comma
        let text_cjk = "先读[[post_1001]]。然后看[[post_1002]]，再反思。";
        let _el1 = render_post_body(text_cjk, &posts, &theme);

        // Fullwidth exclamation and question marks
        let text_cjk_punct = "惊讶[[post_1001]]！质疑[[post_1002]]？";
        let _el2 = render_post_body(text_cjk_punct, &posts, &theme);

        // Mixed ASCII and multi-byte trailing punctuation
        let text_mixed = "Ref: [[post_1001]]... [[post_1002]]!? [[post_1003]]…";
        let _el3 = render_post_body(text_mixed, &posts, &theme);

        // CJK characters directly following wikilink without whitespace
        let text_direct_cjk = "根据[[post_1001]]的论断可知。";
        let _el4 = render_post_body(text_direct_cjk, &posts, &theme);

        // Hashtags with CJK punctuation
        let text_tags = "#哲学，#virtue。#epistemic！";
        let _el5 = render_post_body(text_tags, &posts, &theme);
    }

    #[test]
    fn test_render_post_body_long_continuous_strings() {
        let theme = crate::theme::minimalist_dark();
        let posts = Vec::new();

        // 1000-char continuous ASCII without spaces
        let long_ascii = "z".repeat(1000);
        let _el1 = render_post_body(&long_ascii, &posts, &theme);

        // 600-char continuous CJK without spaces
        let long_cjk = "道可道非常道名可名非常名".repeat(50);
        let _el2 = render_post_body(&long_cjk, &posts, &theme);

        // 3000-char continuous URL
        let long_url = format!("https://simulacra.vault/query?key={}", "payload_".repeat(300));
        let _el3 = render_post_body(&long_url, &posts, &theme);

        // Long emoji chain
        let long_emojis = "🧠💡⚡️".repeat(100);
        let _el4 = render_post_body(&long_emojis, &posts, &theme);
    }

    #[test]
    fn test_extract_host_domain_variations() {
        assert_eq!(extract_host_domain("https://crates.io/crates/gpui"), "crates.io");
        assert_eq!(extract_host_domain("http://www.nature.com/articles/s41586-020-2649-2"), "nature.com");
        assert_eq!(extract_host_domain("plato.stanford.edu/entries/ethics/"), "plato.stanford.edu");
        assert_eq!(extract_host_domain("https://sub.domain.org:8080/path?arg=1#hash"), "sub.domain.org:8080");
        assert_eq!(extract_host_domain("   https://example.com   "), "example.com");
        assert_eq!(extract_host_domain("HTTPS://PHILPAPERS.ORG/rec/xyz"), "philpapers.org");
        assert_eq!(extract_host_domain("HTTP://WWW.EXAMPLE.COM:8080/foo"), "example.com:8080");
    }

    #[test]
    fn test_threadline_style_booleans() {
        assert!(!ThreadlineStyle::None.has_top_line());
        assert!(!ThreadlineStyle::None.has_bottom_line());

        assert!(!ThreadlineStyle::ParentHasReplies.has_top_line());
        assert!(ThreadlineStyle::ParentHasReplies.has_bottom_line());

        assert!(ThreadlineStyle::MiddleReply.has_top_line());
        assert!(ThreadlineStyle::MiddleReply.has_bottom_line());

        assert!(ThreadlineStyle::LastReply.has_top_line());
        assert!(!ThreadlineStyle::LastReply.has_bottom_line());
    }
}

