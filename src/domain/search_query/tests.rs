use super::*;
use QueryFilter as F;

fn q(input: &str) -> PostQuery {
    parse_query(input)
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn day(y: i16, m: i8, d: i8) -> QueryDate {
    QueryDate::Day(jiff::civil::date(y, m, d))
}

#[test]
fn blank_input_is_an_empty_query() {
    assert!(q("").is_empty());
    assert!(q("   \t ").is_empty());
    assert!(!q("x").is_empty());
}

#[test]
fn plain_words_are_free_text() {
    let query = q("sourdough  starter\tnotes");
    assert_eq!(query.words, strings(&["sourdough", "starter", "notes"]));
    assert_eq!(query.text(), "sourdough starter notes");
}

#[test]
fn every_filter_key_is_read() {
    let query = q("from:sam kind:note symbol:task tint:amber has:image is:open before:2026-12-01 after:2026-01-31");
    let expected = vec![
        F::From("sam".into()),
        F::Kind("note".into()),
        F::Symbol("task".into()),
        F::Tint("amber".into()),
        F::Has(HasFilter::Image),
        F::Is(IsFilter::Open),
        F::Before(day(2026, 12, 1)),
        F::After(day(2026, 1, 31)),
    ];
    assert_eq!(query.filters, expected);
    assert!(query.words.is_empty());
}

#[test]
fn all_has_and_is_values_are_known() {
    let has = q("has:image has:file has:code has:todo").filters;
    let expected: Vec<_> = [HasFilter::Image, HasFilter::File, HasFilter::Code, HasFilter::Todo]
        .into_iter()
        .map(F::Has)
        .collect();
    assert_eq!(has, expected);
    let is = q("is:open is:done is:scheduled is:archived is:locked").filters;
    let states = [IsFilter::Open, IsFilter::Done, IsFilter::Scheduled, IsFilter::Archived, IsFilter::Locked];
    assert_eq!(is, states.into_iter().map(F::Is).collect::<Vec<_>>());
}

#[test]
fn keys_and_enum_values_ignore_case() {
    let query = q("FROM:Sam Has:IMAGE IS:Done");
    let expected = vec![F::From("Sam".into()), F::Has(HasFilter::Image), F::Is(IsFilter::Done)];
    assert_eq!(query.filters, expected, "free values keep their case");
}

#[test]
fn unknown_keys_and_values_stay_free_text() {
    let query = q("lang:rust has:video is:pinned from: https://example.com 10:30");
    let words = strings(&["lang:rust", "has:video", "is:pinned", "from:", "https://example.com", "10:30"]);
    assert_eq!(query.words, words);
    assert!(query.filters.is_empty());
}

#[test]
fn quoted_phrases_keep_their_spaces() {
    let query = q(r#"bread "starter recipe" crumb "  padded  ""#);
    assert_eq!(query.words, strings(&["bread", "crumb"]));
    assert_eq!(query.phrases, strings(&["starter recipe", "padded"]));
    assert_eq!(query.text(), "bread crumb starter recipe padded");
}

#[test]
fn unclosed_quote_runs_to_the_end_and_empty_quotes_vanish() {
    assert_eq!(q(r#"a "open phrase here"#).phrases, strings(&["open phrase here"]));
    assert!(q(r#""" """#).is_empty());
}

#[test]
fn curly_quotes_work_like_straight_ones() {
    assert_eq!(q("\u{201C}smart quotes\u{201D} x").phrases, strings(&["smart quotes"]));
}

#[test]
fn quoted_filter_values_may_contain_spaces() {
    let query = q(r#"from:"Sam Lee" tint:"deep blue""#);
    assert_eq!(query.filters, vec![F::From("Sam Lee".into()), F::Tint("deep blue".into())]);
}

#[test]
fn from_drops_the_mention_sigil() {
    assert_eq!(q("from:@sam").filters, vec![F::From("sam".into())]);
    assert_eq!(q("from:@").words, strings(&["from:@"]));
}

#[test]
fn negation_excludes_words_phrases_and_filters() {
    let query = q(r#"bread -burnt -"too salty" -from:bot -has:code -lang:x"#);
    assert_eq!(query.words, strings(&["bread"]));
    assert_eq!(query.excluded_words, strings(&["burnt", "lang:x"]));
    assert_eq!(query.excluded_phrases, strings(&["too salty"]));
    assert_eq!(query.excluded_filters, vec![F::From("bot".into()), F::Has(HasFilter::Code)]);
    assert!(query.filters.is_empty());
}

#[test]
fn lone_dash_and_inner_dashes_are_words() {
    let query = q("- well-known x-y");
    assert_eq!(query.words, strings(&["-", "well-known", "x-y"]));
    assert!(query.excluded_words.is_empty());
}

#[test]
fn dates_that_are_not_valid_days_stay_raw() {
    let query = q("before:yesterday after:2026-02-30 before:2026-1-5 after:20261201");
    let expected = vec![
        F::Before(QueryDate::Raw("yesterday".into())),
        F::After(QueryDate::Raw("2026-02-30".into())),
        F::Before(QueryDate::Raw("2026-1-5".into())),
        F::After(QueryDate::Raw("20261201".into())),
    ];
    assert_eq!(query.filters, expected);
}

#[test]
fn unicode_text_and_values_are_kept() {
    let query = q("パン tint:琥珀 \"焼き たて\" -焦げ");
    assert_eq!(query.words, strings(&["パン"]));
    assert_eq!(query.filters, vec![F::Tint("琥珀".into())]);
    assert_eq!(query.phrases, strings(&["焼き たて"]));
    assert_eq!(query.excluded_words, strings(&["焦げ"]));
}

#[test]
fn parse_matches_the_free_function() {
    assert_eq!(PostQuery::parse("a from:b"), q("a from:b"));
}
