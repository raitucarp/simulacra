use super::*;

fn set(key: &str, members: &[u32]) -> NamedSet {
    NamedSet::new(key, key.to_uppercase(), members.to_vec())
}

#[test]
fn operations_on_sorted_lists() {
    let (a, b) = ([1, 2, 3, 5], [2, 3, 4]);
    assert_eq!(intersection(&a, &b), vec![2, 3]);
    assert_eq!(union(&a, &b), vec![1, 2, 3, 4, 5]);
    assert_eq!(difference(&a, &b), vec![1, 5]);
    assert_eq!(difference(&b, &a), vec![4]);
    assert!((jaccard(&a, &b) - 0.4).abs() < 1e-12);
    assert_eq!(jaccard(&[], &[]), 1.0);
    assert_eq!(jaccard(&[1], &[]), 0.0);
}

#[test]
fn new_sets_are_sorted_and_distinct() {
    let s = set("a", &[3, 1, 3, 2]);
    assert_eq!(s.members, vec![1, 2, 3]);
    assert_eq!((s.len(), s.is_empty(), s.label.as_str()), (3, false, "A"));
}

#[test]
fn venn_regions_partition_the_union() {
    let sets = [set("a", &[1, 2, 3, 4]), set("b", &[3, 4, 5]), set("c", &[4, 6])];
    let regions = exclusive_regions(&sets);
    let by_mask = |mask: u32| regions.iter().find(|r| r.mask == mask).map(|r| r.members.clone()).unwrap_or_default();
    assert_eq!(by_mask(0b001), vec![1, 2]);
    assert_eq!(by_mask(0b011), vec![3]);
    assert_eq!(by_mask(0b111), vec![4]);
    assert_eq!(by_mask(0b010), vec![5]);
    assert_eq!(by_mask(0b100), vec![6]);
    let total: usize = regions.iter().map(|r| r.members.len()).sum();
    assert_eq!(total, union(&union(&sets[0].members, &sets[1].members), &sets[2].members).len());
    assert_eq!(regions.iter().find(|r| r.mask == 0b111).unwrap().sets(), vec![0, 1, 2]);
}

#[test]
fn upset_rows_rank_by_size() {
    let sets = [set("a", &[1, 2, 3, 4, 5]), set("b", &[4, 5, 6]), set("c", &[5]), set("d", &[9])];
    let rows = upset_rows(&sets, 3);
    assert_eq!(rows[0].mask, 0b0001);
    assert_eq!(rows[0].members.len(), 3);
    assert_eq!(rows.len(), 3);
    assert!(rows.windows(2).all(|w| w[0].members.len() >= w[1].members.len()));
}

#[test]
fn jaccard_matrix_is_symmetric() {
    let sets = [set("a", &[1, 2]), set("b", &[2, 3]), set("c", &[])];
    let m = jaccard_matrix(&sets);
    assert_eq!(m[0][0], 1.0);
    assert_eq!(m[0][1], m[1][0]);
    assert!((m[0][1] - 1.0 / 3.0).abs() < 1e-12);
    assert_eq!(m[2][2], 1.0);
    assert_eq!(SetDimension::ALL.map(SetDimension::key), ["concept", "author", "kind", "subject"]);
}
