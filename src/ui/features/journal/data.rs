//! What the shell hands the Journal page: the private records, today's journal day, the
//! activity the mood associations compare against, wikilink previews of what the entries
//! link to, a few things a focus session can be linked to, and display preferences.

use crate::domain::journal::activity::ActivityByDay;
use crate::domain::journal::JournalBook;
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::domain::{Post, Profile};
use crate::ui::features::post::PostLookup;
use jiff::civil::Date;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

/// Previews of the `[[targets]]` the entries name (a post excerpt or a document title).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WikilinkPreviews(pub HashMap<String, String>);

impl PostLookup for &WikilinkPreviews {
    fn find_post(&self, _: &str) -> Option<&Post> {
        None
    }

    fn resolve_wikilink(&self, target_id: &str) -> String {
        self.0.get(target_id.trim()).cloned().unwrap_or_else(|| target_id.trim().to_string())
    }

    fn author_of(&self, _: &Post) -> &Profile {
        Profile::unknown()
    }

    fn body_tokens(&self, post: &Post) -> Rc<[BodyToken]> {
        tokenize_post_body(&post.content).into()
    }
}

/// Something a focus session can work on, offered as a chip.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkChoice {
    pub link: crate::domain::journal::focus::FocusLink,
    pub label: String,
}

/// The page's data.
#[derive(Debug, Clone, PartialEq)]
pub struct JournalData {
    pub book: Rc<JournalBook>,
    pub today: Date,
    pub activity: Rc<ActivityByDay>,
    pub wikilinks: Rc<WikilinkPreviews>,
    /// The open thought and a few vault documents, for linking a focus session.
    pub link_choices: Vec<LinkChoice>,
    pub reduced_motion: bool,
    /// Where capsule images are copied.
    pub media_dir: PathBuf,
}

impl JournalData {
    pub fn empty(today: Date) -> Self {
        Self {
            book: Rc::default(),
            today,
            activity: Rc::default(),
            wikilinks: Rc::default(),
            link_choices: Vec::new(),
            reduced_motion: false,
            media_dir: PathBuf::new(),
        }
    }
}
