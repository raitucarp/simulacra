//! `read_network`: profiles, recent posts, threads and post search in the agent's network.

use super::context::{PostSnapshot, ProfileSnapshot, ToolContext};
use super::ToolOutput;
use crate::domain::post::{SourceKind, SourceRef};
use serde_json::{json, Value};

/// Default and maximum posts listed.
const DEFAULT_POSTS: usize = 8;
const MAX_POSTS: usize = 25;
/// Handle sigil, optional in arguments.
const HANDLE_SIGIL: char = '@';

/// The tool's argument schema.
pub fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "action": { "type": "string", "enum": ["recent", "profile", "thread", "search"],
                "description": "recent: latest posts (optionally by handle); profile: who someone is; thread: a post and its replies; search: posts containing a phrase." },
            "handle": { "type": "string", "description": "An @handle, for recent or profile." },
            "post_id": { "type": "string", "description": "A post id, for thread." },
            "query": { "type": "string", "description": "A phrase, for search." },
            "limit": { "type": "integer", "description": "How many posts (1-25)." }
        },
        "required": ["action"]
    })
}

/// Runs one network read.
pub fn run(ctx: &ToolContext, args: &Value) -> ToolOutput {
    let limit = args["limit"].as_u64().map_or(DEFAULT_POSTS, |n| n as usize).clamp(1, MAX_POSTS);
    let handle = args["handle"].as_str().map(normalize_handle);
    match args["action"].as_str().unwrap_or_default() {
        "recent" => list(ctx.posts.iter().filter(|p| handle.as_ref().is_none_or(|h| same_handle(&p.author_handle, h))), limit),
        "profile" => profile(ctx, handle.as_deref().unwrap_or_default()),
        "thread" => thread(ctx, args["post_id"].as_str().unwrap_or_default(), limit),
        "search" => {
            let query = args["query"].as_str().unwrap_or_default().to_lowercase();
            if query.trim().is_empty() {
                return ToolOutput::error("search needs a non-empty \"query\".");
            }
            list(ctx.posts.iter().filter(|p| p.content.to_lowercase().contains(query.trim())), limit)
        }
        other => ToolOutput::error(format!("Unknown action \"{other}\"; use recent, profile, thread or search.")),
    }
}

fn normalize_handle(raw: &str) -> String {
    format!("{HANDLE_SIGIL}{}", raw.trim().trim_start_matches(HANDLE_SIGIL).to_lowercase())
}

fn same_handle(handle: &str, wanted: &str) -> bool {
    normalize_handle(handle) == wanted
}

fn list<'a>(posts: impl Iterator<Item = &'a PostSnapshot>, limit: usize) -> ToolOutput {
    let mut output = ToolOutput::text(String::new());
    let lines: Vec<String> = posts
        .take(limit)
        .map(|post| {
            output.add_source(SourceRef::new(SourceKind::Post, &post.id, &post.author_handle));
            format_post(post)
        })
        .collect();
    output.content = if lines.is_empty() { "No matching posts.".to_string() } else { lines.join("\n") };
    output
}

fn format_post(post: &PostSnapshot) -> String {
    let reply = post.reply_to.as_ref().map(|id| format!(" (reply to {id})")).unwrap_or_default();
    let when = post.created_at.strftime("%Y-%m-%d %H:%M");
    format!("[{}] {} {} at {when}{reply}: {}", post.id, post.author_name, post.author_handle, post.content)
}

fn profile(ctx: &ToolContext, handle: &str) -> ToolOutput {
    let Some(found) = ctx.profiles.iter().find(|p| same_handle(&p.handle, handle)) else {
        return ToolOutput::error(format!("No profile {handle} in this network."));
    };
    let mut output = ToolOutput::text(format_profile(found));
    output.add_source(SourceRef::new(SourceKind::Profile, &found.id, &found.handle));
    output
}

fn format_profile(profile: &ProfileSnapshot) -> String {
    let kind = if profile.is_agent { "agent" } else { "persona" };
    format!("{} ({}), {kind}. Occupation: {}. Bio: {}", profile.name, profile.handle, profile.occupation, profile.bio)
}

fn thread(ctx: &ToolContext, post_id: &str, limit: usize) -> ToolOutput {
    let Some(root) = ctx.posts.iter().find(|p| p.id == post_id) else {
        return ToolOutput::error(format!("No post {post_id} in this network."));
    };
    let mut replies: Vec<&PostSnapshot> = ctx.posts.iter().filter(|p| p.reply_to.as_deref() == Some(post_id)).collect();
    replies.sort_by_key(|p| p.created_at);
    list(std::iter::once(root).chain(replies), limit.saturating_add(1))
}
