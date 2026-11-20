//! Property checks over thousands of generated bodies (a seeded generator, so runs repeat).

use super::*;

/// Generated bodies per property.
const CASES: usize = 4000;
/// Longest generated body, in characters.
const MAX_LEN: u64 = 48;
/// Characters that write no markup at all.
const PLAIN: &[char] = &['a', 'b', 'Z', ' ', ' ', '\n', '.', ',', '-', '!', 'é', '字', '1', '(', ')'];
/// Plain characters plus every markup character, weighted towards markup.
const MIXED: &[char] = &[
    'a', 'b', ' ', '\n', '.', '-', 'é', '*', '*', '_', '_', '|', '|', '`', '`', '\\', '#', '[', ']', '[', ']', '\r',
];
/// Characters the parser lets a backslash escape.
const ESCAPABLE: &[char] = &['\\', '*', '_', '`', '|', '#', '[', ']'];

/// Small xorshift generator: deterministic, no extra dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn body(&mut self, alphabet: &[char]) -> String {
        let len = self.next() % MAX_LEN;
        (0..len).map(|_| alphabet[(self.next() % alphabet.len() as u64) as usize]).collect()
    }
}

fn bodies(alphabet: &'static [char], seed: u64) -> impl Iterator<Item = String> {
    let mut rng = Rng(seed);
    (0..CASES).map(move |_| rng.body(alphabet))
}

/// Text runs are never empty, never adjacent, and marks always hold something.
fn assert_well_formed(tokens: &[BodyToken], source: &str) {
    for pair in tokens.windows(2) {
        let adjacent_text = matches!(pair, [BodyToken::Text(_), BodyToken::Text(_)]);
        assert!(!adjacent_text, "unmerged text in {source:?}: {tokens:?}");
    }
    for token in tokens {
        assert!(!matches!(token, BodyToken::Text(t) if t.is_empty()), "empty text in {source:?}");
        if let Some((_, children)) = token.as_mark() {
            assert!(!children.is_empty(), "empty mark in {source:?}");
            assert_well_formed(children, source);
        }
    }
}

/// Whether `needle` appears in `haystack` in order (not necessarily contiguous).
fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut rest = haystack.chars();
    needle.chars().all(|c| rest.any(|h| h == c))
}

#[test]
fn generated_markup_never_panics_and_only_hides_markup() {
    for source in bodies(MIXED, 0x9E37_79B9_7F4A_7C15) {
        let tokens = tokenize_post_body(&source);
        assert_well_formed(&tokens, &source);
        let visible = visible_text(&tokens);
        assert_eq!(visible_len(&tokens), visible.chars().count(), "{source:?}");
        assert!(visible_len(&tokens) <= source.chars().count(), "{source:?}");
        assert!(is_subsequence(&visible, &source), "{source:?} -> {visible:?}");
    }
}

#[test]
fn plain_text_round_trips_unchanged() {
    for source in bodies(PLAIN, 0xD1B5_4A32_D192_ED03) {
        let expected = if source.is_empty() { Vec::new() } else { vec![text(&source)] };
        assert_eq!(tokenize_post_body(&source), expected, "{source:?}");
        assert_eq!(visible_len(&tokenize_post_body(&source)), source.chars().count());
    }
}

#[test]
fn escaping_every_markup_character_gives_back_the_text() {
    for source in bodies(MIXED, 0x2545_F491_4F6C_DD1D) {
        let escaped: String = source
            .chars()
            .flat_map(|c| if ESCAPABLE.contains(&c) { vec!['\\', c] } else { vec![c] })
            .collect();
        let expected = if source.is_empty() { Vec::new() } else { vec![text(&source)] };
        assert_eq!(tokenize_post_body(&escaped), expected, "{escaped:?}");
    }
}

#[test]
fn wrapping_plain_words_in_a_mark_yields_that_mark() {
    let mut rng = Rng(0xA076_1D64_78BD_642F);
    for _ in 0..CASES {
        let word: String = rng.body(&['a', 'b', 'é', '字', '1']);
        if word.is_empty() {
            continue;
        }
        for mark in [Mark::Bold, Mark::Italic, Mark::Underline, Mark::Spoiler] {
            let source = format!("x {0}{word}{0} y", mark.delimiter());
            assert_eq!(tokenize_post_body(&source), vec![text("x "), mark.wrap(vec![text(&word)]), text(" y")]);
        }
    }
}
