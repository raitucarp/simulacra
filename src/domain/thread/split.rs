//! Splitting an over-long draft into a thread of posts that each fit the character limit.

/// Characters that end a sentence when followed by whitespace (or the end of the text).
const SENTENCE_TERMINATORS: [char; 4] = ['.', '!', '?', '…'];
/// Line break: always a boundary, and kept as the joiner between pieces it separated.
const LINE_BREAK: char = '\n';
/// Joiner between pieces that were separated by inline whitespace.
const SPACE_JOINER: &str = " ";
/// Joiner between pieces that were separated by a line break.
const LINE_JOINER: &str = "\n";

/// Splits `text` into chunks of at most `limit` characters, preferring sentence boundaries,
/// then word boundaries, and hard-splitting only words that alone exceed the limit.
///
/// Text that already fits comes back as a single trimmed chunk; blank text yields no chunks.
pub fn split_into_thread(text: &str, limit: usize) -> Vec<String> {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }
    if limit == 0 || char_len(trimmed) <= limit {
        return vec![trimmed.to_string()];
    }
    let mut packer = Packer::new(limit);
    for (joiner, sentence) in sentences(trimmed) {
        packer.push_sentence(joiner, sentence);
    }
    packer.finish()
}

fn char_len(text: &str) -> usize {
    text.chars().count()
}

/// Sentences of `text` (trimmed, non-empty), each with the joiner that preceded it.
fn sentences(text: &str) -> Vec<(&'static str, &str)> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((idx, c)) = chars.next() {
        let next_is_space = chars.peek().is_none_or(|(_, n)| n.is_whitespace());
        let is_end = c == LINE_BREAK || (SENTENCE_TERMINATORS.contains(&c) && next_is_space);
        if is_end {
            let end = idx + c.len_utf8();
            push_piece(text, start, end, &mut out);
            start = end;
        }
    }
    push_piece(text, start, text.len(), &mut out);
    out
}

/// Pushes the trimmed `text[start..end]` with the joiner implied by the gap before it.
fn push_piece<'a>(text: &'a str, start: usize, end: usize, out: &mut Vec<(&'static str, &'a str)>) {
    let piece = &text[start..end];
    let body = piece.trim();
    if body.is_empty() {
        return;
    }
    let lead = &piece[..piece.len() - piece.trim_start().len()];
    let after_break = lead.contains(LINE_BREAK) || text[..start].ends_with(LINE_BREAK);
    out.push((if after_break { LINE_JOINER } else { SPACE_JOINER }, body));
}

/// Greedy chunk builder.
struct Packer {
    limit: usize,
    chunks: Vec<String>,
    current: String,
}

impl Packer {
    fn new(limit: usize) -> Self {
        Self { limit, chunks: Vec::new(), current: String::new() }
    }

    /// Appends `piece` to the current chunk when it fits, otherwise starts a new chunk.
    fn push_piece(&mut self, joiner: &str, piece: &str) {
        let joined_len = char_len(&self.current) + char_len(joiner) + char_len(piece);
        if self.current.is_empty() {
            self.current.push_str(piece);
        } else if joined_len <= self.limit {
            self.current.push_str(joiner);
            self.current.push_str(piece);
        } else {
            self.flush();
            self.current.push_str(piece);
        }
    }

    fn push_sentence(&mut self, joiner: &str, sentence: &str) {
        if char_len(sentence) <= self.limit {
            return self.push_piece(joiner, sentence);
        }
        let mut word_joiner = joiner;
        for word in sentence.split_whitespace() {
            self.push_word(word_joiner, word);
            word_joiner = SPACE_JOINER;
        }
    }

    /// Words that alone exceed the limit are cut into limit-sized pieces.
    fn push_word(&mut self, joiner: &str, word: &str) {
        if char_len(word) <= self.limit {
            return self.push_piece(joiner, word);
        }
        let chars: Vec<char> = word.chars().collect();
        for piece in chars.chunks(self.limit) {
            self.push_piece(joiner, &piece.iter().collect::<String>());
        }
    }

    fn flush(&mut self) {
        if !self.current.is_empty() {
            self.chunks.push(std::mem::take(&mut self.current));
        }
    }

    fn finish(mut self) -> Vec<String> {
        self.flush();
        self.chunks
    }
}
