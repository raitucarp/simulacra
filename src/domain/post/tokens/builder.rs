//! Delimiter pairing: a stack of open marks that turns delimiters into nested tokens.
//!
//! A closing delimiter pairs with the nearest open mark of its kind; marks opened after it
//! (crossing ones) and marks still open at a line break or the end fall back to literal text.

use super::{BodyToken, Mark};

/// Line break text pushed after the open marks are dropped.
const LINE_BREAK: &str = "\n";

/// One open mark (or the root) and the tokens collected inside it.
struct Frame {
    mark: Option<Mark>,
    tokens: Vec<BodyToken>,
}

impl Frame {
    fn new(mark: Option<Mark>) -> Self {
        Self { mark, tokens: Vec::new() }
    }

    /// Appends `token`, merging adjacent text runs.
    fn push(&mut self, token: BodyToken) {
        if let (Some(BodyToken::Text(last)), BodyToken::Text(text)) = (self.tokens.last_mut(), &token) {
            last.push_str(text);
            return;
        }
        if !matches!(&token, BodyToken::Text(text) if text.is_empty()) {
            self.tokens.push(token);
        }
    }
}

/// Builds the token tree of one prose segment.
pub(super) struct Builder {
    frames: Vec<Frame>,
}

impl Builder {
    pub(super) fn new() -> Self {
        Self { frames: vec![Frame::new(None)] }
    }

    fn top(&mut self) -> &mut Frame {
        self.frames.last_mut().expect("the root frame is never popped")
    }

    /// The innermost open mark, if any.
    pub(super) fn top_mark(&self) -> Option<Mark> {
        self.frames.last().and_then(|frame| frame.mark)
    }

    pub(super) fn text(&mut self, text: &str) {
        self.top().push(BodyToken::Text(text.to_string()));
    }

    pub(super) fn char(&mut self, c: char) {
        self.top().push(BodyToken::Text(c.to_string()));
    }

    pub(super) fn atom(&mut self, token: BodyToken) {
        self.top().push(token);
    }

    /// Closes the nearest open `mark` when allowed, else opens one, else writes it literally.
    pub(super) fn delimiter(&mut self, mark: Mark, can_open: bool, can_close: bool) {
        let open_at = self.frames.iter().rposition(|frame| frame.mark == Some(mark));
        match open_at {
            Some(depth) if can_close => {
                self.collapse_to(depth + 1);
                let frame = self.frames.pop().expect("the frame found above");
                if frame.tokens.is_empty() {
                    self.text(&mark.delimiter().repeat(2));
                } else {
                    self.top().push(mark.wrap(frame.tokens));
                }
            }
            _ if can_open => self.frames.push(Frame::new(Some(mark))),
            _ => self.text(mark.delimiter()),
        }
    }

    /// Drops every open mark (they stay literal) and writes the line break.
    pub(super) fn line_break(&mut self) {
        self.collapse_to(1);
        self.text(LINE_BREAK);
    }

    pub(super) fn finish(mut self) -> Vec<BodyToken> {
        self.collapse_to(1);
        self.frames.pop().map(|root| root.tokens).unwrap_or_default()
    }

    /// Pops frames until `len` remain, writing each popped mark's opener back as text.
    fn collapse_to(&mut self, len: usize) {
        while self.frames.len() > len {
            let frame = self.frames.pop().expect("more frames than `len`");
            let parent = self.top();
            if let Some(mark) = frame.mark {
                parent.push(BodyToken::Text(mark.delimiter().to_string()));
            }
            for token in frame.tokens {
                parent.push(token);
            }
        }
    }
}
