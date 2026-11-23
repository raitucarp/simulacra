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
    CircleAlert => "icons/circle-alert.svg",
    CircleCheck => "icons/circle-check.svg",
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
    ImagePlus => "icons/image-plus.svg",
    EyeOff => "icons/eye-off.svg",
    Info => "icons/info.svg",
    Keyboard => "icons/keyboard.svg",
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
    SunMoon => "icons/sun-moon.svg",
    Tag => "icons/tag.svg",
    Terminal => "icons/terminal.svg",
    Trash => "icons/trash.svg",
    User => "icons/user.svg",
    UserCheck => "icons/user-check.svg",
    Undo2 => "icons/undo-2.svg",
    Redo2 => "icons/redo-2.svg",
    Users => "icons/users.svg",
    Volume2 => "icons/volume-2.svg",
    Wifi => "icons/wifi.svg",
    X => "icons/x.svg",
    Zap => "icons/zap.svg",
    // sb A2: navigation rail, "New" menu, pins and the command palette
    Command => "icons/command.svg",
    CornerDownLeft => "icons/corner-down-left.svg",
    FileUp => "icons/file-up.svg",
    MessageSquarePlus => "icons/message-square-plus.svg",
    PanelLeftClose => "icons/panel-left-close.svg",
    PanelLeftOpen => "icons/panel-left-open.svg",
    PinOff => "icons/pin-off.svg",
    Star => "icons/star.svg",
    StickyNote => "icons/sticky-note.svg",
    UserPlus => "icons/user-plus.svg",
    // sb B1: Knowledge Vault (view switcher, sort, citations, catalogue fields)
    LayoutGrid => "icons/layout-grid.svg",
    List => "icons/list.svg",
    Bookmark => "icons/bookmark.svg",
    Quote => "icons/quote.svg",
    ArrowUpDown => "icons/arrow-up-down.svg",
    BookMarked => "icons/book-marked.svg",
    Languages => "icons/languages.svg",
    MapPin => "icons/map-pin.svg",
    Feather => "icons/feather.svg", Inbox => "icons/inbox.svg", Pilcrow => "icons/pilcrow.svg", VenetianMask => "icons/venetian-mask.svg", // Import and capture
    Network => "icons/network.svg", Shield => "icons/shield.svg", Scale => "icons/scale.svg", Puzzle => "icons/puzzle.svg", Swords => "icons/swords.svg", Gauge => "icons/gauge.svg", GitCompareArrows => "icons/git-compare-arrows.svg", FlaskConical => "icons/flask-conical.svg", ListTree => "icons/list-tree.svg", ChevronsDownUp => "icons/chevrons-down-up.svg", // sb C2: post actions and the outline view
    // sb C1: review page, quizzes and statistics
    GraduationCap => "icons/graduation-cap.svg",
    Layers => "icons/layers.svg",
    ListOrdered => "icons/list-ordered.svg",
    ChartColumn => "icons/chart-column.svg",
    Flame => "icons/flame.svg",
    Target => "icons/target.svg",
    Lightbulb => "icons/lightbulb.svg",
    RotateCcw => "icons/rotate-ccw.svg",
    CircleX => "icons/circle-x.svg",
    WandSparkles => "icons/wand-sparkles.svg",
    TextCursorInput => "icons/text-cursor-input.svg",
    Trophy => "icons/trophy.svg",
    Pause => "icons/pause.svg",
    Play => "icons/play.svg",
    MessageSquareReply => "icons/message-square-reply.svg", CircleStop => "icons/circle-stop.svg", Eraser => "icons/eraser.svg", ArrowLeftRight => "icons/arrow-left-right.svg", ListChecks => "icons/list-checks.svg", // sb C1b
    NotebookPen => "icons/notebook-pen.svg", HeartPulse => "icons/heart-pulse.svg", Timer => "icons/timer.svg", Mail => "icons/mail.svg", MailOpen => "icons/mail-open.svg", LockOpen => "icons/lock-open.svg", Shuffle => "icons/shuffle.svg", Paperclip => "icons/paperclip.svg", HandHeart => "icons/hand-heart.svg", // sb E3
    Waypoints => "icons/waypoints.svg", Atom => "icons/atom.svg", Percent => "icons/percent.svg", ChartScatter => "icons/chart-scatter.svg", Split => "icons/split.svg", CalendarClock => "icons/calendar-clock.svg", GripVertical => "icons/grip-vertical.svg", // sb E1
    FaceSlightlySmilingPlus => "icons/face-slightly-smiling-plus.svg", NotebookText => "icons/notebook-text.svg", CalendarDays => "icons/calendar-days.svg", CalendarRange => "icons/calendar-range.svg", // epic R: add a reaction; epic W: the views
    Save => "icons/save.svg", Bold => "icons/bold.svg", Italic => "icons/italic.svg", Underline => "icons/underline.svg", Code => "icons/code.svg", // sb F1; posts B: composer formatting
    Pencil => "icons/pencil.svg", RotateCcwClock => "icons/rotate-ccw-clock.svg", BookmarkCheck => "icons/bookmark-check.svg", FolderPlus => "icons/folder-plus.svg", Download => "icons/download.svg", KeyRound => "icons/key-round.svg", ShieldCheck => "icons/shield-check.svg", ArchiveRestore => "icons/archive-restore.svg", Package => "icons/package.svg", LockKeyhole => "icons/lock-keyhole.svg", FileDiff => "icons/file-diff.svg", // sb F2
}
