use super::counts::TextCounts;
use super::formulas::{fernandez_huerta, flesch_kincaid_grade, lix};
use super::scale::{ease, interpolate};
use super::syllables::{syllables, SyllableRules};
use super::*;

const CLOSE: f64 = 1e-6;

fn close(actual: f64, expected: f64) {
    assert!((actual - expected).abs() < CLOSE, "{actual} != {expected}");
}

/// Ten words in one sentence, fifteen syllables, three long words.
fn counts() -> TextCounts {
    TextCounts { sentences: 1, words: 10, syllables: 15, letters: 50, long_words: 3, cjk_chars: 0 }
}

#[test]
fn english_syllables_follow_the_usual_heuristic() {
    let count = |word| syllables(word, SyllableRules::English);
    assert_eq!([count("the"), count("make"), count("table"), count("jumped"), count("beautiful"), count("yellow")], [1, 1, 2, 1, 3, 2]);
    assert_eq!(count("readability"), 5);
}

#[test]
fn other_languages_count_their_own_way() {
    assert_eq!(syllables("Freiheit", SyllableRules::German), 2);
    assert_eq!(syllables("liberté", SyllableRules::French), 3);
    assert_eq!(syllables("table", SyllableRules::French), 1, "mute final e");
    assert_eq!(syllables("poeta", SyllableRules::Spanish), 3, "hiatus o-e");
    assert_eq!(syllables("día", SyllableRules::Spanish), 2, "accented weak vowel");
    assert_eq!(syllables("aire", SyllableRules::Spanish), 2, "diphthong ai");
    assert_eq!(syllables("coração", SyllableRules::Portuguese), 3);
    assert_eq!(syllables("молоко", SyllableRules::Russian), 3);
}

#[test]
fn the_published_formulas() {
    let c = counts();
    close(Formula::FleschEnglish.value(&c), 206.835 - 10.15 - 126.9);
    close(Formula::Amstad.value(&c), 180.0 - 10.0 - 87.75);
    close(Formula::KandelMoles.value(&c), 207.0 - 10.15 - 110.4);
    close(Formula::FleschPortuguese.value(&c), 248.835 - 10.15 - 126.9);
    close(Formula::Oborneva.value(&c), 206.835 - 13.0 - 90.15);
    close(fernandez_huerta(10.0, 1.5), 206.84 - 90.0 - 10.2);
    close(lix(&c), 40.0);
    close(flesch_kincaid_grade(&c), 3.9 + 17.7 - 15.59);
}

#[test]
fn a_simple_english_sentence_reads_very_easily() {
    let score = readability("The cat sat on the mat.", ReadingLanguage::English).expect("scored");
    assert_eq!((score.formula, score.counts.words, score.counts.sentences, score.counts.syllables), (Formula::FleschEnglish, 6, 1, 6));
    close(score.value, 206.835 - 1.015 * 6.0 - 84.6);
    close(score.ease, 100.0);
    assert_eq!(score.band, Band::VeryEasy);
    close(score.grade.expect("grade"), 0.39 * 6.0 + 11.8 - 15.59);
}

#[test]
fn dense_prose_reads_as_hard() {
    let text = "Epistemological considerations notwithstanding, institutional responsibilities necessitate comprehensive interdisciplinary deliberation regarding fundamental methodological presuppositions.";
    let score = readability(text, ReadingLanguage::English).expect("scored");
    assert!(score.band.is_hard(), "{score:?}");
    assert!(score.grade.expect("grade") > 16.0);
}

#[test]
fn the_script_picks_the_formula() {
    assert_eq!(formula_for("Это простой текст на русском языке.", ReadingLanguage::English), Formula::Oborneva);
    assert_eq!(formula_for("これは短い文です。次の文です。", ReadingLanguage::English), Formula::CjkSentenceLength);
    assert_eq!(formula_for("هذا نص قصير باللغة العربية", ReadingLanguage::English), Formula::Lix);
    assert_eq!(formula_for("Ini adalah kalimat sederhana.", ReadingLanguage::Other), Formula::Lix);
    assert_eq!(formula_for("Das ist ein Satz.", ReadingLanguage::German), Formula::Amstad);
    assert_eq!(ReadingLanguage::from_code("pt-BR"), ReadingLanguage::Portuguese);
    assert_eq!(ReadingLanguage::from_code("zh-CN"), ReadingLanguage::Other);
}

#[test]
fn japanese_is_measured_in_characters_per_sentence() {
    let score = readability("これは日本語の短い文です。とても読みやすい文です。", ReadingLanguage::Other).expect("scored");
    assert_eq!(score.formula, Formula::CjkSentenceLength);
    assert_eq!(score.counts.sentences, 2);
    assert!(score.value > 9.0 && score.value < 13.0, "{}", score.value);
    assert_eq!(score.band, Band::VeryEasy);
}

#[test]
fn too_short_texts_get_no_score() {
    assert!(readability("Hi there", ReadingLanguage::English).is_none());
    assert!(readability("", ReadingLanguage::English).is_none());
    assert!(readability("短い文。", ReadingLanguage::Other).is_none());
}

#[test]
fn bands_and_the_shared_scale() {
    assert_eq!([80.0, 79.9, 60.0, 50.0, 49.0, 29.0].map(Band::of_ease), [Band::VeryEasy, Band::Plain, Band::Plain, Band::FairlyDifficult, Band::Difficult, Band::VeryDifficult]);
    close(ease(Formula::Lix, 30.0), 80.0);
    close(ease(Formula::Lix, 45.0), 55.0);
    close(ease(Formula::CjkSentenceLength, 200.0), 0.0);
    close(ease(Formula::Amstad, 130.0), 100.0);
    close(interpolate(&[(0.0, 10.0), (10.0, 0.0)], -5.0), 10.0);
    assert_eq!(scale::band(Formula::Lix, 61.0), Band::VeryDifficult);
}

#[test]
fn links_and_mentions_are_not_words() {
    let counts = TextCounts::of("See https://example.org and @alice for #details here.", SyllableRules::English);
    assert_eq!(counts.words, 4, "see, and, for, here");
}
