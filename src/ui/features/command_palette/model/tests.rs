use super::*;
use crate::domain::PostId;
use crate::ui::features::command_palette::registry::CommandId;
use crate::ui::icons::LucideIcon;

fn item(target: PaletteTarget, group: PaletteGroup, title: &str, keywords: &str) -> PaletteItem {
    PaletteItem {
        target,
        group,
        title: title.to_string(),
        detail: String::new(),
        keywords: keywords.to_string(),
        icon: LucideIcon::Search,
        shortcut: None,
    }
}

fn items() -> Vec<PaletteItem> {
    vec![
        item(PaletteTarget::Command(CommandId::GoTimeline), PaletteGroup::Screens, "Timeline", "home feed"),
        item(PaletteTarget::Command(CommandId::GoMindGraph), PaletteGroup::Screens, "Mind Graph", "network map"),
        item(PaletteTarget::Command(CommandId::NewThread), PaletteGroup::Commands, "New thread", "post write"),
        item(PaletteTarget::Command(CommandId::ToggleSidebar), PaletteGroup::Commands, "Toggle sidebar", "rail collapse"),
        item(PaletteTarget::Post(PostId::new("p1")), PaletteGroup::Posts, "Is the mind a graph of ideas?", ""),
        item(PaletteTarget::Document("d1".into()), PaletteGroup::Documents, "Ethics", "spinoza geometry"),
    ]
}

fn titles(groups: &[ResultGroup], all: &[PaletteItem]) -> Vec<(PaletteGroup, Vec<String>)> {
    groups.iter().map(|group| (group.group, group.items.iter().map(|ranked| all[ranked.index].title.clone()).collect())).collect()
}

#[test]
fn a_leading_prefix_switches_to_command_mode() {
    assert_eq!(parse_query("  > new  "), PaletteQuery { mode: PaletteMode::Commands, text: "new".into() });
    assert_eq!(parse_query(" graph "), PaletteQuery { mode: PaletteMode::Everything, text: "graph".into() });
    assert!(parse_query(">").is_blank());
}

#[test]
fn search_groups_results_with_the_best_group_first() {
    let all = items();
    let groups = rank(&all, &parse_query("mind graph"), &PaletteHistory::default());
    let found = titles(&groups, &all);
    assert_eq!(found[0], (PaletteGroup::Screens, vec!["Mind Graph".to_string()]));
    assert!(found.iter().any(|(group, _)| *group == PaletteGroup::Posts), "posts match by content too");
}

#[test]
fn command_mode_keeps_only_commands() {
    let all = items();
    let groups = rank(&all, &parse_query(">i"), &PaletteHistory::default());
    assert!(groups.iter().all(|group| group.items.iter().all(|ranked| all[ranked.index].target.is_command())));
    assert!(!groups.is_empty());
}

#[test]
fn keyword_matches_rank_below_title_matches_and_are_not_highlighted() {
    let all = items();
    let groups = rank(&all, &parse_query("spinoza"), &PaletteHistory::default());
    let ranked = &groups[0].items[0];
    assert_eq!(all[ranked.index].title, "Ethics");
    assert!(ranked.positions.is_empty());
}

#[test]
fn scattered_letters_are_not_a_match() {
    assert!(is_compact(&[0, 1, 2]));
    assert!(is_compact(&[0, 5]), "acronyms across two words still count");
    assert!(!is_compact(&[0, 9, 20, 31]));
    let all = items();
    let groups = rank(&all, &parse_query("mgi"), &PaletteHistory::default());
    assert!(groups.iter().all(|group| group.group != PaletteGroup::Posts), "the post only matches scattered letters");
}

#[test]
fn keywords_match_only_as_a_literal_phrase() {
    let all = items();
    assert!(!rank(&all, &parse_query("geometry"), &PaletteHistory::default()).is_empty());
    assert!(rank(&all, &parse_query("gmtry"), &PaletteHistory::default()).is_empty(), "no fuzzy keyword hits");
}

