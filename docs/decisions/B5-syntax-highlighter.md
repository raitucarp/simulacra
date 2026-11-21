# B5: syntax highlighter for code blocks in posts

Status: decided (issue #31). Used by the code block card (#32).

## Need

Fenced code blocks in post bodies (```` ```lang ````) get colored keywords, strings,
comments, numbers and type names. Posts are short (a few hundred characters), the blocks
are read, never edited, and every frame of the feed may re-render them. Colors must come
from the theme tokens like every other color in the app.

## Options compared

| | UI toolkit + `tree-sitter` feature | UI toolkit + `syntect` feature | Own lexical highlighter |
| --- | --- | --- | --- |
| What it is | tree-sitter parser plus one C grammar crate per language | syntect (TextMate grammars, `fancy-regex`) plus the two-face syntax set | a small tokenizer in `src/domain/post/highlight`: comments, strings, numbers, keywords and type names from per-language tables |
| Pure Rust | no: every grammar is C, built by `cc` | yes | yes |
| New crates | `tree-sitter` and a grammar crate per language (plus their `cc` builds) | `syntect`, `two-face`, `fancy-regex`, `bincode` 1 and their own dependencies | none |
| Licences | MIT (runtime and most grammars) | MIT / MIT OR Apache-2.0 | ours |
| Advisories | none known | `bincode` 1.x is flagged unmaintained (RUSTSEC-2025-0141); allowed by `deny.toml` only because it is transitive | none |
| Languages | one crate per language, opt-in | about 200 | the 15 most common in posts (see below), more by adding a table |
| Binary size (upstream figures) | "tens of megabytes" for the full set; a few MB for a handful | "a few megabytes" for the syntax dump | a few KB of tables |
| Build time | a C compile per grammar on every OS; needs a C toolchain on Windows, macOS and Linux runners | moderate (regex engine, dump embedding) | none measurable: one more module of the app crate |
| Reachable API | the toolkit's public `SyntaxHighlighter` (rope-based, made for its editor) | `highlight_code` is crate-private; only reachable through the toolkit's markdown text view, which we do not use for post bodies | a plain function returning byte ranges, easy to test |
| Accuracy | structural, best | very good (TextMate grammars) | lexical: right for keywords, strings, comments, numbers; no context such as string interpolation or nested generics |

Binary-size and build-time figures for the two toolkit features are the toolkit's own
documentation (its `syntect` backend notes "about two hundred languages for a few
megabytes, where the full tree-sitter set costs tens"). They were not re-measured on each
OS: neither feature is picked, and measuring would have meant another full build of the
UI toolkit. The chosen option adds no crate, so the existing CI builds on Windows, macOS
and Linux are the measurement: their size and build time do not change in any way that
shows.

## Decision

**Own lexical highlighter**, in the domain layer (`crate::domain::post::highlight`).

- It keeps the repository pure Rust and adds no dependency, licence or advisory to
  `cargo deny check`.
- Post code blocks are short snippets shown read-only; a lexical pass gets the parts people
  actually look at (keywords, strings, comments, numbers, types) right, and costs a
  microsecond-scale scan per render.
- The output is a list of `(byte range, CodeStyle)` pairs, so the UI maps each style onto a
  theme token (`code_keyword`, `code_type`, `code_string`, `code_number`, `code_comment`).
- Unknown or missing language tags fall back to plain monospace text, never to a guess.

Languages at launch: Rust, Python, JavaScript, TypeScript, JSON, Go, C, C++, Java, C#,
Kotlin, shell (sh / bash / zsh), SQL, TOML and YAML, with the usual aliases (`rs`, `py`,
`js`, `ts`, `c++`, `cs`, `kt`, `sh`, `yml`, ...).

## Revisit when

- Posts start carrying long code (whole files), or people ask for languages whose lexical
  shape the tables cannot express (markup, heredocs, string interpolation).
- The UI toolkit makes its `syntect` highlighter public for arbitrary text and the
  unmaintained `bincode` 1 leaves its tree. Switching then only replaces the body of
  `highlight()`: the `CodeStyle` ranges and theme tokens stay.
