//! Editing a post's text: the body is replaced and what is derived from it (mentions and
//! linked posts) follows; the reply, quote and repost links stay.

use crate::domain::Post;

/// `post` with `content` (trimmed) as its text.
pub fn edited_post(post: &Post, content: &str) -> Post {
    let content = content.trim().to_string();
    let referenced = [&post.quote_id, &post.reply_to_id, &post.repost_id];
    let mut linked_posts = Post::parse_linked_posts(&content);
    for id in referenced.into_iter().flatten() {
        if !linked_posts.contains(id) {
            linked_posts.push(id.clone());
        }
    }
    Post { mentions: Post::parse_mentions(&content), linked_posts, content, ..post.clone() }
}
