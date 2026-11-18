use super::{at, day, item_on_post, network, own_post, session_on};
use crate::domain::learning::bloom::BloomLevel;
use crate::domain::learning::goals::edit::{goal_problem, new_goal, toggled_task, with_link, with_tasks, without_link, GoalDraft, GoalProblem};
use crate::domain::learning::goals::progress::{score, MATURE_STABILITY_DAYS};
use crate::domain::learning::goals::{expected_share, forecast, goal_progress, Forecast, Goal, GoalId, GoalInputs, GoalLink, GoalProgress};
use crate::domain::learning::tutor::{Assessment, Exchange, SocraticMove, TutorQuestion, Verdict};
use crate::domain::review::fsrs::{Memory, Params};
use crate::domain::review::schedule::Phase;

fn goal() -> Goal {
    let draft = GoalDraft { title: " Understand Spinoza ".into(), why: "For the reading group".into(), target: day(20) };
    new_goal(&draft, network(), GoalId("goal_1".into()), day(0), at(0)).expect("valid")
}

fn progress(memory: Option<f64>, tasks: (usize, usize), practice: (usize, usize, Option<BloomLevel>)) -> GoalProgress {
    GoalProgress { items: 0, retention: None, memory, new_posts: practice.0, sessions: practice.1, best_level: practice.2, tasks_done: tasks.0, tasks_total: tasks.1, score: 0.0 }
}

#[test]
fn a_goal_needs_a_title_and_a_future_target() {
    assert_eq!(goal().title, "Understand Spinoza");
    let blank = GoalDraft { title: " ".into(), why: String::new(), target: day(5) };
    assert_eq!(goal_problem(&blank, day(0)), Some(GoalProblem::MissingTitle));
    let past = GoalDraft { title: "x".into(), why: String::new(), target: day(0) };
    assert_eq!(goal_problem(&past, day(0)), Some(GoalProblem::TargetInPast));
    assert!(new_goal(&past, network(), GoalId("g".into()), day(0), at(0)).is_none());
}

#[test]
fn links_and_tasks_are_edited_without_repeats() {
    let goal = with_link(with_link(goal(), GoalLink::concept("#Necessity")), GoalLink::concept("necessity"));
    assert_eq!(goal.concepts(), vec!["necessity".to_string()]);
    let goal = with_tasks(goal, &["Write a post".into(), " ".into(), "Write a post".into()], day(1));
    assert_eq!(goal.tasks.len(), 1);
    let id = goal.tasks[0].id.clone();
    let goal = toggled_task(goal, &id, day(2));
    assert!(goal.tasks[0].done);
    assert_eq!(goal.tasks[0].done_on, Some(day(2)));
    assert!(!toggled_task(goal.clone(), &id, day(3)).tasks[0].done);
    assert!(without_link(goal, &GoalLink::concept("necessity")).concepts().is_empty());
}

#[test]
fn progress_counts_linked_items_posts_sessions_and_tasks() {
    let goal = with_link(with_link(goal(), GoalLink::Post { id: "p1".into() }), GoalLink::concept("necessity"));
    let goal = with_tasks(goal, &["one".into(), "two".into()], day(1));
    let goal = toggled_task(goal.clone(), &goal.tasks[0].id.clone(), day(2));
    let mut learnt = item_on_post("i1", "p1", "What is freedom?");
    learnt.schedule.phase = Phase::Review;
    learnt.schedule.memory = Some(Memory { stability: MATURE_STABILITY_DAYS * 2.0, difficulty: 5.0 });
    learnt.schedule.last_review = Some(day(3));
    let unrelated = item_on_post("i2", "p9", "What is a mode?");
    let tagged = crate::domain::review::ReviewItem { tags: vec!["Necessity".into()], ..item_on_post("i3", "p8", "Q?") };
    let items = [learnt, unrelated, tagged];
    let posts = [own_post("o1", "Thinking about necessity today", 2), own_post("o2", "Lunch", 2), own_post("o0", "necessity before the goal", -1)];
    let mut session = session_on("p1");
    let question = TutorQuestion { text: "Q?".into(), level: BloomLevel::Apply, kind: SocraticMove::Apply };
    session.exchanges.push(Exchange { question, answer: Some("a".into()), assessment: Some(Assessment { reached: None, verdict: Verdict::Solid, right: vec![], misconceptions: vec![] }) });
    let params = Params::default();
    let inputs = GoalInputs { items: &items, own_posts: &posts, sessions: std::slice::from_ref(&session), today: day(4), params: &params };
    let progress = goal_progress(&goal, &inputs);
    assert_eq!(progress.items, 2, "the item on the linked post and the item tagged with the concept");
    assert!(progress.retention.is_some_and(|r| r > 0.9));
    assert_eq!(progress.memory, Some(0.5), "one mature item, one new");
    assert_eq!(progress.new_posts, 1, "only own posts on the goal since it started");
    assert_eq!((progress.sessions, progress.best_level), (1, Some(BloomLevel::Apply)));
    assert_eq!((progress.tasks_done, progress.tasks_total), (1, 2));
    assert!(progress.score > 0.0 && progress.score < 1.0);
}

#[test]
fn the_score_leaves_out_what_cannot_be_measured() {
    assert_eq!(score(&progress(None, (0, 0), (0, 0, None))), 0.0);
    let practice_only = score(&progress(None, (0, 0), (6, 0, Some(BloomLevel::Create))));
    assert!((practice_only - 1.0).abs() < 1e-9, "practice alone carries the whole weight");
    let half_tasks = score(&progress(None, (1, 2), (0, 0, None)));
    assert!((half_tasks - 0.3 * 0.5 / 0.6).abs() < 1e-9);
    let all = score(&progress(Some(1.0), (2, 2), (6, 0, Some(BloomLevel::Create))));
    assert!((all - 1.0).abs() < 1e-9);
}

#[test]
fn the_forecast_compares_progress_with_time_spent() {
    let goal = goal();
    assert_eq!(expected_share(&goal, day(10)), 0.5);
    assert_eq!(expected_share(&goal, day(40)), 1.0);
    let at_score = |s: f64| GoalProgress { score: s, ..progress(None, (0, 0), (0, 0, None)) };
    assert_eq!(forecast(&goal, &at_score(0.45), day(10)), Forecast::OnTrack, "within the slack");
    assert_eq!(forecast(&goal, &at_score(0.2), day(10)), Forecast::AtRisk);
    assert_eq!(forecast(&goal, &at_score(0.5), day(21)), Forecast::Overdue);
    assert_eq!(forecast(&goal, &at_score(0.96), day(21)), Forecast::Achieved);
    let done = Goal { completed: true, ..goal };
    assert_eq!(forecast(&done, &at_score(0.0), day(30)), Forecast::Achieved);
}
