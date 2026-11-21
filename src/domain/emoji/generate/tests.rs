//! The emoji table generator (R3) on a small excerpt of `emoji-test.txt`.

use super::{parse_emoji_test, render_table, ParseError};

const EXCERPT: &str = "\
# emoji-test.txt
# Version: 17.0

# group: Smileys & Emotion

# subgroup: face-smiling
1F600                                                  ; fully-qualified     # \u{1F600} E1.0 grinning face
263A FE0F                                              ; fully-qualified     # \u{263A}\u{FE0F} E0.6 smiling face
263A                                                   ; unqualified         # \u{263A} E0.6 smiling face

# group: People & Body

# subgroup: hand-fingers-open
1F44B                                                  ; fully-qualified     # \u{1F44B} E0.6 waving hand
1F44B 1F3FB                                            ; fully-qualified     # x E1.0 waving hand: light skin tone
1F44B 1F3FC                                            ; fully-qualified     # x E1.0 waving hand: medium-light skin tone
1F44B 1F3FD                                            ; fully-qualified     # x E1.0 waving hand: medium skin tone
1F44B 1F3FE                                            ; fully-qualified     # x E1.0 waving hand: medium-dark skin tone
1F44B 1F3FF                                            ; fully-qualified     # x E1.0 waving hand: dark skin tone

# subgroup: person
1F468 200D 1F9B0                                       ; fully-qualified     # x E11.0 man: red hair
1F468 1F3FB 200D 1F9B0                                 ; fully-qualified     # x E11.0 man: light skin tone, red hair
1F91D                                                  ; fully-qualified     # x E3.0 handshake
1FAF1 1F3FB 200D 1FAF2 1F3FC                           ; fully-qualified     # x E14.0 handshake: light skin tone, medium-light skin tone

# group: Component

# subgroup: skin-tone
1F3FB                                                  ; component           # x E1.0 light skin tone
";

#[test]
fn reads_the_version_groups_names_and_versions() {
    let data = parse_emoji_test(EXCERPT).expect("parses");
    assert_eq!(data.version, "17.0");
    let names: Vec<&str> = data.emoji.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, vec!["grinning face", "smiling face", "waving hand", "man: red hair", "handshake"]);
    assert_eq!(data.emoji[0].glyph, "\u{1F600}", "glyphs are built from the code points");
    assert_eq!(data.emoji[1].glyph, "\u{263A}\u{FE0F}", "only the fully-qualified form is kept");
    assert_eq!((data.emoji[0].since, data.emoji[3].since), ((1, 0), (11, 0)));
    assert_eq!((data.emoji[0].group, data.emoji[2].group), (0, 1));
    assert_eq!(data.emoji[2].subgroup, "hand-fingers-open");
}

#[test]
fn folds_single_tones_into_their_base_and_skips_mixed_or_partial_ones() {
    let data = parse_emoji_test(EXCERPT).expect("parses");
    let wave = &data.emoji[2];
    assert_eq!(wave.tones.len(), 5);
    assert_eq!(wave.tones[0], "\u{1F44B}\u{1F3FB}");
    assert_eq!(wave.tones[4], "\u{1F44B}\u{1F3FF}");
    assert!(data.emoji[3].tones.is_empty(), "one tone of five is not offered");
    assert!(data.emoji[4].tones.is_empty(), "mixed tones are left out");
}

#[test]
fn reports_what_it_cannot_read() {
    assert_eq!(parse_emoji_test("# group: Smileys & Emotion\n"), Err(ParseError::MissingVersion));
    assert_eq!(parse_emoji_test("# Version: 1\n# group: Smileys & Emotion\nnot a line\n"), Err(ParseError::BadLine(3)));
    assert_eq!(parse_emoji_test("# Version: 1\n# group: Sports\n"), Err(ParseError::UnknownGroup("Sports".into())));
}

#[test]
fn renders_one_line_per_subgroup_with_escaped_text() {
    let source = render_table(&parse_emoji_test(EXCERPT).expect("parses"));
    assert!(source.contains("pub const UNICODE_EMOJI_VERSION: &str = \"17.0\";"));
    let rows: Vec<&str> = source.lines().filter(|line| line.starts_with("    /* ")).collect();
    assert_eq!(rows.len(), 3);
    assert!(rows[0].starts_with("    /* face-smiling */ e(\"\u{1F600}\", \"grinning face\", SmileysEmotion, 1, 0, &[]),"));
    assert!(rows[1].contains("\"waving hand\", PeopleBody, 0, 6, &[\"\u{1F44B}\u{1F3FB}\""));
    assert!(rows[0].contains("\\u{fe0f}"), "invisible selectors are written as escapes");
}
