//! What the sidebar can pin or remember, and its collapsible sections.

use crate::domain::{PostId, ProfileId};
use serde::{Deserialize, Serialize};

/// A pinnable item: a thread (its root post), a vault document or an agent / persona.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "id", rename_all = "snake_case")]
pub enum NavRef {
    Thread(PostId),
    Document(String),
    Profile(ProfileId),
}

/// A collapsible section of the expanded sidebar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SidebarSection {
    Pinned,
    Recent,
    Sources,
    Members,
}

impl SidebarSection {
    /// Every section, top to bottom.
    pub const ALL: [SidebarSection; 4] = [Self::Pinned, Self::Recent, Self::Sources, Self::Members];

    /// Whether the section is left out while it has no rows. Pinned and Recent only fill up
    /// with use, so their empty hints would be permanent clutter; empty Sources and Members
    /// still tell what the network lacks.
    pub const fn hides_when_empty(self) -> bool {
        matches!(self, Self::Pinned | Self::Recent)
    }

    /// Locale key of the section title.
    pub const fn title_key(self) -> &'static str {
        match self {
            Self::Pinned => "sidebar.section_pinned",
            Self::Recent => "sidebar.section_recent",
            Self::Sources => "sidebar.section_sources",
            Self::Members => "sidebar.section_members",
        }
    }

    /// Locale key of the hint shown while the section is empty.
    pub const fn empty_key(self) -> &'static str {
        match self {
            Self::Pinned => "sidebar.empty_pinned",
            Self::Recent => "sidebar.empty_recent",
            Self::Sources => "sidebar.empty_sources",
            Self::Members => "sidebar.empty_members",
        }
    }

    /// Element id of the section header.
    pub const fn header_id(self) -> &'static str {
        match self {
            Self::Pinned => "sidebar-section-pinned",
            Self::Recent => "sidebar-section-recent",
            Self::Sources => "sidebar-section-sources",
            Self::Members => "sidebar-section-members",
        }
    }
}
