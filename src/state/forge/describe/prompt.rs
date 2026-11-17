//! The LLM instruction for turning a free-form description into a character sheet.

use super::{describable_scores, describable_sets, describable_traits, DESCRIBABLE_COUNTS, DESCRIBABLE_TEXT};
use crate::state::forge::fields::MAX_SCORE;

const INTRO: &str = "You are filling in a character sheet for a thoughtful member of a small online discussion network. Members are usually human, but may be any kind of being (a creature, a plant, an everyday object, an element, a machine, a place or an abstraction); if the description names one, set body_kind and form to match. Read the description below and reply with ONLY one JSON object: no prose, no comments, no code fences. Include only the keys you can reasonably infer and leave the others out.";
const TEXT_HEADING: &str = "String keys (plain text):";
const COUNT_HEADING: &str = "Whole-number keys:";
const SCORE_HEADING_PREFIX: &str = "Score keys (integers from 0 to ";
const SCORE_HEADING_SUFFIX: &str = "):";
const CHOICE_HEADING: &str = "Choice keys (use exactly one of the listed values, spelled exactly):";
const SET_HEADING: &str = "List keys (a JSON array of any of the listed values, spelled exactly):";
const DESCRIPTION_HEADING: &str = "Description:";
const FENCE: &str = "\"\"\"";
const BULLET: &str = "- ";
const CHOICE_SEPARATOR: &str = " | ";
const KEY_SEPARATOR: &str = ": ";
const LIST_SEPARATOR: &str = ", ";

fn bullet_list<'a>(keys: impl Iterator<Item = &'a str>) -> String {
    format!("{BULLET}{}", keys.collect::<Vec<_>>().join(LIST_SEPARATOR))
}

/// The full prompt for `description`: allowed keys, value types and every enum option.
pub fn describe_prompt(description: &str) -> String {
    let mut lines = vec![INTRO.to_string(), String::new(), TEXT_HEADING.to_string()];
    lines.push(bullet_list(DESCRIBABLE_TEXT.iter().map(|f| f.path())));
    lines.push(COUNT_HEADING.to_string());
    lines.push(bullet_list(DESCRIBABLE_COUNTS.iter().map(|f| f.path())));
    lines.push(format!("{SCORE_HEADING_PREFIX}{MAX_SCORE}{SCORE_HEADING_SUFFIX}"));
    lines.push(bullet_list(describable_scores().iter().map(|f| f.path())));
    lines.push(CHOICE_HEADING.to_string());
    for field in describable_traits() {
        let options = field.option_keys().join(CHOICE_SEPARATOR);
        lines.push(format!("{BULLET}{}{KEY_SEPARATOR}{options}", field.path()));
    }
    lines.push(SET_HEADING.to_string());
    for field in describable_sets() {
        let options = field.option_keys().join(CHOICE_SEPARATOR);
        lines.push(format!("{BULLET}{}{KEY_SEPARATOR}{options}", field.path()));
    }
    lines.extend([String::new(), DESCRIPTION_HEADING.to_string(), FENCE.to_string()]);
    lines.push(description.trim().to_string());
    lines.push(FENCE.to_string());
    lines.join("\n")
}
