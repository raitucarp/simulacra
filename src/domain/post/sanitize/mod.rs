//! Agent output to post body: the marks posts support stay (`**bold**`, `*italic*`,
//! `__underline__`, `` `code` ``, fenced blocks, `||spoiler||`), other markdown is reduced
//! to the plain text a reader would see.
//!
//! - Headings (`# Title`, `Title` over `===`) become their text; a `#Tag` hashtag stays.
//! - Horizontal rules, table separator rows and setext underlines are dropped.
//! - Block quote markers are removed; `*` and `+` bullets become `-` bullets (a `*` at a line
//!   start would read as an italic delimiter).
//! - Links keep their text and address, images their description, strikethrough its text.
//! - `~~~` fences become backtick fences; code inside fences and spans is never touched.
//! - Runs of blank lines outside code collapse to one.

mod blocks;
mod inline;

/// Line break between output lines.
const LINE_BREAK: &str = "\n";
const BACKTICK: char = '`';
const TILDE: char = '~';
/// Shortest run of fence characters that opens a fenced block.
const MIN_FENCE_RUN: usize = 3;

/// The open fence while copying a fenced block: its character and run length.
#[derive(Clone, Copy)]
struct Fence {
    marker: char,
    run: usize,
}

/// Output lines; blank prose lines never follow one another.
#[derive(Default)]
struct Lines(Vec<String>);

impl Lines {
    fn code(&mut self, line: impl Into<String>) {
        self.0.push(line.into());
    }

    fn prose(&mut self, line: String) {
        let blank = line.trim().is_empty();
        if blank && self.0.last().is_none_or(|last| last.trim().is_empty()) {
            return;
        }
        self.0.push(if blank { String::new() } else { line });
    }
}

/// `text` (an agent's answer) as a post body (see the module rules).
pub fn sanitize_agent_markdown(text: &str) -> String {
    let mut out = Lines::default();
    let mut fence: Option<Fence> = None;
    for line in text.lines() {
        match fence {
            Some(open) if is_closer(line, open) => {
                fence = None;
                out.code(fence_line(open.run, ""));
            }
            Some(_) => out.code(line),
            None => match opener(line) {
                Some((open, info)) => {
                    fence = Some(open);
                    out.code(fence_line(open.run, info));
                }
                None => blocks::prose_line(line).into_iter().for_each(|line| out.prose(line)),
            },
        }
    }
    out.0.join(LINE_BREAK).trim().to_string()
}

/// A backtick fence line of `run` backticks and an info string.
fn fence_line(run: usize, info: &str) -> String {
    format!("{}{info}", BACKTICK.to_string().repeat(run.max(MIN_FENCE_RUN)))
}

/// An opening fence line: its fence and info string (language tag).
fn opener(line: &str) -> Option<(Fence, &str)> {
    let trimmed = line.trim_start();
    let marker = trimmed.chars().next().filter(|c| *c == BACKTICK || *c == TILDE)?;
    let run = trimmed.chars().take_while(|&c| c == marker).count();
    let info = trimmed[run..].trim();
    (run >= MIN_FENCE_RUN && !info.contains(BACKTICK)).then_some((Fence { marker, run }, info))
}

fn is_closer(line: &str, open: Fence) -> bool {
    let trimmed = line.trim();
    trimmed.len() >= open.run && trimmed.chars().all(|c| c == open.marker)
}

#[cfg(test)]
mod tests;
