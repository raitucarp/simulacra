use crate::domain::review::connections::edit::{cycled_direction, map_problem, relabelled, toggled_edge, with_concept, without_point, MapProblem};
use crate::domain::review::connections::layout::{drill_seed, scatter, slots};
use crate::domain::review::connections::{drill_grade, score_drawing, ConnectionMap, Drill, DrillEffort, DrawnEdge, EdgeMistake, MapEdge, MapPoint};
use crate::domain::review::Grade;

fn concepts(n: usize) -> Vec<MapPoint> {
    (0..n).map(|i| MapPoint::Concept { label: format!("c{i}") }).collect()
}

/// Four points: 0–1 undirected, 1→2 directed "causes", 2–3 labelled "opposes".
fn reference() -> ConnectionMap {
    let edges = vec![
        MapEdge::new(0, 1),
        MapEdge { from: 1, to: 2, directed: true, label: Some("causes".into()) },
        MapEdge { from: 2, to: 3, directed: false, label: Some("Opposes".into()) },
    ];
    ConnectionMap { title: "t".into(), points: concepts(4), edges }
}

fn drawn(from: usize, to: usize, label: Option<&str>) -> DrawnEdge {
    DrawnEdge { from, to, label: label.map(String::from) }
}

#[test]
fn an_exact_drawing_matches_in_any_order_for_undirected_connections() {
    let drawing = [drawn(3, 2, Some(" opposes ")), drawn(1, 0, None), drawn(1, 2, Some("causes"))];
    let score = score_drawing(&reference().edges, &drawing);
    assert!(score.is_exact());
    assert_eq!((score.precision(), score.recall()), (1.0, 1.0));
}

#[test]
fn wrong_direction_wrong_label_extra_and_missing_connections_are_told_apart() {
    let drawing = [drawn(2, 1, Some("causes")), drawn(2, 3, Some("supports")), drawn(0, 3, None)];
    let score = score_drawing(&reference().edges, &drawing);
    let mistakes: Vec<EdgeMistake> = score.wrong.iter().map(|(_, m)| *m).collect();
    assert_eq!(mistakes, vec![EdgeMistake::WrongDirection, EdgeMistake::WrongLabel, EdgeMistake::Unexpected]);
    assert_eq!(score.missing.len(), 3, "the misdrawn ones still count as missing");
    assert!(!score.is_exact());
    assert_eq!((score.precision(), score.recall()), (0.0, 0.0));
}

#[test]
fn an_unlabelled_reference_accepts_any_label_and_a_duplicate_is_extra() {
    let score = score_drawing(&[MapEdge::new(0, 1)], &[drawn(0, 1, Some("anything")), drawn(1, 0, None)]);
    assert_eq!((score.correct.len(), score.wrong.len(), score.missing.len()), (1, 1, 0));
    assert_eq!(score.precision(), 0.5);
    assert_eq!(score_drawing(&[], &[]).precision(), 0.0);
}

#[test]
fn only_an_exact_structure_is_a_success() {
    let exact = score_drawing(&[MapEdge::new(0, 1), MapEdge::new(1, 2)], &[drawn(0, 1, None), drawn(1, 2, None)]);
    let fluent = DrillEffort { seconds: 10, corrections: 0 };
    assert_eq!(drill_grade(&exact, fluent), Grade::Easy);
    assert_eq!(drill_grade(&exact, DrillEffort { corrections: 1, ..fluent }), Grade::Good, "second thoughts are not fluent");
    assert_eq!(drill_grade(&exact, DrillEffort { seconds: 60, corrections: 0 }), Grade::Good);
    assert_eq!(drill_grade(&exact, DrillEffort { seconds: 200, corrections: 0 }), Grade::Hard);
    let almost = score_drawing(&[MapEdge::new(0, 1), MapEdge::new(1, 2)], &[drawn(0, 1, None)]);
    assert_eq!(drill_grade(&almost, fluent), Grade::Again, "one missing connection fails");
    assert_eq!(drill_grade(&score_drawing(&[], &[]), fluent), Grade::Again);
}

