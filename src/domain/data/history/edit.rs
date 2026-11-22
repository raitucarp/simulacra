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

/// What an edit starts from (V3, V4): the text and version edited, and every version the
/// post already has (an edit never takes one of their numbers).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EditBase {
    pub text: String,
    pub version: super::SemVer,
    /// The older version edited from (`None`: the version shown), V4.
    pub revision: Option<super::RevisionId>,
    pub taken: Vec<super::SemVer>,
    /// The symbols and tint of the version edited from, when it kept them (V10).
    pub record: Option<super::PostRecord>,
}

impl EditBase {
    /// The bump suggested for turning the base text into `draft`.
    pub fn suggested(&self, draft: &str) -> super::Bump {
        super::Bump::between(&self.text, draft)
    }

    /// The version an edit by `bump` makes.
    pub fn next(&self, bump: super::Bump) -> super::SemVer {
        super::free_version(self.version, bump, &self.taken)
    }
}
