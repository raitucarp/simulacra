use super::super::sentence::split_sentences;
use super::*;

fn chars(size: usize, overlap: usize) -> ChunkSpec {
    ChunkSpec { size, overlap, unit: ChunkUnit::Characters }
}

fn texts(chunks: &[TextChunk]) -> Vec<&str> {
    chunks.iter().map(|chunk| chunk.text.as_str()).collect()
}

#[test]
fn sentences_split_at_terminals_and_line_breaks_only() {
    let text = "Pi is 3.14 today. Is it? \"Yes!\" she said.\nNew line… and more";
    assert_eq!(split_sentences(text), vec!["Pi is 3.14 today.", "Is it?", "\"Yes!\"", "she said.", "New line…", "and more"]);
    assert_eq!(split_sentences("它是什么。我不知道！"), vec!["它是什么。", "我不知道！"]);
    assert!(split_sentences("   \n  ").is_empty());
}

#[test]
fn blank_text_has_no_chunks_and_short_text_one() {
    assert!(chunk_text("  ", chars(100, 10)).is_empty());
    let one = chunk_text("A single short thought.", chars(100, 10));
    assert_eq!(texts(&one), vec!["A single short thought."]);
    assert_eq!(one[0].index, 0);
}

#[test]
fn whole_sentences_are_packed_up_to_the_size() {
    let text = "One two. Three four. Five six. Seven eight.";
    let chunks = chunk_text(text, chars(22, 0));
    assert_eq!(texts(&chunks), vec!["One two. Three four.", "Five six. Seven eight."]);
    assert_eq!(chunks.iter().map(|c| c.index).collect::<Vec<_>>(), vec![0, 1]);
}

#[test]
fn the_next_chunk_repeats_trailing_sentences_that_fit_the_overlap() {
    let text = "Aaaa. Bbbb. Cccc. Dddd. Eeee.";
    let chunks = chunk_text(text, chars(17, 5));
    assert_eq!(texts(&chunks), vec!["Aaaa. Bbbb. Cccc.", "Cccc. Dddd. Eeee."]);
}

#[test]
fn a_sentence_longer_than_a_chunk_is_cut_by_words() {
    let text = "alpha beta gamma delta epsilon zeta";
    let chunks = chunk_text(text, chars(12, 0));
    assert!(chunks.len() >= 3, "{chunks:?}");
    assert!(chunks.iter().all(|c| c.text.chars().count() <= 12), "{chunks:?}");
    let rejoined: Vec<&str> = chunks.iter().flat_map(|c| c.text.split_whitespace()).collect();
    assert_eq!(rejoined.join(" "), text, "no word is lost or repeated without overlap");
}

#[test]
fn token_sizes_estimate_four_characters_per_token() {
    let spec = ChunkSpec { size: 4, overlap: 0, unit: ChunkUnit::Tokens };
    assert_eq!(spec.measure("a bb ccccc"), 1 + 1 + 2);
    let chunks = chunk_text("Tiny. Words here. More words now.", spec);
    assert!(chunks.iter().all(|c| spec.measure(&c.text) <= 4), "{chunks:?}");
}

#[test]
fn prefs_give_token_chunks_and_overlap_never_exceeds_half_a_chunk() {
    let prefs = crate::domain::preferences::EmbeddingPrefs::default();
    let spec = ChunkSpec::from_prefs(&prefs);
    assert_eq!((spec.size, spec.overlap, spec.unit), (512, 64, ChunkUnit::Tokens));
    let text = "Aaaa. Bbbb. Cccc. Dddd.";
    let greedy = chunk_text(text, chars(11, 100));
    assert!(greedy.len() >= 2 && greedy.len() <= 4, "a huge overlap still makes progress: {greedy:?}");
}
