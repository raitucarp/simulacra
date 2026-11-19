//! The polarity vocabulary of the tension finder, lower case: negation words, antonym pairs
//! and opposite quantifiers, plus the stopwords that never mark a shared topic. English
//! first, then small sets for Indonesian, German, French, Spanish and Portuguese.
//!
//! Antonyms are listed in their singular form (words are compared after a light plural
//! trim); quantifier poles may be phrases ("no one", "must not") and are compared as word
//! sequences. French "ne" is left out of the negations: "ne … pas" would count twice.

/// Words that negate the clause they stand in.
pub const NEGATIONS: &[&str] = &[
    "not", "no", "never", "none", "nobody", "nothing", "neither", "nor", "cannot", "without",
    "tidak", "bukan", "tak", "jangan", "belum", "tanpa",
    "nicht", "kein", "keine", "keinen", "keiner", "nie", "niemals", "nichts", "ohne",
    "pas", "jamais", "aucun", "aucune", "rien", "sans",
    "nunca", "jamás", "ningún", "ninguna", "nada", "sin",
    "não", "nenhum", "nenhuma", "sem",
];

/// The English contraction ending that negates ("don't", "isn't").
pub const NEGATION_SUFFIX: &str = "n't";

/// Words with opposite meanings (either order).
pub const ANTONYMS: &[(&str, &str)] = &[
    ("true", "false"), ("good", "bad"), ("increase", "decrease"), ("rise", "fall"), ("more", "less"),
    ("better", "worse"), ("possible", "impossible"), ("cause", "prevent"), ("help", "harm"),
    ("safe", "dangerous"), ("strong", "weak"), ("love", "hate"), ("agree", "disagree"), ("accept", "reject"),
    ("right", "wrong"), ("success", "failure"), ("win", "lose"), ("healthy", "unhealthy"), ("legal", "illegal"),
    ("rational", "irrational"), ("moral", "immoral"), ("necessary", "unnecessary"), ("high", "low"),
    ("simple", "complex"), ("certain", "uncertain"), ("objective", "subjective"), ("innate", "learned"),
    ("free", "determined"), ("benefit", "cost"), ("efficient", "inefficient"), ("support", "oppose"),
    ("benar", "salah"), ("baik", "buruk"), ("naik", "turun"), ("lebih", "kurang"), ("setuju", "menolak"),
    ("wahr", "falsch"), ("gut", "schlecht"), ("mehr", "weniger"), ("steigt", "sinkt"), ("richtig", "falsch"),
    ("vrai", "faux"), ("bon", "mauvais"), ("plus", "moins"), ("augmente", "diminue"),
    ("verdadero", "falso"), ("bueno", "malo"), ("más", "menos"), ("aumenta", "disminuye"),
    ("verdadeiro", "falso"), ("bom", "mau"), ("mais", "menos"), ("diminui", "aumenta"),
];

/// Opposite quantifiers and modalities (a pole may be a phrase).
pub const QUANTIFIERS: &[(&str, &str)] = &[
    ("all", "none"), ("always", "never"), ("everyone", "no one"), ("everyone", "nobody"),
    ("everything", "nothing"), ("must", "must not"), ("can", "cannot"), ("most", "few"), ("often", "rarely"),
    ("semua", "tidak ada"), ("selalu", "tidak pernah"), ("pasti", "mustahil"),
    ("alle", "keine"), ("immer", "nie"), ("alles", "nichts"), ("jeder", "niemand"),
    ("tous", "aucun"), ("toujours", "jamais"), ("tout", "rien"),
    ("todos", "ninguno"), ("siempre", "nunca"), ("todo", "nada"), ("todos", "nadie"),
    ("sempre", "nunca"), ("tudo", "nada"), ("todos", "ninguém"),
];

/// Frequent words that say nothing about a topic.
pub const STOPWORDS: &[&str] = &[
    "that", "this", "with", "from", "have", "been", "they", "their", "there", "which", "would", "could",
    "should", "about", "what", "when", "where", "because", "into", "than", "then", "them", "these", "those",
    "also", "just", "only", "very", "some", "such", "will", "were", "your", "does", "being", "here", "while",
    "other", "every", "each", "over", "after", "before", "even", "still", "much", "many", "make", "makes",
    "yang", "dengan", "untuk", "dari", "pada", "adalah", "akan", "juga", "atau", "karena", "bahwa", "dalam",
    "sebuah", "eine", "einen", "einer", "dass", "sind", "oder", "aber", "auch", "wird", "werden", "nach",
    "über", "noch", "wenn", "dans", "pour", "avec", "sont", "mais", "cette", "comme", "nous", "vous", "leur",
    "para", "como", "pero", "este", "esta", "porque", "sobre", "cuando", "entre", "isso", "quando", "mas",
];

/// Whether `word` (one word, lower case) carries polarity rather than topic.
pub fn is_polarity_word(word: &str) -> bool {
    NEGATIONS.contains(&word)
        || word.ends_with(NEGATION_SUFFIX)
        || QUANTIFIERS.iter().any(|(a, b)| *a == word || *b == word)
}
