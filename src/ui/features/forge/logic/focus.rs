//! Keyboard cursor of an option grid laid out in rows of a fixed number of columns.

/// Keys that pick the option under the cursor.
pub const ACTIVATE_KEYS: [&str; 2] = ["enter", "space"];

/// A cursor movement key.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavKey {
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
}

impl NavKey {
    /// The movement for a gpui key name (`"left"`, `"down"`, …), if it is one.
    pub fn from_key(key: &str) -> Option<Self> {
        Some(match key {
            "left" => Self::Left,
            "right" => Self::Right,
            "up" => Self::Up,
            "down" => Self::Down,
            "home" => Self::Home,
            "end" => Self::End,
            _ => return None,
        })
    }
}

/// Position of the highlighted card among `len` cards in rows of `columns`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridCursor {
    pub index: usize,
    pub len: usize,
    pub columns: usize,
}

impl GridCursor {
    /// A cursor at `index` (clamped into the grid).
    pub fn new(index: usize, len: usize, columns: usize) -> Self {
        Self { index: index.min(len.saturating_sub(1)), len, columns: columns.max(1) }
    }

    /// The cursor after pressing `key`: left / right wrap through the whole grid,
    /// up / down move a row and stop at the edges.
    pub fn moved(self, key: NavKey) -> Self {
        if self.len == 0 {
            return self;
        }
        let last = self.len - 1;
        let index = match key {
            NavKey::Left => self.index.checked_sub(1).unwrap_or(last),
            NavKey::Right => if self.index >= last { 0 } else { self.index + 1 },
            NavKey::Up => self.index.checked_sub(self.columns).unwrap_or(self.index),
            NavKey::Down => {
                let below = self.index + self.columns;
                if below <= last { below } else { self.index }
            }
            NavKey::Home => 0,
            NavKey::End => last,
        };
        Self { index, ..self }
    }
}
