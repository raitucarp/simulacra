//! Writing and import choices: the identity posts go out under, what clicking a wikilink or
//! a hashtag does, and how imported text is cut into posts.

use super::super::choice::choice_enum;
use jiff::Timestamp;

choice_enum! {
    /// Who a new post (or an imported passage) is published as.
    PostIdentity, default: Yourself,
    [Yourself => "yourself", Pseudonym => "pseudonym", Anonymous => "anonymous"]
}

choice_enum! {
    /// What clicking a `[[wikilink]]` pill does.
    WikilinkClick, default: Preview,
    [Preview => "preview", Inspector => "inspector"]
}

choice_enum! {
    /// What clicking a `#hashtag` does.
    HashtagClick, default: SearchFeed,
    [SearchFeed => "search_feed", PlainText => "plain_text"]
}

choice_enum! {
    /// The unit an imported document is split into.
    ImportSegmentation, default: Paragraph,
    [Paragraph => "paragraph", Sentence => "sentence", Chapter => "chapter"]
}

choice_enum! {
    /// How long after a post goes out it moves to the archive (Y6).
    ArchiveAfter, default: Never,
    [Never => "never", Week => "week", Month => "month", Quarter => "quarter", Year => "year"]
}

/// Days of the auto-archive delays.
const WEEK_DAYS: i64 = 7;
const MONTH_DAYS: i64 = 30;
const QUARTER_DAYS: i64 = 90;
const YEAR_DAYS: i64 = 365;

impl ArchiveAfter {
    /// Days after posting (`None`: never).
    pub const fn days(self) -> Option<i64> {
        match self {
            Self::Never => None,
            Self::Week => Some(WEEK_DAYS),
            Self::Month => Some(MONTH_DAYS),
            Self::Quarter => Some(QUARTER_DAYS),
            Self::Year => Some(YEAR_DAYS),
        }
    }

    /// When a post that went out at `posted` moves to the archive.
    pub fn archive_at(self, posted: Timestamp) -> Option<Timestamp> {
        self.days().and_then(|days| posted.checked_add(crate::time::days(days)).ok())
    }

    /// The choice that archives a post that went out at `posted` at `archive_at`, if any.
    pub fn of(posted: Timestamp, archive_at: Option<Timestamp>) -> Option<Self> {
        Self::ALL.iter().copied().find(|after| after.archive_at(posted) == archive_at)
    }
}
