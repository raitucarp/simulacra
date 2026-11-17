//! [`AvatarKind`]: the four avatar styles the Appearance step's switch offers.

use crate::domain::portrait::AvatarMode;
use crate::ui::icons::LucideIcon;

/// The kinds of avatar, as offered by the mode switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AvatarKind {
    Portrait,
    Form,
    Emblem,
    Photo,
}

impl AvatarKind {
    pub(super) const ALL: [Self; 4] = [Self::Portrait, Self::Form, Self::Emblem, Self::Photo];

    pub(super) fn of(mode: Option<&AvatarMode>) -> Option<Self> {
        mode.map(|mode| match mode {
            AvatarMode::Pcg(_) => Self::Portrait,
            AvatarMode::Form(_) => Self::Form,
            AvatarMode::Emblem(_) => Self::Emblem,
            AvatarMode::Photo { .. } => Self::Photo,
        })
    }

    /// `portrait.mode_*` label key.
    pub(super) fn label_key(self) -> &'static str {
        match self {
            Self::Portrait => "portrait.mode_pcg",
            Self::Form => "portrait.mode_form",
            Self::Emblem => "portrait.mode_emblem",
            Self::Photo => "portrait.mode_photo",
        }
    }

    pub(super) fn icon(self) -> LucideIcon {
        match self {
            Self::Portrait => LucideIcon::User,
            Self::Form => LucideIcon::Boxes,
            Self::Emblem => LucideIcon::Sparkles,
            Self::Photo => LucideIcon::Image,
        }
    }
}