#[test]
fn title_matches_carry_highlight_positions() {
    let all = items();
    let groups = rank(&all, &parse_query("tl"), &PaletteHistory::default());
    let first = &groups[0].items[0];
    assert_eq!(all[first.index].title, "Timeline");
    assert_eq!(first.positions, vec![0, 4]);
}

#[test]
fn past_uses_lift_a_result() {
    let all = items();
    let query = parse_query("t");
    let plain = rank(&all, &query, &PaletteHistory::default());
    let history = (0..5).fold(PaletteHistory::default(), |history, _| history.recorded("cmd:toggle_sidebar"));
    let boosted = rank(&all, &query, &history);
    let first = |groups: &[ResultGroup]| all[groups[0].items[0].index].title.clone();
    assert_eq!(first(&plain), "Timeline");
    assert_eq!(first(&boosted), "Toggle sidebar");
}

#[test]
fn commands_gather_under_their_categories() {
    use crate::ui::features::command_palette::registry::CommandCategory;
    let all = items();
    let categories = |groups: &[ResultGroup]| groups.iter().find(|group| group.group == PaletteGroup::Commands).map(|group| group.items.iter().map(|ranked| category_of(&all[ranked.index])).collect::<Vec<_>>());
    let blank = rank(&all, &parse_query(""), &PaletteHistory::default());
    assert_eq!(categories(&blank), Some(vec![Some(CommandCategory::Navigate), Some(CommandCategory::Create)]), "the registry's order with no query");
    let searched = rank(&all, &parse_query(">new thread"), &PaletteHistory::default());
    assert_eq!(categories(&searched).and_then(|found| found.first().copied().flatten()), Some(CommandCategory::Create), "the best result's category first");
    assert_eq!(category_of(&all[4]), None, "only commands have a category");
}

#[test]
fn empty_query_shows_recent_then_frequent_then_screens_and_commands() {
    let all = items();
    let history = PaletteHistory::default().recorded("doc:d1").recorded("cmd:new_thread").recorded("doc:d1").recorded("cmd:go_timeline");
    let groups = rank(&all, &parse_query(""), &history);
    let order: Vec<PaletteGroup> = groups.iter().map(|group| group.group).collect();
    assert_eq!(order, vec![PaletteGroup::Recent, PaletteGroup::Screens, PaletteGroup::Commands]);
    let recent = &titles(&groups, &all)[0].1;
    assert_eq!(recent, &vec!["Timeline".to_string(), "Ethics".to_string(), "New thread".to_string()]);
}

#[test]
fn frequent_lists_repeated_items_not_already_recent() {
    let all = items();
    let history = (0..7).fold(PaletteHistory::default().recorded("doc:d1").recorded("doc:d1"), |history, at| {
        history.recorded(if at % 2 == 0 { "cmd:new_thread" } else { "cmd:go_timeline" })
    });
    assert_eq!(history.frequent(), vec!["cmd:new_thread", "cmd:go_timeline", "doc:d1"]);
    let groups = rank(&all, &parse_query(""), &history);
    assert_eq!(groups[0].group, PaletteGroup::Recent);
    assert!(groups.iter().all(|group| group.group != PaletteGroup::Frequent), "every frequent item is already recent");
}

#[test]
fn history_moves_items_to_the_front_and_counts_uses() {
    let history = PaletteHistory::default().recorded("a").recorded("b").recorded("a");
    assert_eq!(history.recent().collect::<Vec<_>>(), vec!["a", "b"]);
    assert_eq!((history.uses("a"), history.uses("b"), history.uses("c")), (2, 1, 0));
}

#[test]
fn history_round_trips_through_json_and_tolerates_garbage() {
    let history = PaletteHistory::default().recorded("cmd:new_thread").recorded("post:p1");
    assert_eq!(PaletteHistory::from_json(&history.to_json()), history);
    assert_eq!(PaletteHistory::from_json("{ not json"), PaletteHistory::default());
}

