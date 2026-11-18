//! The task text of every action (prompts are written in English; the answer language is set
//! in the system prompt).

use super::super::context::ActionContext;
use super::super::kind::ActionKind;
use super::super::request::ActionRequest;

const SUMMARISE: &str = "Summarise the whole thread (the posts above the thought, the thought and its replies). \
Sections: ## Gist (two or three sentences), ## Positions (\"- @handle: position\" for each voice), ## Open questions (list).";
const MAP_CONCEPTS: &str = "Map the key concepts of the thought and how they relate. Sections: ## Concepts (\"- concept: one-line gloss\", at most eight), \
## Relations (one per line, exactly \"- Concept A -> relation -> Concept B\", using the concept names above).";
const STEELMAN: &str = "Steelman the thought: build the strongest, most charitable version of its argument. \
Sections: ## Strongest version (a tight paragraph), ## Why it holds (list of the best reasons and evidence), ## Hardest objection answered.";
const STRAWMAN: &str = "Check whether the thought misrepresents an opposing view (a strawman). \
For each distortion: \"- The view: … — How it is distorted: … — The fair version: …\". \
If the thought treats opposing views fairly, say so in one sentence and name the view it engages best.";
const HIDDEN_PREMISE: &str = "Reconstruct the argument and find its hidden premises. Sections: ## Conclusion (one sentence), \
## Stated premises (list), ## Hidden premises (list; each with why the conclusion needs it), ## Most fragile premise (one sentence).";
const COUNTEREXAMPLE: &str = "Find two to four concrete counterexamples to the thought's central claim. \
For each: \"- Case: … — Why it breaks the claim: …\". End with ## Revised claim: the strongest version that survives them.";
const BIASES: &str = "Identify cognitive biases or heuristics that may be shaping the reasoning (for example confirmation bias, \
availability, anchoring, survivorship, motivated reasoning). Only name biases with a cue in the text. \
For each: \"- Bias: the cue in the text — a debiasing question\". If none is clear, say so.";
const CONFIDENCE: &str = "List the distinct claims of the thought and rate the confidence a careful, well-informed reasoner should hold in each. \
One line per claim, exactly \"- claim — NN% — the main reason\". End with one line exactly \"Overall: NN%\".";
const CRUX: &str = "Find the crux of the disagreement between the thought and its replies (if there are no replies, between the thought \
and its strongest opposing view): the single factual or value question whose answer would change someone's mind. \
Sections: ## Crux (one question), ## What each side bets (list), ## Evidence that would settle it (list).";
const SOCRATIC: &str = "Ask six Socratic questions about the thought, one each probing: clarification, assumptions, evidence, \
alternative viewpoints, implications, and the question itself. List them as \"- question\" with no answers.";
const ANALOGIES: &str = "Offer analogies for the thought's core idea from five different domains: nature, engineering, everyday life, \
history or society, and art or music. One per line: \"- Domain: the analogy — where it breaks down\".";
const FALSIFIABLE: &str = "Turn the thought's central claim into falsifiable predictions. Sections: ## Predictions (list; each says what we \
would observe, where and by when), ## What would prove it wrong (list), ## Cheapest test (one paragraph).";
const CONTINUE: &str = "Continue the thought one step further in the author's own voice and register: one or two short paragraphs \
that advance the idea without repeating it. Output only the continuation.";

/// The task of `request` (the thread follows it in the prompt).
pub(super) fn task_text(request: &ActionRequest, context: &ActionContext) -> String {
    match request.kind {
        ActionKind::SearchConcept => concept_task(&request.concept),
        ActionKind::Explain => explain_task(&request.audience.describe()),
        ActionKind::Translate => translate_task(&request.language.english_name()),
        ActionKind::SummariseThread if !context.has_thread() => SUMMARISE_SINGLE.to_string(),
        kind => fixed_task(kind).to_string(),
    }
}

const SUMMARISE_SINGLE: &str = "The thought has no replies yet. Summarise it: ## Gist (two sentences), ## Key claims (list), ## Open questions (list).";

const MAP_ARGUMENT: &str = "Map the argument of the thread with Toulmin's roles. Sections: ## Claims (list), ## Grounds (\"- ground -> the claim it supports\"), ## Warrants (list), ## Objections (\"- objection -> the claim it attacks\").";
const FIND_TENSIONS: &str = "Find statements in the thread that are in tension with the thought. For each: \"- The statement: … — Why it conflicts: … — How both could be true: …\". If nothing conflicts, say so in one sentence.";
const CHECK_ATOMICITY: &str = "Check whether the thought is an atomic note: one claim, with enough context to stand alone. Sections: ## Claims (each distinct claim), ## Missing context (list), ## Atomic versions (one short post per claim).";

/// The task of an action that takes no choice.
fn fixed_task(kind: ActionKind) -> &'static str {
    match kind {
        ActionKind::SummariseThread => SUMMARISE,
        ActionKind::MapConcepts => MAP_CONCEPTS,
        ActionKind::Steelman => STEELMAN,
        ActionKind::Strawman => STRAWMAN,
        ActionKind::HiddenPremise => HIDDEN_PREMISE,
        ActionKind::Counterexample => COUNTEREXAMPLE,
        ActionKind::CognitiveBiases => BIASES,
        ActionKind::ClaimConfidence => CONFIDENCE,
        ActionKind::Crux => CRUX,
        ActionKind::Socratic => SOCRATIC,
        ActionKind::Analogies => ANALOGIES,
        ActionKind::FalsifiablePrediction => FALSIFIABLE,
        ActionKind::ContinueThought => CONTINUE,
        ActionKind::SearchConcept | ActionKind::Explain | ActionKind::Translate => "",
        // sb E1: thinking tools open their own views; these tasks are the plain-text fallback.
        ActionKind::MapArgument => MAP_ARGUMENT,
        ActionKind::FindTensions => FIND_TENSIONS,
        ActionKind::CheckAtomicity => CHECK_ATOMICITY,
    }
}

fn concept_task(concept: &str) -> String {
    format!(
        "Define the concept \"{concept}\" as it is used in the thought. Sections: ## Definition (two or three sentences), \
         ## In this thought (how the author uses it), ## Related concepts (three to five, \"- concept: how it relates\")."
    )
}

fn explain_task(audience: &str) -> String {
    format!(
        "Explain the thought for this reader: {audience}. Keep its meaning and claims intact; adapt vocabulary, \
         examples and depth to the reader. Use short paragraphs, and a list only where it helps."
    )
}

fn translate_task(language: &str) -> String {
    format!(
        "Translate the thought into {language}. Keep its tone and line breaks. Keep [[wikilinks]], #hashtags, @mentions \
         and URLs exactly as they are. Output only the translation."
    )
}
