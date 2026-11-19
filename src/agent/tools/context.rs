//! The `Send` snapshot tools read on the generation thread: the agent's network posts,
//! profiles and vault documents, plus the web-fetch policy.

use crate::domain::{GroundingDocument, NetworkId, Post, Profile};
use jiff::Timestamp;
use std::collections::HashMap;
use std::time::Duration;

/// Most recent network posts captured for `read_network`.
pub const MAX_CONTEXT_POSTS: usize = 300;
/// Largest response body a web tool reads.
pub const DEFAULT_FETCH_MAX_BYTES: usize = 512 * 1024;
/// Per-request timeout of web tools.
pub const DEFAULT_FETCH_TIMEOUT: Duration = Duration::from_secs(12);

/// A post as tools see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostSnapshot {
    pub id: String,
    pub author_handle: String,
    pub author_name: String,
    pub content: String,
    pub created_at: Timestamp,
    pub reply_to: Option<String>,
}

/// A profile as tools see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSnapshot {
    pub id: String,
    pub name: String,
    pub handle: String,
    pub occupation: String,
    pub bio: String,
    pub is_agent: bool,
}

/// A vault document with its full text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSnapshot {
    pub id: String,
    pub title: String,
    pub content: String,
}

/// Limits of tools that reach the web.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FetchPolicy {
    /// Whether model-chosen URLs may target loopback / private / link-local hosts.
    pub allow_private_hosts: bool,
    pub max_bytes: usize,
    pub timeout: Duration,
}

impl Default for FetchPolicy {
    fn default() -> Self {
        Self { allow_private_hosts: false, max_bytes: DEFAULT_FETCH_MAX_BYTES, timeout: DEFAULT_FETCH_TIMEOUT }
    }
}

/// Everything tools may read, owned so it can move to the generation thread.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ToolContext {
    /// Newest first.
    pub posts: Vec<PostSnapshot>,
    pub profiles: Vec<ProfileSnapshot>,
    pub documents: Vec<DocumentSnapshot>,
    pub fetch: FetchPolicy,
}

impl ToolContext {
    /// Captures `network`'s posts (newest [`MAX_CONTEXT_POSTS`]), the profiles visible in it
    /// (members and global personas) and its documents.
    pub fn capture(network: &NetworkId, posts: &[Post], profiles: &[Profile], documents: &[GroundingDocument]) -> Self {
        let by_id: HashMap<&str, &Profile> = profiles.iter().map(|p| (p.id.as_str(), p)).collect();
        let mut network_posts: Vec<&Post> = posts.iter().filter(|p| &p.network_id == network).collect();
        network_posts.sort_by_key(|a| std::cmp::Reverse(a.created_at));
        network_posts.truncate(MAX_CONTEXT_POSTS);
        Self {
            posts: network_posts.into_iter().map(|post| post_snapshot(post, by_id.get(post.author_id.as_str()))).collect(),
            profiles: profiles
                .iter()
                .filter(|p| p.network_id.as_ref().is_none_or(|id| id == network))
                .map(profile_snapshot)
                .collect(),
            documents: documents
                .iter()
                .filter(|d| &d.network_id == network)
                .map(|d| DocumentSnapshot { id: d.id.clone(), title: d.title.clone(), content: d.content.clone() })
                .collect(),
            fetch: FetchPolicy::default(),
        }
    }
}

fn post_snapshot(post: &Post, author: Option<&&Profile>) -> PostSnapshot {
    PostSnapshot {
        id: post.id.as_str().to_string(),
        author_handle: author.map(|a| a.handle.clone()).unwrap_or_default(),
        author_name: author.map(|a| a.name.clone()).unwrap_or_default(),
        content: post.content.clone(),
        created_at: post.created_at,
        reply_to: post.reply_to_id.as_ref().map(|id| id.as_str().to_string()),
    }
}

fn profile_snapshot(profile: &Profile) -> ProfileSnapshot {
    ProfileSnapshot {
        id: profile.id.as_str().to_string(),
        name: profile.name.clone(),
        handle: profile.handle.clone(),
        occupation: profile.config.occupation.clone(),
        bio: profile.config.bio.clone(),
        is_agent: profile.is_agent(),
    }
}
