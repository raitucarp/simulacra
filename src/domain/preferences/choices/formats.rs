//! General and appearance choices: the startup screen, date / time / number formats, the
//! first day of the week and the reading width.

use super::super::choice::choice_enum;

choice_enum! {
    /// The screen the main window opens on.
    StartupScreen, default: Timeline,
    [Timeline => "timeline", Knowledge => "knowledge", MindGraph => "mind_graph", AgentLab => "agent_lab"]
}

choice_enum! {
    /// How calendar dates are written.
    DateFormat, default: Iso,
    [
        /// `2026-11-17`
        Iso => "iso",
        /// `17/11/2026`
        DayMonthYear => "day_month_year",
        /// `11/17/2026`
        MonthDayYear => "month_day_year",
    ]
}

choice_enum! {
    /// 24-hour or 12-hour clock.
    TimeFormat, default: TwentyFourHour,
    [TwentyFourHour => "24h", TwelveHour => "12h"]
}

choice_enum! {
    /// Digit grouping and decimal separators.
    NumberFormat, default: Language,
    [
        /// Whatever the interface language uses.
        Language => "language",
        /// `1,234.5`
        CommaPeriod => "comma_period",
        /// `1.234,5`
        PeriodComma => "period_comma",
        /// `1 234,5`
        SpaceComma => "space_comma",
    ]
}

choice_enum! {
    /// The first column of week grids.
    FirstWeekday, default: Monday,
    [Monday => "monday", Sunday => "sunday", Saturday => "saturday"]
}

choice_enum! {
    /// Maximum width of post prose.
    ReadingWidth, default: Comfortable,
    [Narrow => "narrow", Comfortable => "comfortable", Wide => "wide"]
}
