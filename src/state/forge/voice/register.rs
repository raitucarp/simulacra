//! Raw sample clauses per diction register, and the topic they talk about.

use crate::domain::DictionRegister;

/// Placeholder replaced by the thinker's first preferred topic.
pub(super) const TOPIC_SLOT: &str = "{topic}";
/// Topic used when the thinker lists none.
pub(super) const FALLBACK_TOPIC: &str = "this idea";
const TOPIC_SIGIL: char = '#';
const TOPIC_SEPARATORS: [char; 2] = [',', ';'];
const WORD_GAP: char = ' ';

/// Three clauses (opening, middle, closing) and the word an emphatic writer would shout.
pub(super) struct RegisterSample {
    pub clauses: [&'static str; 3],
    pub emphasis: &'static str,
}

pub(super) const fn sample(register: DictionRegister) -> RegisterSample {
    use DictionRegister as D;
    let (clauses, emphasis) = match register {
        D::AcademicPhilosophical => (["I suspect our debate about {topic} rests on an unexamined premise", "we should first ask what the term actually picks out", "only then can the argument carry any weight"], "premise"),
        D::EverydayNatural => (["I keep thinking about {topic} today", "it is simpler than people make it sound", "maybe we just need to talk it through"], "simpler"),
        D::LiteraryPoetic => (["{topic} arrives like late light through a window", "it touches everything and holds nothing", "I want to stay with it a little longer"], "light"),
        D::TechnicalPrecision => (["the current model of {topic} fails under one specific condition", "the error compounds at the boundary", "we need a cleaner test before we conclude anything"], "condition"),
        D::SocraticIronic => (["I confess I do not understand {topic} at all", "perhaps you could explain what it means", "I am sure the answer is obvious to someone wiser"], "wiser"),
        D::WarmEmpathetic => (["I know {topic} can feel heavy right now", "you are not the only one carrying it", "let us take it one small step at a time"], "heavy"),
        D::SlangColloquial => (["ok so {topic} is lowkey wild", "nobody is talking about the real issue", "we gotta zoom out for a sec"], "wild"),
    };
    RegisterSample { clauses, emphasis }
}

/// `"PhilosophyOfScience"` → `"philosophy of science"`.
fn spaced_lowercase(word: &str) -> String {
    let mut out = String::new();
    for c in word.chars() {
        if c.is_uppercase() && !out.is_empty() {
            out.push(WORD_GAP);
        }
        out.extend(c.to_lowercase());
    }
    out
}

/// The first preferred topic in plain words (hashtags and CamelCase unpacked).
pub(super) fn first_topic(preferred_topics: &str) -> String {
    preferred_topics
        .split(|c: char| c.is_whitespace() || TOPIC_SEPARATORS.contains(&c))
        .map(|token| token.trim_matches(TOPIC_SIGIL))
        .find(|token| !token.is_empty())
        .map(spaced_lowercase)
        .unwrap_or_else(|| FALLBACK_TOPIC.to_string())
}
