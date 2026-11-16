//! Restyling passes over a list of sentences (no terminal punctuation until `punctuate`).

use crate::domain::{CapitalizationStyle, EmojiUsage, GrammarStyle, PunctuationHabit, SentenceLengthPreference};

const CONTRACTIONS: [(&str, &str); 6] = [
    ("I am", "I'm"), ("do not", "don't"), ("it is", "it's"),
    ("you are", "you're"), ("let us", "let's"), ("can not", "can't"),
];
const RELAXED_OPENER: &str = "honestly, ";
const COLLOQUIAL_FILLER: &str = "like, ";
const CLASSICAL_OPENER: &str = "thus, ";
const DROPPED_SUBJECTS: [&str; 4] = ["I ", "we ", "it ", "you "];
const LIST_JOIN: &str = ", ";
const FINAL_JOIN: &str = ", and ";
const SENTENCE_GAP: &str = " ";
const PERIOD: &str = ".";
const QUESTION: &str = "?";
const ELLIPSIS: &str = "...";
const SEMICOLON_JOIN: &str = "; ";
const DASH_JOIN: &str = " — ";
const SUBJECT_PRONOUN: &str = "I ";
const NO_EMOJI: &str = "";
const EMOJI_RARE: &str = " 🌿";
const EMOJI_MODERATE: &str = " 🤔✨";
const KAOMOJI: &str = " (・_・;)";

/// Shapes the register's three clauses into sentences of the preferred length.
pub(super) fn shape(clauses: [String; 3], length: SentenceLengthPreference) -> Vec<String> {
    use SentenceLengthPreference as L;
    let [first, second, third] = clauses;
    match length {
        L::UltraConciseAphorism => vec![first],
        L::ShortPunchy => vec![first, second, third],
        L::BalancedCadence => vec![first, second],
        L::LongPeriodic => vec![format!("{first}{LIST_JOIN}{second}{FINAL_JOIN}{third}")],
    }
}

fn with_opener(sentence: &mut String, opener: &str) {
    sentence.insert_str(0, opener);
}

/// Applies the grammar style to every sentence.
pub(super) fn grammar(sentences: &mut [String], style: GrammarStyle) {
    use GrammarStyle as G;
    let last = sentences.len().saturating_sub(1);
    match style {
        G::StandardFormal => {}
        G::RelaxedConversational => {
            for sentence in sentences.iter_mut() {
                *sentence = CONTRACTIONS.iter().fold(sentence.clone(), |s, (long, short)| s.replace(long, short));
            }
            sentences.first_mut().into_iter().for_each(|s| with_opener(s, RELAXED_OPENER));
        }
        G::FragmentedStream => sentences.iter_mut().for_each(|s| {
            if let Some(rest) = DROPPED_SUBJECTS.iter().find_map(|p| s.strip_prefix(p)) {
                *s = rest.to_string();
            }
        }),
        G::PoeticClassical => sentences.get_mut(last).into_iter().for_each(|s| with_opener(s, CLASSICAL_OPENER)),
        G::MinimalistLowercase => sentences.iter_mut().for_each(|s| *s = s.to_lowercase()),
        G::ColloquialExpressive => sentences.get_mut(last.min(1)).into_iter().for_each(|s| with_opener(s, COLLOQUIAL_FILLER)),
    }
}

fn capitalize_first(sentence: &str) -> String {
    let mut chars = sentence.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

/// Applies the capitalization style; `emphasis` is shouted by the expressive style.
pub(super) fn capitalize(sentences: &mut [String], style: CapitalizationStyle, emphasis: &str) {
    for sentence in sentences.iter_mut() {
        *sentence = match style {
            CapitalizationStyle::StandardSentence => capitalize_first(sentence),
            CapitalizationStyle::AllLowercase => sentence.to_lowercase(),
            CapitalizationStyle::ExpressiveEmphasis => capitalize_first(&sentence.replace(emphasis, &emphasis.to_uppercase())),
        };
    }
}

/// Lower-cases a joined sentence's first letter unless it starts with the pronoun "I".
fn joined(sentence: &str) -> String {
    if sentence.starts_with(SUBJECT_PRONOUN) {
        return sentence.to_string();
    }
    let mut chars = sentence.chars();
    chars.next().map(|c| c.to_lowercase().chain(chars).collect()).unwrap_or_default()
}

/// Joins the sentences into one post using the punctuation habit.
pub(super) fn punctuate(sentences: &[String], habit: PunctuationHabit) -> String {
    use PunctuationHabit as P;
    let terminated = |end: &str| sentences.iter().map(|s| format!("{s}{end}")).collect::<Vec<_>>().join(SENTENCE_GAP);
    match habit {
        P::CrispMinimal => terminated(PERIOD),
        P::EllipsisContemplative => terminated(ELLIPSIS),
        P::ZeroTrailingPeriod => sentences.join(&format!("{PERIOD}{SENTENCE_GAP}")),
        P::QuestionDriven => format!("{}{QUESTION}", sentences.join(&format!("{PERIOD}{SENTENCE_GAP}"))),
        P::EmDashAndSemicolon => dash_join(sentences),
    }
}

/// `first; second — last.`: semicolons between clauses, an em dash before the last one.
fn dash_join(sentences: &[String]) -> String {
    let Some((last, rest)) = sentences.split_last() else { return String::new() };
    if rest.is_empty() {
        return format!("{last}{PERIOD}");
    }
    let head: Vec<String> = rest.iter().enumerate().map(|(i, s)| if i == 0 { s.clone() } else { joined(s) }).collect();
    format!("{}{DASH_JOIN}{}{PERIOD}", head.join(SEMICOLON_JOIN), joined(last))
}

/// The emoji (or kaomoji) tail for an emoji habit.
pub(super) const fn emoji_tail(usage: EmojiUsage) -> &'static str {
    match usage {
        EmojiUsage::Never => NO_EMOJI,
        EmojiUsage::RareSubtle => EMOJI_RARE,
        EmojiUsage::ModerateExpressive => EMOJI_MODERATE,
        EmojiUsage::KaomojiTextOnly => KAOMOJI,
    }
}
