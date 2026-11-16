//! Declarative Lucide catalog: one table generates the enum, `ALL` and `path()`.

use gpui_kit::gpui::SharedString;

/// Expands a `Variant => "icons/<name>.svg"` table into the icon enum and its lookups.
macro_rules! lucide_catalog {
    ($($variant:ident => $path:literal),* $(,)?) => {
        /// Complete catalog of mapped Lucide icons.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
        pub enum LucideIcon {
            $($variant,)*
        }

        impl LucideIcon {
            /// Every mapped icon, used to verify each path exists in the bundled asset catalog.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)*];

            /// Resolves the official Lucide SVG path.
            pub fn path(self) -> SharedString {
                match self {
                    $(Self::$variant => $path,)*
                }
                .into()
            }
        }
    };
}

lucide_catalog! {
    Activity => "icons/activity.svg",
    Archive => "icons/archive.svg",
    ArrowDown => "icons/arrow-down.svg",
    ArrowLeft => "icons/arrow-left.svg",
    ArrowRight => "icons/arrow-right.svg",
    ArrowUp => "icons/arrow-up.svg",
    BookOpen => "icons/book-open.svg",
    Bot => "icons/bot.svg",
    Boxes => "icons/boxes.svg",
    Brain => "icons/brain.svg",
    Calendar => "icons/calendar.svg",
    Check => "icons/check.svg",
    ChevronDown => "icons/chevron-down.svg",
    ChevronLeft => "icons/chevron-left.svg",
    ChevronRight => "icons/chevron-right.svg",
    ChevronUp => "icons/chevron-up.svg",
    ChevronsUpDown => "icons/chevrons-up-down.svg",
    Clock => "icons/clock.svg",
    Compass => "icons/compass.svg",
    Copy => "icons/copy.svg",
    Cpu => "icons/cpu.svg",
    Dices => "icons/dices.svg",
    Edit => "icons/square-pen.svg",
    Ellipsis => "icons/ellipsis.svg",
    ExternalLink => "icons/external-link.svg",
    Eye => "icons/eye.svg",
    FileText => "icons/file-text.svg",
    Filter => "icons/funnel.svg",
    Folder => "icons/folder.svg",
    GitBranch => "icons/git-branch.svg",
    Globe => "icons/globe.svg",
    Hash => "icons/hash.svg",
    Heart => "icons/heart.svg",
    HelpCircle => "icons/circle-question-mark.svg",
    Image => "icons/image.svg",
    Info => "icons/info.svg",
    Library => "icons/library.svg",
    Link2 => "icons/link-2.svg",
    Lock => "icons/lock.svg",
    Maximize2 => "icons/maximize-2.svg",
    MessageSquare => "icons/message-square.svg",
    Minimize2 => "icons/minimize-2.svg",
    Minus => "icons/minus.svg",
    Moon => "icons/moon.svg",
    MoreHorizontal => "icons/ellipsis.svg",
    Palette => "icons/palette.svg",
    PanelLeft => "icons/panel-left.svg",
    PanelRight => "icons/panel-right.svg",
    Pin => "icons/pin.svg",
    Plus => "icons/plus.svg",
    RefreshCw => "icons/refresh-cw.svg",
    Repeat => "icons/repeat.svg",
    Search => "icons/search.svg",
    Send => "icons/send.svg",
    Settings => "icons/settings.svg",
    Share2 => "icons/share-2.svg",
    Sliders => "icons/sliders-horizontal.svg",
    Sparkles => "icons/sparkles.svg",
    Sun => "icons/sun.svg",
    Tag => "icons/tag.svg",
    Terminal => "icons/terminal.svg",
    Trash => "icons/trash.svg",
    User => "icons/user.svg",
    UserCheck => "icons/user-check.svg",
    Users => "icons/users.svg",
    Volume2 => "icons/volume-2.svg",
    Wifi => "icons/wifi.svg",
    X => "icons/x.svg",
    Zap => "icons/zap.svg",
}
