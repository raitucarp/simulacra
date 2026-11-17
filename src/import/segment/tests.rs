use super::*;
use crate::import::detect::Format;
use crate::import::document::{Block, BlockKind, DocumentMetadata, FileFacts, ParsedDocument, Section};

fn split(text: &str, tag: &str) -> Vec<String> {
    split_sentences(text, SentenceLocale::from_language_tag(Some(tag))).into_iter().map(str::to_string).collect()
}

#[test]
fn english_abbreviations_initials_and_numbers() {
    assert_eq!(
        split("Mr. Holmes met Dr. Watson at 3.15 p.m. on Baker St. in London. J. R. R. Tolkien wrote e.g. The Hobbit. It sold well.", "en"),
        ["Mr. Holmes met Dr. Watson at 3.15 p.m. on Baker St. in London.", "J. R. R. Tolkien wrote e.g. The Hobbit.", "It sold well."]
    );
}

#[test]
fn quotes_ellipses_and_runs_of_terminators() {
    assert_eq!(
        split("\"Stop!\" he cried. \"Why?!\" She paused… Then she ran... and ran. (It was late.) Fine.", "en"),
        ["\"Stop!\" he cried.", "\"Why?!\"", "She paused…", "Then she ran... and ran.", "(It was late.)", "Fine."]
    );
}

#[test]
fn german_french_spanish_and_portuguese_abbreviations() {
    assert_eq!(split("Wir kaufen z.B. Äpfel. Das ist gut. Vgl. Kapitel 2.", "de"), ["Wir kaufen z.B. Äpfel.", "Das ist gut.", "Vgl. Kapitel 2."]);
    assert_eq!(split("M. Dupont arrive. Il pleut « encore ». Mme Curie aussi.", "fr"), ["M. Dupont arrive.", "Il pleut « encore ».", "Mme Curie aussi."]);
    assert_eq!(split("¿Dónde está la Sra. García? ¡Aquí! Bien.", "es"), ["¿Dónde está la Sra. García?", "¡Aquí!", "Bien."]);
    assert_eq!(split("O Sr. Silva chegou. Que bom!", "pt-BR"), ["O Sr. Silva chegou.", "Que bom!"]);
}

#[test]
fn cjk_terminators_need_no_space() {
    assert_eq!(split("今日は雨です。明日は晴れ！本当？", "ja"), ["今日は雨です。", "明日は晴れ！", "本当？"]);
    assert_eq!(split("他说：「好。」我们走吧。", "zh-CN"), ["他说：「好。」", "我们走吧。"]);
}

#[test]
fn russian_arabic_hindi_and_indonesian() {
    assert_eq!(split("Это т.е. пример. Второе предложение.", "ru"), ["Это т.е. пример.", "Второе предложение."]);
    assert_eq!(split("هل أنت بخير؟ نعم. شكرا.", "ar"), ["هل أنت بخير؟", "نعم.", "شكرا."]);
    assert_eq!(split("यह पहला वाक्य है। यह दूसरा है।", "hi"), ["यह पहला वाक्य है।", "यह दूसरा है।"]);
    assert_eq!(split("Bpk. Budi datang. Dia senang.", "id"), ["Bpk. Budi datang.", "Dia senang."]);
}

#[test]
fn edge_cases_never_lose_text() {
    assert!(split("", "en").is_empty());
    assert_eq!(split("No terminator at all", "en"), ["No terminator at all"]);
    assert_eq!(split("Version 2.0 is out.", "en"), ["Version 2.0 is out."]);
    assert_eq!(split("Ends with abbreviation Mr.", "en"), ["Ends with abbreviation Mr."]);
}

fn book() -> ParsedDocument {
    let chapter = |title: &str, body: &[&str]| {
        let mut blocks = vec![Block::new(BlockKind::Heading(2), title)];
        blocks.extend(body.iter().map(|text| Block::paragraph(*text)));
        Section::new(Some(title.to_string()), blocks)
    };
    ParsedDocument {
        format: Format::Epub,
        metadata: DocumentMetadata { language: Some("en".into()), ..DocumentMetadata::default() },
        facts: FileFacts::default(),
        sections: vec![chapter("One", &["A b. C d.", "E f."]), chapter("Two", &["G h."]), Section::new(None, vec![])],
        notes: Vec::new(),
    }
}

#[test]
fn documents_segment_by_chapter_paragraph_and_sentence() {
    let doc = book();
    let chapters = segment_document(&doc, Granularity::Chapter);
    assert_eq!(chapters.iter().map(|u| u.text.as_str()).collect::<Vec<_>>(), ["A b. C d.\n\nE f.", "G h."]);
    let paragraphs = segment_document(&doc, Granularity::Paragraph);
    assert_eq!(paragraphs.len(), 3);
    assert_eq!(paragraphs[2].position, UnitPosition { chapter: 1, paragraph: Some(0), sentence: None });
    let sentences = segment_document(&doc, Granularity::Sentence);
    let texts: Vec<&str> = sentences.iter().map(|u| u.text.as_str()).collect();
    assert_eq!(texts, ["A b.", "C d.", "E f.", "G h."]);
    assert_eq!(sentences[1].position, UnitPosition { chapter: 0, paragraph: Some(0), sentence: Some(1) });
    assert_eq!(Granularity::from(crate::domain::preferences::ImportSegmentation::Sentence), Granularity::Sentence);
}
