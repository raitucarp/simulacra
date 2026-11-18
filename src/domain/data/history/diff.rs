//! Word-level diff of two texts: both are split into tokens (words, runs of whitespace and
//! single punctuation marks), the common head and tail are kept, and the middle is aligned
//! by a longest common subsequence. A middle too large for the table ([`MAX_DIFF_CELLS`])
//! is reported as one deletion and one insertion, so a huge rewrite never stalls the UI.

/// Largest `old × new` token table aligned exactly.
pub const MAX_DIFF_CELLS: usize = 1_000_000;

/// One run of the diff, in reading order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffOp {
    Same(String),
    Added(String),
    Removed(String),
}

/// Words added and removed (whitespace and punctuation are not counted).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct DiffStats {
    pub added: usize,
    pub removed: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Class {
    Word,
    Space,
    Mark,
}

fn class_of(c: char) -> Class {
    if c.is_alphanumeric() || c == '_' || c == '\'' {
        Class::Word
    } else if c.is_whitespace() {
        Class::Space
    } else {
        Class::Mark
    }
}

/// `text` as tokens: maximal word and whitespace runs, one token per other character.
pub fn tokens(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0;
    let mut previous: Option<Class> = None;
    for (at, c) in text.char_indices() {
        let class = class_of(c);
        let joins = previous == Some(class) && class != Class::Mark;
        if !joins && at > start {
            out.push(&text[start..at]);
            start = at;
        }
        previous = Some(class);
    }
    if start < text.len() {
        out.push(&text[start..]);
    }
    out
}

/// The diff turning `old` into `new`, adjacent runs of one kind merged.
pub fn word_diff(old: &str, new: &str) -> Vec<DiffOp> {
    let (a, b) = (tokens(old), tokens(new));
    let head = a.iter().zip(&b).take_while(|(x, y)| x == y).count();
    let tail = a[head..].iter().rev().zip(b[head..].iter().rev()).take_while(|(x, y)| x == y).count();
    let (middle_a, middle_b) = (&a[head..a.len() - tail], &b[head..b.len() - tail]);
    let ops = a[..head].iter().map(|t| DiffOp::Same(t.to_string()))
        .chain(align(middle_a, middle_b))
        .chain(a[a.len() - tail..].iter().map(|t| DiffOp::Same(t.to_string())));
    merged(ops)
}

/// The middle's edit script (an exact LCS when the table is small enough).
fn align(a: &[&str], b: &[&str]) -> Vec<DiffOp> {
    if a.len().saturating_mul(b.len()) > MAX_DIFF_CELLS {
        let removed = a.iter().map(|t| DiffOp::Removed(t.to_string()));
        return removed.chain(b.iter().map(|t| DiffOp::Added(t.to_string()))).collect();
    }
    let width = b.len() + 1;
    let mut table = vec![0u32; (a.len() + 1) * width];
    for i in (0..a.len()).rev() {
        for j in (0..b.len()).rev() {
            table[i * width + j] = if a[i] == b[j] { table[(i + 1) * width + j + 1] + 1 } else { table[(i + 1) * width + j].max(table[i * width + j + 1]) };
        }
    }
    let (mut i, mut j, mut ops) = (0, 0, Vec::new());
    while i < a.len() || j < b.len() {
        if i < a.len() && j < b.len() && a[i] == b[j] {
            ops.push(DiffOp::Same(a[i].to_string()));
            (i, j) = (i + 1, j + 1);
        } else if i < a.len() && (j == b.len() || table[(i + 1) * width + j] >= table[i * width + j + 1]) {
            ops.push(DiffOp::Removed(a[i].to_string()));
            i += 1;
        } else {
            ops.push(DiffOp::Added(b[j].to_string()));
            j += 1;
        }
    }
    ops
}

/// Joins neighbouring runs of the same kind.
fn merged(ops: impl Iterator<Item = DiffOp>) -> Vec<DiffOp> {
    ops.fold(Vec::new(), |mut out: Vec<DiffOp>, op| {
        match (out.last_mut(), op) {
            (Some(DiffOp::Same(text)), DiffOp::Same(more)) | (Some(DiffOp::Added(text)), DiffOp::Added(more)) | (Some(DiffOp::Removed(text)), DiffOp::Removed(more)) => text.push_str(&more),
            (_, op) => out.push(op),
        }
        out
    })
}

fn word_count(text: &str) -> usize {
    tokens(text).iter().filter(|t| t.chars().next().is_some_and(|c| class_of(c) == Class::Word)).count()
}

/// How many words `ops` add and remove.
pub fn diff_stats(ops: &[DiffOp]) -> DiffStats {
    ops.iter().fold(DiffStats::default(), |stats, op| match op {
        DiffOp::Added(text) => DiffStats { added: stats.added + word_count(text), ..stats },
        DiffOp::Removed(text) => DiffStats { removed: stats.removed + word_count(text), ..stats },
        DiffOp::Same(_) => stats,
    })
}
