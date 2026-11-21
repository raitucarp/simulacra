use super::*;

/// The highlighted text of each span, with its style.
fn spans(lang: &str, code: &str) -> Vec<(String, CodeStyle)> {
    highlight(Some(lang), code).into_iter().map(|(range, style)| (code[range].to_string(), style)).collect()
}

fn has(lang: &str, code: &str, text: &str, style: CodeStyle) -> bool {
    spans(lang, code).contains(&(text.to_string(), style))
}

#[test]
fn rust_keywords_types_strings_numbers_and_comments() {
    let code = "fn main() { let x: u32 = 42; // answer\n  println!(\"hi {x}\"); }";
    assert!(has("rust", code, "fn", CodeStyle::Keyword));
    assert!(has("rust", code, "let", CodeStyle::Keyword));
    assert!(has("rust", code, "u32", CodeStyle::Type));
    assert!(has("rust", code, "42", CodeStyle::Number));
    assert!(has("rust", code, "// answer", CodeStyle::Comment));
    assert!(has("rust", code, "\"hi {x}\"", CodeStyle::String));
    assert!(!spans("rust", code).iter().any(|(text, _)| text == "main" || text == "x"), "plain identifiers stay plain");
}

#[test]
fn rust_lifetimes_are_not_strings_but_char_literals_are() {
    let code = "fn f<'a>(s: &'a str) -> char { 'x' }";
    assert!(!spans("rust", code).iter().any(|(text, style)| *style == CodeStyle::String && text.contains('a')));
    assert!(has("rust", code, "'x'", CodeStyle::String));
    assert!(has("rust", "let c = '\\n';", "'\\n'", CodeStyle::String));
}

#[test]
fn capitalized_names_are_types_where_the_language_says_so() {
    assert!(has("rust", "let v: Vec<String> = Vec::new();", "String", CodeStyle::Type));
    assert!(has("java", "String name = null;", "String", CodeStyle::Type));
    assert!(!has("go", "fmt.Println(x)", "Println", CodeStyle::Type), "Go capitals are exports, not types");
}

#[test]
fn block_comments_and_unclosed_ones_run_to_their_end() {
    assert!(has("c", "int a; /* note */ int b;", "/* note */", CodeStyle::Comment));
    assert!(has("c", "x /* open", "/* open", CodeStyle::Comment));
}

#[test]
fn escaped_quotes_stay_inside_the_string() {
    assert!(has("python", r#"s = "say \"hi\"" # note"#, r#""say \"hi\"""#, CodeStyle::String));
    assert!(has("python", r#"s = "say \"hi\"" # note"#, "# note", CodeStyle::Comment));
}

#[test]
fn unclosed_strings_stop_at_the_line_end_unless_multiline() {
    let lines = highlight_lines(Some("python"), "s = 'open\nx = 1");
    assert_eq!(lines[1], vec![(4..5, CodeStyle::Number)], "the next line is not swallowed");
    assert!(has("javascript", "const t = `a\nb`;", "`a\nb`", CodeStyle::String));
}

#[test]
fn sql_keywords_ignore_case() {
    assert!(has("sql", "SELECT id FROM posts -- all", "SELECT", CodeStyle::Keyword));
    assert!(has("sql", "select id from posts", "from", CodeStyle::Keyword));
    assert!(has("sql", "SELECT 1 -- all", "-- all", CodeStyle::Comment));
}

#[test]
fn numbers_need_a_word_start_and_keep_decimals() {
    assert!(has("python", "x = 3.14", "3.14", CodeStyle::Number));
    assert!(has("rust", "let n = 0xFF_u8;", "0xFF_u8", CodeStyle::Number));
    assert!(!spans("python", "v2 = 1").iter().any(|(text, _)| text == "2"), "a digit inside a name is not a number");
}

#[test]
fn aliases_resolve_and_unknown_languages_stay_plain() {
    for tag in ["rs", "py", "js", "ts", "c++", "cs", "c#", "kt", "sh", "yml", "golang", "jsonc"] {
        assert!(is_known_language(tag), "{tag}");
    }
    assert!(highlight(Some("brainfreeze"), "fn x").is_empty());
    assert!(highlight(None, "fn x").is_empty());
}

#[test]
fn line_highlights_are_relative_to_each_line() {
    let lines = highlight_lines(Some("rust"), "let a = 1;\n// two\nlet b");
    assert_eq!(lines.len(), 3);
    assert_eq!(lines[1], vec![(0..6, CodeStyle::Comment)]);
    assert_eq!(lines[2], vec![(0..3, CodeStyle::Keyword)]);
}

#[test]
fn spans_split_across_lines_are_clipped_per_line() {
    let lines = highlight_lines(Some("c"), "a /* x\ny */ b");
    assert_eq!(lines[0], vec![(2..6, CodeStyle::Comment)]);
    assert_eq!(lines[1], vec![(0..4, CodeStyle::Comment)]);
}

#[test]
fn spans_are_ordered_non_overlapping_and_on_char_boundaries() {
    let code = "fn ünï() { \"é\" /* ü */ 1 } // ö";
    let all = highlight(Some("rust"), code);
    for pair in all.windows(2) {
        assert!(pair[0].0.end <= pair[1].0.start);
    }
    for (range, _) in &all {
        assert!(code.is_char_boundary(range.start) && code.is_char_boundary(range.end));
    }
}

#[test]
fn every_language_highlights_its_keywords() {
    let samples = [
        ("go", "func"), ("cpp", "class"), ("csharp", "namespace"), ("kotlin", "fun"), ("typescript", "interface"),
        ("shell", "fi"), ("json", "true"), ("toml", "false"), ("yaml", "null"),
    ];
    for (lang, keyword) in samples {
        assert!(has(lang, keyword, keyword, CodeStyle::Keyword), "{lang}: {keyword}");
    }
}
