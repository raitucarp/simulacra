//! The chips after a to-do's text: priority, due day (in the danger colour once overdue, the
//! warning colour on the day), repeat rule, contexts, estimate and delegate.

use super::row::RowContext;
use crate::domain::todo::entries::TodoEntry;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use jiff::civil::Date;
use jiff::ToSpan;
use rust_i18n::t;

const MINUTES_PER_HOUR: u32 = 60;
/// The `!` a priority chip repeats.
const PRIORITY_MARK: &str = "!";

/// How a due day stands against today.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum DueTone {
    Overdue,
    Today,
    Later,
}

/// The tone of a due chip: overdue or due today only while the item is still to do.
pub(super) fn due_tone(due: Date, today: Date, closed: bool) -> DueTone {
    match due.cmp(&today) {
        std::cmp::Ordering::Less if !closed => DueTone::Overdue,
        std::cmp::Ordering::Equal if !closed => DueTone::Today,
        _ => DueTone::Later,
    }
}

/// "Today", "Tomorrow", "Yesterday", else the weekday and date.
pub fn due_label(due: Date, today: Date) -> String {
    if due == today {
        t!("todo.due.today").to_string()
    } else if today.checked_add(1.day()).ok() == Some(due) {
        t!("todo.due.tomorrow").to_string()
    } else if today.checked_sub(1.day()).ok() == Some(due) {
        t!("todo.due.yesterday").to_string()
    } else {
        crate::ui::features::journal::text::day_label(due)
    }
}

/// "25 min", "2 h", "1 h 30 min".
pub(super) fn estimate_label(minutes: u32) -> String {
    match (minutes / MINUTES_PER_HOUR, minutes % MINUTES_PER_HOUR) {
        (0, minutes) => t!("todo.estimate.minutes", minutes = minutes).to_string(),
        (hours, 0) => t!("todo.estimate.hours", hours = hours).to_string(),
        (hours, minutes) => t!("todo.estimate.hours_minutes", hours = hours, minutes = minutes).to_string(),
    }
}

fn chip(icon: Option<LucideIcon>, label: String, colour: Rgba, theme: &ThemeTokens) -> AnyElement {
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .text_size(theme.font_size_xs)
        .text_color(colour)
        .children(icon.map(|icon| Icon::new(icon).size(theme.icon_size_xs).text_color(colour)))
        .child(label)
        .into_any_element()
}

fn priority_colour(level: u8, theme: &ThemeTokens) -> Rgba {
    match level {
        1 => theme.info,
        2 => theme.warning,
        _ => theme.accent_danger,
    }
}

/// The colour of a due chip.
pub(super) fn due_colour(tone: DueTone, theme: &ThemeTokens) -> Rgba {
    match tone {
        DueTone::Overdue => theme.accent_danger,
        DueTone::Today => theme.warning,
        DueTone::Later => theme.text_secondary,
    }
}

/// Every chip of `entry`, in a fixed order.
pub(super) fn todo_chips(entry: &TodoEntry, context: &RowContext<'_>) -> Vec<AnyElement> {
    let (theme, meta, muted) = (context.theme, &entry.item.meta, context.theme.text_secondary);
    let mut chips = Vec::new();
    if !entry.children.is_empty() {
        let progress = t!("todo.progress", done = entry.settled_children, total = entry.children.len()).to_string(); // D7: done once every child is
        let colour = if entry.settled_children == entry.children.len() { theme.accent_success } else { muted };
        chips.push(chip(Some(LucideIcon::ListChecks), progress, colour, theme));
    }
    if let Some(level) = meta.priority {
        chips.push(chip(None, PRIORITY_MARK.repeat(usize::from(level)), priority_colour(level, theme), theme));
    }
    if let Some(due) = entry.due(context.written) {
        let colour = due_colour(due_tone(due, context.today, entry.state.is_closed()), theme);
        chips.push(chip(Some(LucideIcon::Calendar), due_label(due, context.today), colour, theme));
    }
    chips.extend(meta.repeat.iter().map(|rule| chip(Some(LucideIcon::Repeat), rule.clone(), muted, theme)));
    if entry.completions > 0 {
        chips.push(chip(Some(LucideIcon::CircleCheck), crate::i18n::t_count("todo.repeat.done", entry.completions as usize), theme.accent_success, theme)); // D8
    }
    chips.extend(meta.contexts.iter().map(|name| chip(Some(LucideIcon::MapPin), name.clone(), muted, theme)));
    chips.extend(meta.estimate_minutes.map(|minutes| chip(Some(LucideIcon::Timer), estimate_label(minutes), muted, theme)));
    chips.extend(meta.delegate.iter().map(|name| chip(Some(LucideIcon::UserCheck), name.clone(), theme.accent_agent, theme)));
    chips
}
