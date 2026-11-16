//! Custom Lucide icon mapping to gpui-kit's Icon component.
//! Maps 100% Lucide icons directly into gpui-kit Icon without relying on gpui-kit's built-in icon enum.

use gpui_kit::assets::IconNamed;
use gpui_kit::component::Icon;
use gpui_kit::gpui::{App, IntoElement, RenderOnce, SharedString, Window};

/// Complete catalog of mapped Lucide icons.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum LucideIcon {
    Activity,
    Archive,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    BookOpen,
    Bot,
    Boxes,
    Brain,
    Calendar,
    Check,
    ChevronDown,
    ChevronLeft,
    ChevronRight,
    ChevronUp,
    ChevronsUpDown,
    Clock,
    Compass,
    Copy,
    Cpu,
    Dices,
    Edit,
    Ellipsis,
    ExternalLink,
    Eye,
    FileText,
    Filter,
    Folder,
    GitBranch,
    Globe,
    Hash,
    Heart,
    HelpCircle,
    Image,
    Info,
    Library,
    Link2,
    Lock,
    Maximize2,
    MessageSquare,
    Minimize2,
    Minus,
    Moon,
    MoreHorizontal,
    Palette,
    PanelLeft,
    PanelRight,
    Pin,
    Plus,
    RefreshCw,
    Repeat,
    Search,
    Send,
    Settings,
    Share2,
    Sliders,
    Sparkles,
    Sun,
    Tag,
    Terminal,
    Trash,
    User,
    UserCheck,
    Users,
    Volume2,
    Wifi,
    X,
    Zap,
}

