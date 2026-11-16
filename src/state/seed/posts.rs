//! Seed timeline posts.

use super::parse_seed;
use crate::domain::{NetworkId, Post, PostId, Profile};
use chrono::{DateTime, Utc};
use serde::Deserialize;

const POSTS_ASSET: &str = "posts.json";
const POSTS_JSON: &str = include_str!("../../../assets/seed/posts.json");

/// Author reference; resolved against the supplied profiles, otherwise rebuilt from these fields
/// (an agent when `persona_type` is present, scoped to the post's network; else a primary user).
#[derive(Deserialize)]
struct AuthorSeed {
    id: String,
    name: String,
    handle: String,
    #[serde(default)]
    persona_type: Option<String>,
}

/// Post record without a timestamp; `created_at` is stamped at seed time.
#[derive(Deserialize)]
struct PostSeed {
    id: PostId,
    network_id: NetworkId,
    author: AuthorSeed,
    content: String,
    likes: usize,
    is_liked: bool,
    replies_count: usize,
    mentions: Vec<String>,
    linked_posts: Vec<PostId>,
    reply_to_id: Option<PostId>,
    quote_id: Option<PostId>,
    repost_id: Option<PostId>,
    link_url: Option<String>,
    image_path: Option<String>,
    reposts_count: usize,
    is_reposted: bool,
}

impl AuthorSeed {
    fn resolve(self, profiles: &[Profile], network_id: &NetworkId) -> Profile {
        if let Some(found) = profiles.iter().find(|p| p.id.as_str() == self.id) {
            return found.clone();
        }
        match self.persona_type {
            Some(persona_type) => Profile::new_agent(self.id, self.name, self.handle, persona_type)
                .with_network(network_id.clone()),
            None => Profile::new_primary(self.id, self.name, self.handle),
        }
    }
}

impl PostSeed {
    fn into_post(self, profiles: &[Profile], created_at: DateTime<Utc>) -> Post {
        let author = self.author.resolve(profiles, &self.network_id);
        Post {
            id: self.id,
            network_id: self.network_id,
            author,
            content: self.content,
            created_at,
            likes: self.likes,
            is_liked: self.is_liked,
            replies_count: self.replies_count,
            mentions: self.mentions,
            linked_posts: self.linked_posts,
            reply_to_id: self.reply_to_id,
            quote_id: self.quote_id,
            repost_id: self.repost_id,
            link_url: self.link_url,
            image_path: self.image_path,
            reposts_count: self.reposts_count,
            is_reposted: self.is_reposted,
        }
    }
}

pub fn seed_posts(profiles: &[Profile]) -> Vec<Post> {
    let now = Utc::now();
    let seeds: Vec<PostSeed> = parse_seed(POSTS_ASSET, POSTS_JSON);
    seeds
        .into_iter()
        .map(|seed| seed.into_post(profiles, now))
        .collect()
}
