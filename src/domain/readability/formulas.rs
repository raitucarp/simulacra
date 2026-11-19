//! The published formulas. ASL is the mean sentence length in words, ASW the mean syllables
//! per word. Every Flesch-family score reads "higher is easier" on roughly 0–100.
//!
//! | Formula | Language | Expression | Source |
//! |---|---|---|---|
//! | Flesch reading ease | English | 206.835 − 1.015·ASL − 84.6·ASW | R. Flesch (1948), "A new readability yardstick", *Journal of Applied Psychology* 32(3), 221–233 |
//! | Flesch–Kincaid grade | English | 0.39·ASL + 11.8·ASW − 15.59 | J. P. Kincaid, R. P. Fishburne, R. L. Rogers, B. S. Chissom (1975), *Derivation of new readability formulas for Navy enlisted personnel*, Research Branch Report 8-75, Naval Technical Training Command |
//! | Amstad | German | 180 − ASL − 58.5·ASW | T. Amstad (1978), *Wie verständlich sind unsere Zeitungen?*, dissertation, Universität Zürich |
//! | Kandel–Moles | French | 207 − 1.015·ASL − 73.6·ASW | L. Kandel, A. Moles (1958), "Application de l'indice de Flesch à la langue française", *Cahiers d'études de radio-télévision* 19, 253–274 |
//! | Fernández Huerta | Spanish | 206.84 − 0.60·P − 1.02·F (P syllables and F sentences per 100 words) | J. Fernández Huerta (1959), "Medidas sencillas de lecturabilidad", *Consigna* 214, 29–32 |
//! | Flesch adapted to Portuguese | Portuguese | 248.835 − 1.015·ASL − 84.6·ASW | T. B. F. Martins, C. M. Ghiraldelo, M. G. V. Nunes, O. N. Oliveira Jr. (1996), *Readability formulas applied to textbooks in Brazilian Portuguese*, Notas do ICMC-USP 28 |
//! | Oborneva | Russian | 206.835 − 1.3·ASL − 60.1·ASW | I. V. Oborneva (2006), *Автоматизированная оценка сложности учебных текстов на основе статистических параметров*, PhD thesis, Moscow |
//! | LIX | any alphabetic language (the fallback) | ASL + 100·(long words / words), long = more than six letters | C. H. Björnsson (1968), *Läsbarhet*, Stockholm: Liber |
//! | Characters per sentence | Japanese, Chinese | CJK characters / sentences | a plain measure: these scripts have no syllable spelling to count |

use super::counts::TextCounts;
use super::syllables::SyllableRules;

/// Words per hundred, for Fernández Huerta's per-100-word rates.
const PER_HUNDRED: f64 = 100.0;

/// Which formula scores a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Formula {
    #[default]
    FleschEnglish,
    Amstad,
    KandelMoles,
    FernandezHuerta,
    FleschPortuguese,
    Oborneva,
    Lix,
    CjkSentenceLength,
}

impl Formula {
    pub const ALL: [Self; 8] = [Self::FleschEnglish, Self::Amstad, Self::KandelMoles, Self::FernandezHuerta, Self::FleschPortuguese, Self::Oborneva, Self::Lix, Self::CjkSentenceLength];

    /// Stable key (locale keys `insight.formula.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::FleschEnglish => "flesch",
            Self::Amstad => "amstad",
            Self::KandelMoles => "kandel_moles",
            Self::FernandezHuerta => "fernandez_huerta",
            Self::FleschPortuguese => "flesch_portuguese",
            Self::Oborneva => "oborneva",
            Self::Lix => "lix",
            Self::CjkSentenceLength => "cjk",
        }
    }

    /// The syllable rules its counts need.
    pub const fn syllable_rules(self) -> SyllableRules {
        match self {
            Self::FleschEnglish => SyllableRules::English,
            Self::Amstad => SyllableRules::German,
            Self::KandelMoles => SyllableRules::French,
            Self::FernandezHuerta => SyllableRules::Spanish,
            Self::FleschPortuguese => SyllableRules::Portuguese,
            Self::Oborneva => SyllableRules::Russian,
            Self::Lix | Self::CjkSentenceLength => SyllableRules::Generic,
        }
    }

    /// The formula's value for `counts`.
    pub fn value(self, counts: &TextCounts) -> f64 {
        let (asl, asw) = (counts.words_per_sentence(), counts.syllables_per_word());
        match self {
            Self::FleschEnglish => flesch_family(asl, asw, (206.835, 1.015, 84.6)),
            Self::Amstad => flesch_family(asl, asw, (180.0, 1.0, 58.5)),
            Self::KandelMoles => flesch_family(asl, asw, (207.0, 1.015, 73.6)),
            Self::FernandezHuerta => fernandez_huerta(asl, asw),
            Self::FleschPortuguese => flesch_family(asl, asw, (248.835, 1.015, 84.6)),
            Self::Oborneva => flesch_family(asl, asw, (206.835, 1.3, 60.1)),
            Self::Lix => lix(counts),
            Self::CjkSentenceLength => counts.cjk_chars_per_sentence(),
        }
    }
}

/// `base − sentence·ASL − word·ASW`: the shape every Flesch adaptation shares.
pub fn flesch_family(asl: f64, asw: f64, (base, sentence, word): (f64, f64, f64)) -> f64 {
    base - sentence * asl - word * asw
}

/// Fernández Huerta: 206.84 − 0.60·P − 1.02·F, with P syllables and F sentences per 100 words.
pub fn fernandez_huerta(asl: f64, asw: f64) -> f64 {
    let syllables_per_hundred = asw * PER_HUNDRED;
    let sentences_per_hundred = if asl > 0.0 { PER_HUNDRED / asl } else { 0.0 };
    206.84 - 0.60 * syllables_per_hundred - 1.02 * sentences_per_hundred
}

/// LIX: words per sentence plus the percentage of words longer than six letters.
pub fn lix(counts: &TextCounts) -> f64 {
    counts.words_per_sentence() + PER_HUNDRED * counts.long_word_share()
}

/// Flesch–Kincaid grade level (US school grade).
pub fn flesch_kincaid_grade(counts: &TextCounts) -> f64 {
    0.39 * counts.words_per_sentence() + 11.8 * counts.syllables_per_word() - 15.59
}
