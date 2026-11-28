//! What each view looks like in a switcher: its icon and its translated name.

use crate::domain::views::PostView;
use crate::ui::icons::LucideIcon;
use rust_i18n::t;

/// The icon of `view` in a switcher.
pub fn view_icon(view: PostView) -> LucideIcon {
    match view {
        PostView::Cards => LucideIcon::LayoutGrid,
        PostView::Outline => LucideIcon::ListTree,
        PostView::Argument => LucideIcon::Waypoints,
        PostView::RapidLog => LucideIcon::NotebookText,
        PostView::Raw => LucideIcon::FileText,
        PostView::Calendar => LucideIcon::CalendarDays,
        PostView::Future => LucideIcon::CalendarRange,
        PostView::Board => LucideIcon::SquareKanban,
        PostView::Garden => LucideIcon::Sprout,
        PostView::Branches => LucideIcon::GitBranch,
        PostView::Series => LucideIcon::ListOrdered,
        PostView::StillTrue => LucideIcon::RotateCcwClock,
        PostView::LensFan => LucideIcon::Layers,
        PostView::Glossary => LucideIcon::BookOpen,
        PostView::OpenQuestions => LucideIcon::HelpCircle,
        PostView::MindChanges => LucideIcon::RefreshCw,
    }
}

/// The locale key of `view`'s name.
pub fn view_label_key(view: PostView) -> &'static str {
    match view {
        PostView::Cards => "feed.outline.show_cards",
        PostView::Outline => "feed.outline.show_outline",
        PostView::Argument => "thinking.argument.show",
        PostView::RapidLog => "views.rapid_log.show",
        PostView::Raw => "views.raw.show",
        PostView::Calendar => "views.calendar.show",
        PostView::Future => "views.future.show",
        PostView::Board => "views.board.show",
        PostView::Garden => "views.garden.show",
        PostView::Branches => "views.branches.show",
        PostView::Series => "views.series.show",
        PostView::StillTrue => "views.still_true.show",
        PostView::LensFan => "views.lens_fan.show",
        PostView::Glossary => "views.glossary.show",
        PostView::OpenQuestions => "views.open_questions.show",
        PostView::MindChanges => "views.mind_changes.show",
    }
}

/// The translated name of `view` (its switcher tooltip).
pub fn view_label(view: PostView) -> String {
    t!(view_label_key(view)).to_string()
}

/// The section of the view menu a view is listed in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViewSection {
    /// The same posts, laid out another way (cards, outline, calendar, board…).
    Layouts,
    /// One kind of thought singled out (forks, lens fans, claims to recheck…).
    Lenses,
    /// What the posts gathered (garden, series, glossary).
    Collections,
}

impl ViewSection {
    pub const ALL: [Self; 3] = [Self::Layouts, Self::Lenses, Self::Collections];

    /// Locale key of the section heading.
    pub const fn title_key(self) -> &'static str {
        match self {
            Self::Layouts => "views.menu.section.layouts",
            Self::Lenses => "views.menu.section.lenses",
            Self::Collections => "views.menu.section.collections",
        }
    }
}

/// Where `view` is listed in the view menu.
pub const fn view_section(view: PostView) -> ViewSection {
    match view {
        PostView::Cards | PostView::Outline | PostView::Argument | PostView::RapidLog | PostView::Raw | PostView::Calendar | PostView::Future | PostView::Board => ViewSection::Layouts,
        PostView::Branches | PostView::LensFan | PostView::StillTrue | PostView::OpenQuestions | PostView::MindChanges => ViewSection::Lenses,
        PostView::Garden | PostView::Series | PostView::Glossary => ViewSection::Collections,
    }
}

/// The locale key of `view`'s entry in the view menu ("Show as a calendar"); the stored
/// [`PostView`] key is unchanged.
pub fn view_menu_key(view: PostView) -> String {
    format!("views.menu.{}", view.key())
}
