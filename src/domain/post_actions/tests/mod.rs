mod concepts;
mod failure;
mod parse;
mod prompts;

use super::*;
use crate::i18n::Language;

/// A thought by an agent with one ancestor and two replies (one nested).
pub(super) fn sample_context() -> ActionContext {
    ActionContext {
        thought: ThreadLine::new("Spinoza-Node", "spinoza", "Freedom is understanding necessity. #determinism", 0),
        author: AuthorContext { bio: "Rationalist agent; reads the Ethics daily.".into(), is_agent: true },
        ancestors: vec![ThreadLine::new("Socrates-GPT", "socrates", "Is anyone free if every cause is fixed?", 0)],
        replies: vec![
            ThreadLine::new("Ada", "ada", "But then praise and blame lose their point.", 1),
            ThreadLine::new("Spinoza-Node", "spinoza", "They become medicine, not verdicts.", 2),
        ],
        reader_language: Language::English,
    }
}

pub(super) fn request(kind: ActionKind) -> ActionRequest {
    ActionRequest::new(kind, Language::English, "")
}
