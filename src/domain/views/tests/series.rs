use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::series::{series_id, SeriesEntry};
use crate::domain::views::series::series_groups;
use crate::domain::Post;

fn part(id: &str, title: &str, number: u32, goal: Option<u32>, hour: i8) -> Post {
    let mut post = Post::create(id, "n1", "u1", &format!("Part {number} of {title}.")).expect("valid");
    post.created_at = crate::time::utc(2026, 11, 1, hour, 0, 0);
    post.extras.series = Some(Box::new(SeriesEntry { id: series_id(title), title: title.into(), number, goal }));
    post
}

#[test]
fn series_are_grouped_latest_first_with_parts_in_order() {
    let posts = vec![part("a2", "Sketches", 2, None, 3), part("b1", "Walks", 1, Some(10), 5), part("a1", "Sketches", 1, Some(30), 1), Post::create("x", "n1", "u1", "Not in a series.").expect("valid")];
    let groups = series_groups(&posts);
    assert_eq!(groups.iter().map(|g| g.title.as_str()).collect::<Vec<_>>(), vec!["Walks", "Sketches"]);
    assert_eq!(groups[1].parts.iter().map(|p| p.number).collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(groups[1].goal, Some(30), "the goal of any part counts");
}

#[test]
fn a_locked_part_keeps_its_number_but_not_its_text() {
    let mut locked = part("s1", "Secrets", 1, None, 1);
    locked.lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    let groups = series_groups([&locked]);
    assert_eq!((groups[0].parts[0].number, groups[0].parts[0].locked, groups[0].parts[0].text.as_str()), (1, true, ""));
}
