//! Analysis: unsupported claims, circular support, unanswered objections, grounded labels.

use super::super::analysis::{analyse, Standing};
use super::super::model::{EdgeKind, NodeRole};
use super::map;
use EdgeKind::{Attack, Support};
use NodeRole::*;
use Standing::*;

#[test]
fn an_unattacked_argument_stands_and_its_target_falls() {
    let m = map(&[("n1", Claim), ("n2", Rebuttal)], &[("n2", "n1", Attack)]);
    let report = analyse(&m);
    assert_eq!(report.standing, vec![Rejected, Accepted]);
    assert_eq!(report.unanswered, vec![1]);
    assert_eq!(report.standing_claims(&m), 0);
}

#[test]
fn reinstatement_a_counter_objection_restores_the_claim() {
    // n3 attacks the objection n2, which attacked the claim n1: n1 is defended.
    let m = map(&[("n1", Claim), ("n2", Rebuttal), ("n3", Ground)], &[("n2", "n1", Attack), ("n3", "n2", Attack)]);
    let report = analyse(&m);
    assert_eq!(report.standing, vec![Accepted, Rejected, Accepted]);
    assert!(report.unanswered.is_empty(), "the objection was answered, and its answer is no objection: {:?}", report.unanswered);
    assert_eq!(report.standing_claims(&m), 1);
}

#[test]
fn mutual_attack_and_odd_cycles_stay_undecided() {
    let mutual = map(&[("n1", Claim), ("n2", Claim)], &[("n1", "n2", Attack), ("n2", "n1", Attack)]);
    assert_eq!(analyse(&mutual).standing, vec![Undecided, Undecided]);
    let odd = map(&[("a", Claim), ("b", Claim), ("c", Claim)], &[("a", "b", Attack), ("b", "c", Attack), ("c", "a", Attack)]);
    assert_eq!(analyse(&odd).standing, vec![Undecided; 3]);
    // An unattacked argument breaks the tie: d defeats n2, so n1 is accepted.
    let broken = map(&[("n1", Claim), ("n2", Claim), ("d", Ground)], &[("n1", "n2", Attack), ("n2", "n1", Attack), ("d", "n2", Attack)]);
    assert_eq!(analyse(&broken).standing, vec![Accepted, Rejected, Accepted]);
}

#[test]
fn supported_attacks_carry_through_support_chains() {
    // g supports the objection r, r attacks the claim c, and e defeats r. Under supported
    // attacks g still attacks c through r, so c falls; with plain Dung attacks it would stand.
    let m = map(&[("g", Ground), ("r", Rebuttal), ("c", Claim), ("e", Ground)], &[("g", "r", Support), ("r", "c", Attack), ("e", "r", Attack)]);
    assert_eq!(analyse(&m).standing, vec![Accepted, Rejected, Rejected, Accepted]);
    let plain = map(&[("g", Ground), ("r", Rebuttal), ("c", Claim), ("e", Ground)], &[("r", "c", Attack), ("e", "r", Attack)]);
    assert_eq!(analyse(&plain).standing, vec![Accepted, Rejected, Accepted, Accepted]);
    // A supporter breaks a mutual attack: s attacks b through a, so a is defended.
    let tie = map(&[("a", Claim), ("b", Claim), ("s", Ground)], &[("a", "b", Attack), ("b", "a", Attack), ("s", "a", Support)]);
    assert_eq!(analyse(&tie).standing, vec![Accepted, Rejected, Accepted]);
}

#[test]
fn self_attack_is_never_accepted() {
    let m = map(&[("n1", Claim), ("n2", Ground)], &[("n1", "n1", Attack), ("n2", "n1", Support)]);
    let report = analyse(&m);
    assert_eq!(report.standing[1], Accepted);
    assert_ne!(report.standing[0], Accepted);
}

#[test]
fn unsupported_claims_are_claims_without_incoming_support() {
    let m = map(&[("n1", Claim), ("n2", Ground), ("n3", Claim), ("n4", Claim)], &[("n2", "n1", Support), ("n4", "n3", Attack)]);
    assert_eq!(analyse(&m).unsupported, vec![2, 3]);
}

#[test]
fn circular_support_finds_each_cycle_once() {
    let m = map(
        &[("a", Claim), ("b", Ground), ("c", Ground), ("d", Claim), ("e", Ground), ("f", Ground)],
        &[("a", "b", Support), ("b", "c", Support), ("c", "a", Support), ("e", "d", Support), ("d", "f", Support), ("f", "d", Support)],
    );
    let report = analyse(&m);
    assert_eq!(report.circular, vec![vec![0, 1, 2], vec![3, 5]]);
    assert!(report.is_circular(5) && !report.is_circular(4));
    let acyclic = map(&[("a", Claim), ("b", Ground)], &[("b", "a", Support)]);
    assert!(analyse(&acyclic).circular.is_empty());
}

#[test]
fn edges_to_missing_nodes_are_ignored_and_the_empty_map_is_quiet() {
    let m = map(&[("n1", Claim)], &[("ghost", "n1", Attack)]);
    assert_eq!(analyse(&m).standing, vec![Accepted]);
    let empty = map(&[], &[]);
    let report = analyse(&empty);
    assert!(report.standing.is_empty() && report.unsupported.is_empty() && report.circular.is_empty());
}
