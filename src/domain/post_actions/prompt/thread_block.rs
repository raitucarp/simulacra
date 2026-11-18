//! The thread as quoted data: the author, the posts above the thought, the thought itself and
//! its replies (indented by depth), each shortened so one long thread cannot crowd out the task.

use super::super::context::{ActionContext, ThreadLine};

/// Characters kept of each post around the thought (the thought itself is kept whole).
pub const MAX_LINE_CHARS: usize = 600;
/// Replies included below the thought.
pub const MAX_REPLIES: usize = 16;
const ELLIPSIS: char = '…';
const INDENT: &str = "  ";

/// `<thread>…</thread>` for `context`.
pub fn render_thread(context: &ActionContext) -> String {
    let mut parts = vec!["<thread>".to_string(), author_line(context)];
    if !context.ancestors.is_empty() {
        parts.push("Earlier in the thread (oldest first):".into());
        parts.extend(context.ancestors.iter().map(|line| bullet(line, MAX_LINE_CHARS)));
    }
    parts.push("The thought:".into());
    parts.push(context.thought.text.trim().to_string());
    if !context.replies.is_empty() {
        parts.push("Replies:".into());
        parts.extend(context.replies.iter().take(MAX_REPLIES).map(|line| bullet(line, MAX_LINE_CHARS)));
        let left_out = context.replies.len().saturating_sub(MAX_REPLIES);
        if left_out > 0 {
            parts.push(format!("({left_out} more replies not shown)"));
        }
    }
    parts.push("</thread>".into());
    parts.join("\n")
}

fn author_line(context: &ActionContext) -> String {
    let thought = &context.thought;
    let kind = if context.author.is_agent { "an autonomous agent" } else { "a person" };
    let mut line = format!("Author of the thought: {} (@{}), {kind}.", thought.author_name, thought.author_handle);
    let bio = context.author.bio.trim();
    if !bio.is_empty() {
        line.push_str(&format!(" About them: {}", shortened(bio, MAX_LINE_CHARS)));
    }
    line
}

/// `- @handle: text`, indented by reply depth.
fn bullet(line: &ThreadLine, limit: usize) -> String {
    let indent = INDENT.repeat(line.depth.saturating_sub(1));
    format!("{indent}- @{}: {}", line.author_handle, shortened(&single_line(&line.text), limit))
}

fn single_line(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// At most `limit` characters, marking a cut with an ellipsis.
fn shortened(text: &str, limit: usize) -> String {
    if text.chars().count() <= limit {
        return text.to_string();
    }
    let mut cut: String = text.chars().take(limit).collect();
    cut.push(ELLIPSIS);
    cut
}