#[test]
fn a_drill_draws_by_clicking_pairs_takes_back_and_freezes_once_checked() {
    let map = reference();
    let drill = Drill::default().click(0).click(0);
    assert_eq!(drill.pending, None, "clicking the same point cancels");
    let drill = drill.click(0).click(1).click(1).with_label(Some("causes".into())).click(2);
    assert_eq!(drill.drawn, vec![drawn(0, 1, None), drawn(1, 2, Some("causes"))]);
    let drill = drill.click(1).click(0);
    assert_eq!((drill.drawn.len(), drill.corrections), (1, 1));
    let drill = drill.click(0).click(1).with_label(Some("Opposes".into())).click(3).click(2).checked(&map);
    assert!(drill.score.as_ref().unwrap().is_exact());
    assert_eq!(drill.clone().click(0).click(3), drill, "a checked drill is frozen");
    assert_eq!(Drill::default().click(0).click(1).cleared().corrections, 1);
}

#[test]
fn editing_a_map_connects_cycles_directions_and_renumbers() {
    let map = ConnectionMap { title: "t".into(), points: concepts(3), edges: vec![] };
    let map = toggled_edge(map, 0, 1, false, None);
    let map = toggled_edge(map, 2, 1, true, Some(" leads to ".into()));
    assert_eq!(map.edges[1], MapEdge { from: 2, to: 1, directed: true, label: Some("leads to".into()) });
    assert_eq!(toggled_edge(map.clone(), 1, 0, false, None).edges.len(), 1, "toggling a joined pair disconnects it");
    assert_eq!(toggled_edge(map.clone(), 1, 1, false, None), map);
    let undirected = cycled_direction(map.clone(), 1);
    assert!(!undirected.edges[1].directed, "backwards turns undirected");
    let again = cycled_direction(undirected, 1);
    assert!(again.edges[1].directed && (again.edges[1].from, again.edges[1].to) == (1, 2));
    let forwards = cycled_direction(map.clone(), 0);
    assert!(forwards.edges[0].directed && (forwards.edges[0].from, forwards.edges[0].to) == (0, 1));
    assert_eq!(relabelled(map.clone(), 0, "  ").edges[0].label, None);
    let shrunk = without_point(map.clone(), 0);
    assert_eq!((shrunk.points.len(), shrunk.edges.clone()), (2, vec![MapEdge { from: 1, to: 0, directed: true, label: Some("leads to".into()) }]));
    assert_eq!(map.labels(), vec!["leads to".to_string()]);
}

#[test]
fn a_map_needs_points_connections_and_no_lonely_point() {
    let mut map = ConnectionMap { title: "t".into(), points: concepts(2), edges: vec![MapEdge::new(0, 1)] };
    assert_eq!(map_problem(&map), Some(MapProblem::TooFewPoints));
    map = with_concept(map, "Conatus");
    assert_eq!(with_concept(map.clone(), " conatus ").points.len(), 3, "a known concept is not added twice");
    assert_eq!(map_problem(&map), Some(MapProblem::TooFewConnections));
    map = toggled_edge(map, 0, 2, false, None);
    assert_eq!(map_problem(&map), None);
    map = with_concept(map, "Affect");
    assert_eq!(map_problem(&map), Some(MapProblem::LonelyPoint));
}

#[test]
fn the_scatter_is_reproducible_shuffled_and_inside_the_frame() {
    let a = scatter(10, drill_seed("item_1", 0));
    assert_eq!(a, scatter(10, drill_seed("item_1", 0)));
    assert_ne!(a, scatter(10, drill_seed("item_1", 1)), "a new review, a new arrangement");
    assert!(a.iter().all(|(x, y)| (0.0..=1.0).contains(x) && (0.0..=1.0).contains(y)));
    let mut sorted: Vec<(i64, i64)> = a.iter().map(|(x, y)| ((x * 1e4) as i64, (y * 1e4) as i64)).collect();
    sorted.dedup();
    assert_eq!(sorted.len(), 10, "every point has its own slot");
    assert_eq!(slots(5).len(), 5);
}
