//! Reading the model's reply leniently and validating it: the JSON object may sit inside a
//! code fence or prose; ids may be numbers; roles and edge kinds may use common synonyms;
//! a post may be cited as `P2`, `2` or its id. Every kept node cites a post of the thread,
//! every kept edge joins two kept nodes; anything else is dropped and counted.

use super::super::edit::clean_text;
use super::super::model::{ArgEdge, ArgNode, ArgumentMap, EdgeKind, NodeRole, NODE_ID_PREFIX};
use super::prompt::{ThreadPost, MAX_NODES, POST_LABEL_PREFIX};
use crate::domain::PostId;
use serde_json::Value;
use std::collections::HashMap;

const ID_FIELDS: &[&str] = &["id", "key", "name"];
const ROLE_FIELDS: &[&str] = &["role", "type", "kind"];
const TEXT_FIELDS: &[&str] = &["text", "statement", "label", "content"];
const POST_FIELDS: &[&str] = &["post", "source", "post_id", "cites", "from_post"];
const FROM_FIELDS: &[&str] = &["from", "source", "src"];
const TO_FIELDS: &[&str] = &["to", "target", "dst"];
const KIND_FIELDS: &[&str] = &["kind", "type", "relation"];
const ROLE_SYNONYMS: &[(&str, NodeRole)] = &[
    ("conclusion", NodeRole::Claim), ("thesis", NodeRole::Claim), ("premise", NodeRole::Ground), ("grounds", NodeRole::Ground),
    ("data", NodeRole::Ground), ("evidence", NodeRole::Ground), ("reason", NodeRole::Ground), ("objection", NodeRole::Rebuttal),
    ("counterargument", NodeRole::Rebuttal), ("exception", NodeRole::Rebuttal), ("rule", NodeRole::Warrant), ("modality", NodeRole::Qualifier),
];
const SUPPORT_WORDS: &[&str] = &["support", "supports", "pro", "for", "backs", "justifies"];
const ATTACK_WORDS: &[&str] = &["attack", "attacks", "con", "against", "rebuts", "undercuts", "undermines", "objects"];

/// A validated map and what had to be dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Extraction {
    pub map: ArgumentMap,
    pub dropped_nodes: usize,
    pub dropped_edges: usize,
}

/// The map in `raw` for the thread `posts` (root first), or `None` when the reply holds no
/// readable object or no valid node.
pub fn parse_extraction(raw: &str, posts: &[ThreadPost]) -> Option<Extraction> {
    let root = posts.first()?.id.clone();
    let value = json_object(raw)?;
    let raw_nodes = value.get("nodes").and_then(Value::as_array).cloned().unwrap_or_default();
    let raw_edges = value.get("edges").and_then(Value::as_array).cloned().unwrap_or_default();
    let (nodes, ids) = read_nodes(&raw_nodes, posts);
    let edges = read_edges(&raw_edges, &ids);
    let (dropped_nodes, dropped_edges) = (raw_nodes.len() - nodes.len(), raw_edges.len() - edges.len());
    (!nodes.is_empty()).then(|| Extraction { map: ArgumentMap { root, nodes, edges }, dropped_nodes, dropped_edges })
}

/// The outermost `{…}` of the reply, parsed.
fn json_object(raw: &str) -> Option<Value> {
    let (start, end) = (raw.find('{')?, raw.rfind('}')?);
    (start < end).then(|| serde_json::from_str::<Value>(&raw[start..=end]).ok()).flatten().filter(Value::is_object)
}

fn field<'a>(value: &'a Value, names: &[&str]) -> Option<&'a Value> {
    names.iter().find_map(|name| value.get(*name))
}

/// A string or number field as text.
fn text_of(value: Option<&Value>) -> Option<String> {
    match value? {
        Value::String(text) => Some(text.trim().to_string()),
        Value::Number(number) => Some(number.to_string()),
        _ => None,
    }
}

/// Valid nodes renumbered `n1…`, and the reply's id of each mapped to its new id.
fn read_nodes(raw: &[Value], posts: &[ThreadPost]) -> (Vec<ArgNode>, HashMap<String, String>) {
    let mut ids = HashMap::new();
    let mut nodes = Vec::new();
    for value in raw {
        let reply_id = text_of(field(value, ID_FIELDS)).filter(|id| !id.is_empty());
        let role = text_of(field(value, ROLE_FIELDS)).and_then(|r| read_role(&r));
        let text = text_of(field(value, TEXT_FIELDS)).map(|t| clean_text(&t)).filter(|t| !t.is_empty());
        let source = text_of(field(value, POST_FIELDS)).and_then(|p| resolve_post(&p, posts));
        let (Some(role), Some(text), Some(source)) = (role, text, source) else { continue };
        if nodes.len() >= MAX_NODES || reply_id.as_ref().is_some_and(|id| ids.contains_key(id)) {
            continue;
        }
        let id = format!("{NODE_ID_PREFIX}{}", nodes.len() + 1);
        ids.insert(reply_id.unwrap_or_else(|| id.clone()), id.clone());
        nodes.push(ArgNode { id, role, text, source });
    }
    (nodes, ids)
}

fn read_edges(raw: &[Value], ids: &HashMap<String, String>) -> Vec<ArgEdge> {
    let mut edges: Vec<ArgEdge> = Vec::new();
    for value in raw {
        let end = |names: &[&str]| text_of(field(value, names)).and_then(|id| ids.get(&id).cloned());
        let kind = text_of(field(value, KIND_FIELDS)).and_then(|k| read_kind(&k));
        let (Some(from), Some(to), Some(kind)) = (end(FROM_FIELDS), end(TO_FIELDS), kind) else { continue };
        if from != to && !edges.iter().any(|edge| edge.from == from && edge.to == to) {
            edges.push(ArgEdge { from, to, kind });
        }
    }
    edges
}

/// A role by its key or a common synonym.
pub fn read_role(word: &str) -> Option<NodeRole> {
    let word = word.trim().to_lowercase();
    NodeRole::from_key(&word).or_else(|| ROLE_SYNONYMS.iter().find(|(synonym, _)| *synonym == word).map(|(_, role)| *role))
}

/// An edge kind by its key or a common synonym.
pub fn read_kind(word: &str) -> Option<EdgeKind> {
    let word = word.trim().to_lowercase();
    if SUPPORT_WORDS.contains(&word.as_str()) {
        Some(EdgeKind::Support)
    } else if ATTACK_WORDS.contains(&word.as_str()) {
        Some(EdgeKind::Attack)
    } else {
        None
    }
}

/// The post a citation names: `P2` / `p2` / `2` (1-based) or a post id of the thread.
pub fn resolve_post(cited: &str, posts: &[ThreadPost]) -> Option<PostId> {
    let cited = cited.trim().trim_matches(['[', ']']);
    let digits = cited.strip_prefix(POST_LABEL_PREFIX).or_else(|| cited.strip_prefix(&POST_LABEL_PREFIX.to_lowercase())).unwrap_or(cited);
    let by_label = digits.parse::<usize>().ok().filter(|n| *n >= 1).and_then(|n| posts.get(n - 1));
    by_label.or_else(|| posts.iter().find(|post| post.id.as_str() == cited)).map(|post| post.id.clone())
}
