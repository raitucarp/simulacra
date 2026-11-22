//! L13: agents read the files attached to a post as names and facts, never their contents:
//! in the reply prompt, in the network tools' view of posts and in a post action's thread.

use super::DialecticEngine;
use crate::agent::tools::ToolContext;
use crate::domain::media::files::with_files_note;
use crate::domain::post_actions::{build_prompt, ActionContext, ActionKind, ActionRequest, AuthorContext, ThreadLine};
use crate::domain::{AuthoredPost, Post, Profile};
use crate::i18n::Language;

/// Words that are only inside the attached file.
const SECRET: &str = "ORCHID-7741 the vault combination";

fn post_with_a_file() -> (Post, Profile) {
    let dir = std::env::temp_dir().join(format!("simulacra-agent-files-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("scratch dir");
    let path = dir.join("combination.txt");
    std::fs::write(&path, format!("{SECRET}\n")).expect("write");
    let ann = Profile::new_primary("u_ann", "Ann", "@ann");
    let mut post = Post::create("p1", "n1", ann.clone(), "Notes from the meeting").expect("post");
    post.extras.files.push(crate::media::files::read_file_ref(&path).expect("read"));
    (post, ann)
}

#[test]
fn agents_see_file_names_and_facts_but_never_contents() {
    let (post, ann) = post_with_a_file();
    let agent = Profile::new_agent("agent_rina", "Rina", "@rina", "Baker");
    let prompt = DialecticEngine::persona_user_prompt(&AuthoredPost::new(post.clone(), ann.clone()), &agent);
    assert!(prompt.contains("combination.txt") && prompt.contains("text/plain"), "{prompt}");
    assert!(prompt.contains("contents are not shared"), "{prompt}");
    let tools = ToolContext::capture(&post.network_id, std::slice::from_ref(&post), &[ann, agent], &[]);
    assert!(tools.posts[0].content.contains("combination.txt"));
    let line = ThreadLine::new("Ann", "@ann", &with_files_note(&post.content, &post.extras.files), 0);
    let context = ActionContext { thought: line, author: AuthorContext::default(), ancestors: Vec::new(), replies: Vec::new(), reader_language: Language::English };
    let action = build_prompt(&ActionRequest::new(ActionKind::Explain, Language::English, ""), &context);
    for (what, text) in [("reply prompt", prompt.as_str()), ("tool view", tools.posts[0].content.as_str()), ("post action", action.user.as_str())] {
        assert!(!text.contains(SECRET) && !text.contains("ORCHID"), "{what} leaked the file's contents");
        assert!(!text.contains(&post.extras.files[0].sha256), "{what} carries no hash");
    }
}

#[test]
fn a_post_without_files_reads_as_before() {
    let (mut post, _) = post_with_a_file();
    post.extras.files.clear();
    assert_eq!(with_files_note(&post.content, &post.extras.files), post.content);
}