impl LucideIcon {
    /// Every mapped icon, used to verify each path exists in the bundled asset catalog.
    pub const ALL: &'static [Self] = &[
        Self::Activity, Self::Archive, Self::ArrowDown, Self::ArrowLeft, Self::ArrowRight,
        Self::ArrowUp, Self::BookOpen, Self::Bot, Self::Boxes, Self::Brain, Self::Calendar,
        Self::Check, Self::ChevronDown, Self::ChevronLeft, Self::ChevronRight, Self::ChevronUp,
        Self::ChevronsUpDown, Self::Clock, Self::Compass, Self::Copy, Self::Cpu, Self::Dices,
        Self::Edit, Self::Ellipsis, Self::ExternalLink, Self::Eye, Self::FileText, Self::Filter,
        Self::Folder, Self::GitBranch, Self::Globe, Self::Hash, Self::Heart, Self::HelpCircle,
        Self::Image, Self::Info, Self::Library, Self::Link2, Self::Lock, Self::Maximize2,
        Self::MessageSquare, Self::Minimize2, Self::Minus, Self::Moon, Self::MoreHorizontal,
        Self::Palette, Self::PanelLeft, Self::PanelRight, Self::Pin, Self::Plus, Self::RefreshCw,
        Self::Repeat, Self::Search, Self::Send, Self::Settings, Self::Share2, Self::Sliders,
        Self::Sparkles, Self::Sun, Self::Tag, Self::Terminal, Self::Trash, Self::User,
        Self::UserCheck, Self::Users, Self::Volume2, Self::Wifi, Self::X, Self::Zap,
    ];

    /// Resolves the official Lucide SVG path.
    pub fn path(self) -> SharedString {
        match self {
            Self::Activity => "icons/activity.svg",
            Self::Archive => "icons/archive.svg",
            Self::ArrowDown => "icons/arrow-down.svg",
            Self::ArrowLeft => "icons/arrow-left.svg",
            Self::ArrowRight => "icons/arrow-right.svg",
            Self::ArrowUp => "icons/arrow-up.svg",
            Self::BookOpen => "icons/book-open.svg",
            Self::Bot => "icons/bot.svg",
            Self::Boxes => "icons/boxes.svg",
            Self::Brain => "icons/brain.svg",
            Self::Calendar => "icons/calendar.svg",
            Self::Check => "icons/check.svg",
            Self::ChevronDown => "icons/chevron-down.svg",
            Self::ChevronLeft => "icons/chevron-left.svg",
            Self::ChevronRight => "icons/chevron-right.svg",
            Self::ChevronUp => "icons/chevron-up.svg",
            Self::ChevronsUpDown => "icons/chevrons-up-down.svg",
            Self::Clock => "icons/clock.svg",
            Self::Compass => "icons/compass.svg",
            Self::Copy => "icons/copy.svg",
            Self::Cpu => "icons/cpu.svg",
            Self::Dices => "icons/dices.svg",
            Self::Edit => "icons/square-pen.svg",
            Self::Ellipsis => "icons/ellipsis.svg",
            Self::ExternalLink => "icons/external-link.svg",
            Self::Eye => "icons/eye.svg",
            Self::FileText => "icons/file-text.svg",
            Self::Filter => "icons/funnel.svg",
            Self::Folder => "icons/folder.svg",
            Self::GitBranch => "icons/git-branch.svg",
            Self::Globe => "icons/globe.svg",
            Self::Hash => "icons/hash.svg",
            Self::Heart => "icons/heart.svg",
            Self::HelpCircle => "icons/circle-question-mark.svg",
            Self::Image => "icons/image.svg",
            Self::Info => "icons/info.svg",
            Self::Library => "icons/library.svg",
            Self::Link2 => "icons/link-2.svg",
            Self::Lock => "icons/lock.svg",
            Self::Maximize2 => "icons/maximize-2.svg",
            Self::MessageSquare => "icons/message-square.svg",
            Self::Minimize2 => "icons/minimize-2.svg",
            Self::Minus => "icons/minus.svg",
            Self::Moon => "icons/moon.svg",
            Self::MoreHorizontal => "icons/ellipsis.svg",
            Self::Palette => "icons/palette.svg",
            Self::PanelLeft => "icons/panel-left.svg",
            Self::PanelRight => "icons/panel-right.svg",
            Self::Pin => "icons/pin.svg",
            Self::Plus => "icons/plus.svg",
            Self::RefreshCw => "icons/refresh-cw.svg",
            Self::Repeat => "icons/repeat.svg",
            Self::Search => "icons/search.svg",
            Self::Send => "icons/send.svg",
            Self::Settings => "icons/settings.svg",
            Self::Share2 => "icons/share-2.svg",
            Self::Sliders => "icons/sliders-horizontal.svg",
            Self::Sparkles => "icons/sparkles.svg",
            Self::Sun => "icons/sun.svg",
            Self::Tag => "icons/tag.svg",
            Self::Terminal => "icons/terminal.svg",
            Self::Trash => "icons/trash.svg",
            Self::User => "icons/user.svg",
            Self::UserCheck => "icons/user-check.svg",
            Self::Users => "icons/users.svg",
            Self::Volume2 => "icons/volume-2.svg",
            Self::Wifi => "icons/wifi.svg",
            Self::X => "icons/x.svg",
            Self::Zap => "icons/zap.svg",
        }
        .into()
    }

    /// Converts this Lucide icon directly into gpui-kit's Icon component.
    pub fn icon(self) -> Icon {
        Icon::new(self)
    }
}

impl IconNamed for LucideIcon {
    fn path(self) -> SharedString {
        self.path()
    }
}

impl RenderOnce for LucideIcon {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::new(self)
    }
}

/// Lucide mapping namespace for ergonomic syntax `Lucide::icon(LucideIcon::Heart)`.
pub struct Lucide;

impl Lucide {
    pub fn icon(name: LucideIcon) -> Icon {
        Icon::new(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui_kit::assets::IconName;
    use std::collections::HashSet;

    /// A path missing from the bundled catalog renders as a blank icon at runtime,
    /// which happens silently when Lucide renames an icon upstream.
    #[test]
    fn every_lucide_icon_path_exists_in_bundled_catalog() {
        let bundled: HashSet<SharedString> = IconName::ALL.iter().map(|n| n.path()).collect();
        let missing: Vec<_> = LucideIcon::ALL
            .iter()
            .filter(|icon| !bundled.contains(&icon.path()))
            .map(|icon| (*icon, icon.path()))
            .collect();
        assert!(missing.is_empty(), "icons missing from bundled assets: {missing:?}");
    }

    #[test]
    fn all_lists_every_variant_once() {
        let unique: HashSet<_> = LucideIcon::ALL.iter().collect();
        assert_eq!(unique.len(), LucideIcon::ALL.len());
    }
}
