//! Reading status and progress of a document.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// Where the reader is with a document.
    ReadingStatus {
        Unread => "unread",
        Reading => "reading",
        Read => "read",
    }
}

impl Default for ReadingStatus {
    fn default() -> Self {
        Self::Unread
    }
}

/// Progress of a finished document, in percent.
pub const PROGRESS_COMPLETE: u8 = 100;

/// Reading state: a status and how far along (0–100 %).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Reading {
    pub status: ReadingStatus,
    pub progress: u8,
}

impl Reading {
    /// The state at `progress` percent (clamped): none is unread, all is read.
    pub fn at(progress: u8) -> Self {
        let progress = progress.min(PROGRESS_COMPLETE);
        let status = match progress {
            0 => ReadingStatus::Unread,
            PROGRESS_COMPLETE => ReadingStatus::Read,
            _ => ReadingStatus::Reading,
        };
        Self { status, progress }
    }

    /// The state with `status` set explicitly (progress follows: 0 for unread, 100 for read,
    /// kept for reading).
    pub fn with_status(self, status: ReadingStatus) -> Self {
        let progress = match status {
            ReadingStatus::Unread => 0,
            ReadingStatus::Reading => self.progress.min(PROGRESS_COMPLETE - 1),
            ReadingStatus::Read => PROGRESS_COMPLETE,
        };
        Self { status, progress }
    }

    /// Progress only moves forward while reading, so scrolling back does not lose it.
    pub fn advanced_to(self, progress: u8) -> Self {
        if progress > self.progress {
            Self::at(progress)
        } else {
            self
        }
    }
}
