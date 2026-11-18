//! Localized labels of the Review page: intervals, due dates, grades, kinds and states.

use crate::domain::post::SourceLocator;
use crate::domain::review::schedule::{ItemSchedule, SAME_DAY};
use crate::domain::review::{day::days_between, Grade};
use crate::i18n::t_count;
use chrono::NaiveDate;
use rust_i18n::t;

const DAYS_PER_MONTH: f64 = 30.44;
const DAYS_PER_YEAR: f64 = 365.25;
const MONTH_FROM_DAYS: u32 = 30;
const YEAR_FROM_DAYS: u32 = 365;
const ONE_DECIMAL_BELOW: f64 = 10.0;
const PLACE_SEPARATOR: &str = " · ";

/// A translated key.
pub fn tr(key: &str) -> String {
    t!(key).to_string()
}

/// `value` with one decimal below ten ("1.5"), whole above ("14").
fn short_number(value: f64) -> String {
    if value < ONE_DECIMAL_BELOW && (value - value.round()).abs() >= 0.05 { format!("{value:.1}") } else { format!("{}", value.round() as u64) }
}

/// The compact interval a grade button promises: "Soon", "4 d", "2.5 mo", "1.2 y".
pub fn interval_label(days: u32) -> String {
    match days {
        SAME_DAY => tr("review.interval.soon"),
        d if d < MONTH_FROM_DAYS => t!("review.interval.days", count = d).to_string(),
        d if d < YEAR_FROM_DAYS => t!("review.interval.months", count = short_number(f64::from(d) / DAYS_PER_MONTH)).to_string(),
        d => t!("review.interval.years", count = short_number(f64::from(d) / DAYS_PER_YEAR)).to_string(),
    }
}

/// When an item is due, from `today`.
pub fn due_label(schedule: &ItemSchedule, today: NaiveDate) -> String {
    if schedule.is_new() {
        return tr("review.due.new");
    }
    match (schedule.due < today, schedule.due == today) {
        (true, _) => t_count("review.due.overdue", days_between(schedule.due, today) as usize),
        (_, true) => tr("review.due.today"),
        _ => t_count("review.due.in_days", days_between(today, schedule.due) as usize),
    }
}

pub fn grade_label(grade: Grade) -> String {
    tr(&format!("review.grade.{}", grade.key()))
}

/// What a grade means, shown as its tooltip.
pub fn grade_hint(grade: Grade) -> String {
    tr(&format!("review.grade.{}_hint", grade.key()))
}

/// "question" / "answer" / "connections" item kind label (the subject's key).
pub fn kind_label(key: &str) -> String {
    tr(&format!("review.kind.{key}"))
}

/// Where a passage sits: "Chapter 2 · ¶ 4 · sentence 1" (a chapter's own title when known).
pub fn passage_place(locator: &SourceLocator) -> String {
    let chapter = locator.chapter_title.clone().filter(|title| !title.trim().is_empty()).unwrap_or_else(|| t!("review.passage.chapter", number = locator.chapter).to_string());
    let paragraph = locator.paragraph.map(|n| t!("review.passage.paragraph", number = n).to_string());
    let sentence = locator.sentence.map(|n| t!("review.passage.sentence", number = n).to_string());
    std::iter::once(chapter).chain(paragraph).chain(sentence).collect::<Vec<_>>().join(PLACE_SEPARATOR)
}

/// A share (0–1) as a whole percent.
pub fn percent(share: f64) -> String {
    t!("review.percent", value = (share * 100.0).round() as u64).to_string()
}
