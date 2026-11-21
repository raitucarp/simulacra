//! `emoji-table` (R3): regenerates the emoji table from Unicode's `emoji-test.txt`.
//!
//! `cargo run --features codegen --bin emoji-table -- <emoji-test.txt> [output.rs]`
//!
//! The output defaults to `src/domain/emoji/table.rs`. The input is a local file (from the
//! Unicode site, or a copy shipped inside a crate); nothing is downloaded.

use simulacra::domain::emoji::generate::{parse_emoji_test, render_table};
use std::process::ExitCode;

const DEFAULT_OUTPUT: &str = "src/domain/emoji/table.rs";
const USAGE: &str = "usage: emoji-table <emoji-test.txt> [output.rs]";

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(input) = args.next() else {
        eprintln!("{USAGE}");
        return ExitCode::FAILURE;
    };
    let output = args.next().unwrap_or_else(|| DEFAULT_OUTPUT.to_string());
    let text = match std::fs::read_to_string(&input) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("emoji-table: cannot read {input}: {err}");
            return ExitCode::FAILURE;
        }
    };
    let data = match parse_emoji_test(&text) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("emoji-table: {input}: {err:?}");
            return ExitCode::FAILURE;
        }
    };
    if let Err(err) = std::fs::write(&output, render_table(&data)) {
        eprintln!("emoji-table: cannot write {output}: {err}");
        return ExitCode::FAILURE;
    }
    println!("emoji-table: {} emoji (Unicode emoji {}) written to {output}", data.emoji.len(), data.version);
    ExitCode::SUCCESS
}
