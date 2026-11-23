//! Run on code blocks (P5): which blocks get it, what it sends, and when the user confirms.

use super::super::code_run::run_button;
use super::super::layout::layout;
use super::super::{needs_confirm, BodyLinks, CodeActions};
use crate::domain::code::Runtime;
use crate::domain::post::{tokenize_post_body, PostOrigin};
use crate::state::AppState;
use std::rc::Rc;

fn actions(runnable: &[Runtime]) -> CodeActions {
    CodeActions { runnable: runnable.into(), on_run: Rc::new(|_, _, _| {}), running: Rc::new([]), on_cancel: Rc::new(|_, _, _, _| {}) }
}

#[test]
fn only_blocks_in_a_runnable_runtime_get_run() {
    let code = actions(&[Runtime::Python]);
    assert_eq!(code.runtime_of(Some("py")), Some(Runtime::Python));
    assert_eq!(code.runtime_of(Some("js")), None, "node is not switched on or not found");
    assert_eq!(code.runtime_of(None), None, "plain text never runs");
    assert_eq!(code.runtime_of(Some("sh")), None);
}

#[test]
fn run_names_the_post_and_the_block() {
    let tokens = tokenize_post_body("first\n```js\nlet a = 1\n```\nthen\n```py\nprint(2)\n```");
    let items = layout(&tokens);
    let links = BodyLinks { code: Some(actions(&[Runtime::Python, Runtime::Node])), ..BodyLinks::default() }.scoped("post_7");
    let blocks: Vec<(usize, Option<&str>, &str)> = items.iter().enumerate().filter_map(|(at, item)| match item {
        super::super::layout::Item::Block { lang, code } => Some((at, *lang, *code)),
        _ => None,
    }).collect();
    let requests: Vec<_> = blocks.iter().filter_map(|(at, lang, code)| run_button(&items, *at, *lang, code, &links)).map(|button| button.request).collect();
    assert_eq!(requests.len(), 2);
    assert_eq!((requests[0].post.as_str(), requests[0].block, requests[0].runtime), ("post_7", 0, Runtime::Node));
    assert_eq!((requests[1].block, requests[1].runtime, requests[1].code.as_str()), (1, Runtime::Python, "print(2)"));
    assert!(run_button(&items, blocks[0].0, blocks[0].1, blocks[0].2, &BodyLinks::default()).is_none(), "no Run while code running is off");
}

#[test]
fn code_the_user_did_not_type_is_confirmed_first() {
    let mut state = AppState::new_in_memory();
    let mine = state.submit_post("```py\nprint(1)\n```").expect("posted");
    let post = state.get_post(&mine).expect("post").clone();
    let me = state.author_of(&post).clone();
    assert!(me.is_human_persona());
    assert!(!needs_confirm(&post, &me), "the user's own code runs at once");
    let imported = crate::domain::Post { extras: crate::domain::post::PostExtras { origin: Some(PostOrigin::Import), ..post.extras.clone() }, ..post.clone() };
    assert!(needs_confirm(&imported, &me), "an imported passage is not the user's code");
    let agent = state.user_profiles.iter().find(|profile| profile.is_agent()).expect("a seeded agent").clone();
    assert!(needs_confirm(&post, &agent), "an agent's code is always shown first");
}

// P6: the output panel

#[test]
fn the_panel_shows_its_blocks_saved_run_and_cancel_while_it_runs() {
    use super::super::run_panel::run_panel;
    use super::super::run_line;
    use crate::domain::code::{CodeRun, RunEnd};
    let mut state = AppState::new_in_memory();
    let id = state.submit_post("```py\nprint(1)\n```").expect("posted");
    let saved = CodeRun { block: 0, runtime: "python".into(), version: "3.12.2".into(), end: RunEnd::Exited, exit_code: Some(0), duration_ms: 250, output: "1\n".into(), ..CodeRun::default() };
    state.save_code_run(&id, saved.clone()).expect("saved");
    let tokens = tokenize_post_body(&state.get_post(&id).expect("post").content);
    let items = layout(&tokens);
    let at = items.iter().position(|item| matches!(item, super::super::layout::Item::Block { .. })).expect("a block");
    let links = BodyLinks::default().scoped(id.as_str());
    let panel = run_panel(&items, at, &&state, &links).expect("the saved run shows while code running is off");
    assert_eq!((panel.saved, panel.running.is_none()), (Some(saved.clone()), true));
    let busy = CodeActions { running: Rc::new([(id.as_str().to_string(), 0)]), ..actions(&[Runtime::Python]) };
    let running = BodyLinks { code: Some(busy), ..links.clone() };
    assert!(run_panel(&items, at, &&state, &running).expect("panel").running.is_some());
    assert!(run_button(&items, at, Some("py"), "print(1)", &running).is_none(), "a running block offers Cancel, not Run");
    let line = run_line(&saved);
    assert!(line.starts_with("Python 3.12.2 · ") && line.contains('0'), "{line}");
    let other = BodyLinks::default().scoped("another_post");
    assert!(run_panel(&items, at, &&state, &other).is_none());
}