#[test]
fn history_keys_are_stable() {
    assert_eq!(PaletteTarget::Command(CommandId::NewThread).history_key(), "cmd:new_thread");
    assert_eq!(PaletteTarget::Post(PostId::new("p1")).history_key(), "post:p1");
    assert_eq!(PaletteTarget::Document("d1".into()).history_key(), "doc:d1");
}

fn groups_of(sizes: &[usize]) -> Vec<ResultGroup> {
    sizes.iter().map(|&size| ResultGroup { group: PaletteGroup::Commands, items: (0..size).map(|index| RankedItem { index, positions: Vec::new() }).collect() }).collect()
}

#[test]
fn arrows_wrap_around_the_results() {
    assert_eq!(stepped(0, -1, 5), 4);
    assert_eq!(stepped(4, 1, 5), 0);
    assert_eq!(stepped(2, 1, 5), 3);
    assert_eq!(stepped(0, 1, 0), 0);
}

#[test]
fn tab_jumps_between_group_starts() {
    let groups = groups_of(&[2, 3, 1]);
    assert_eq!(result_count(&groups), 6);
    assert_eq!(next_group_start(&groups, 0), 2);
    assert_eq!(next_group_start(&groups, 3), 5);
    assert_eq!(next_group_start(&groups, 5), 0, "wraps to the first group");
    assert_eq!(previous_group_start(&groups, 3), 2, "first to the start of its own group");
    assert_eq!(previous_group_start(&groups, 2), 0);
    assert_eq!(previous_group_start(&groups, 0), 5, "wraps to the last group");
}

#[test]
fn flat_indices_locate_their_group_and_row() {
    let groups = groups_of(&[2, 3]);
    assert_eq!(locate(&groups, 0), Some((0, 0)));
    assert_eq!(locate(&groups, 3), Some((1, 1)));
    assert_eq!(locate(&groups, 5), None);
}

#[test]
fn a_leading_question_mark_searches_by_meaning_and_ranks_nothing_itself() {
    assert_eq!(parse_query("  ? free will "), PaletteQuery { mode: PaletteMode::Semantic, text: "free will".into() });
    assert_eq!(PaletteMode::Semantic.initial_text(), "?");
    let query = parse_query("?graph");
    assert!(rank(&items(), &query, &PaletteHistory::default()).is_empty(), "semantic results come from the shell");
}

#[test]
fn semantic_results_form_one_group_by_their_source() {
    let row = |title: &str, score: f32| SemanticRow { item: item(PaletteTarget::Post(PostId::new(title)), PaletteGroup::Meaning, title, ""), excerpt: String::new(), score };
    let ready = SemanticPhase::Ready { query: "mind".into(), source: SemanticSource::Meaning, rows: vec![row("a", 0.9), row("b", 0.4)] };
    let groups = ready.groups();
    assert_eq!(groups.len(), 1);
    assert_eq!((groups[0].group, groups[0].items.len()), (PaletteGroup::Meaning, 2));
    assert_eq!(ready.query(), Some("mind"));
    let words = SemanticPhase::Ready { query: "mind".into(), source: SemanticSource::Words { note: "no key".into() }, rows: vec![row("a", 1.0)] };
    assert_eq!(words.groups()[0].group, PaletteGroup::Words);
    assert!(SemanticPhase::Searching { query: "mind".into() }.groups().is_empty());
    assert!(SemanticPhase::Prompt.rows().is_empty() && SemanticPhase::Prompt.query().is_none());
}

#[test]
fn prose_matches_only_the_words_as_typed() {
    let all = items();
    let found = |query: &str| rank(&all, &parse_query(query), &PaletteHistory::default()).iter().any(|group| group.group == PaletteGroup::Posts);
    assert!(found("mind graph"), "every word appears as typed, even with words between them");
    assert!(!found("grph"), "letters strung through a word are noise in a post");
}
