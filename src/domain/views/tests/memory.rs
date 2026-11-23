use crate::domain::views::memory::{ViewMemory, ViewOwner, MAX_REMEMBERED};
use crate::domain::views::PostView;
use crate::domain::{PostId, ProfileId};

fn thread(id: &str) -> ViewOwner {
    ViewOwner::Thread(PostId::new(id))
}

#[test]
fn each_thread_and_profile_keeps_its_own_choice() {
    let mut memory = ViewMemory::default();
    memory.remember(&thread("a"), PostView::Outline);
    memory.remember(&ViewOwner::Profile(ProfileId::new("p")), PostView::Calendar);
    memory.remember(&thread("a"), PostView::RapidLog);
    assert_eq!(memory.chosen(&thread("a")), Some(PostView::RapidLog));
    assert_eq!(memory.chosen(&ViewOwner::Profile(ProfileId::new("p"))), Some(PostView::Calendar));
    assert_eq!(memory.chosen(&thread("b")), None);
    assert_eq!(memory.choices.len(), 2, "a new choice replaces the old one");
}

#[test]
fn the_memory_round_trips_as_json_and_survives_damage() {
    let mut memory = ViewMemory::default();
    memory.remember(&thread("a"), PostView::Board);
    let json = memory.to_json();
    assert_eq!(ViewMemory::from_json(Some(&json)), memory);
    assert_eq!(ViewMemory::from_json(Some("{not json")), ViewMemory::default());
    assert_eq!(ViewMemory::from_json(Some(r#"{"choices":[{"owner":"thread:a","view":"no_such_view"}]}"#)), ViewMemory::default());
    assert_eq!(ViewMemory::from_json(None), ViewMemory::default());
}

#[test]
fn a_view_the_scope_no_longer_offers_is_not_chosen() {
    let mut memory = ViewMemory::default();
    memory.remember(&ViewOwner::Profile(ProfileId::new("p")), PostView::Argument);
    assert_eq!(memory.chosen(&ViewOwner::Profile(ProfileId::new("p"))), None);
}

#[test]
fn the_oldest_choices_are_forgotten_past_the_cap() {
    let mut memory = ViewMemory::default();
    for n in 0..=MAX_REMEMBERED {
        memory.remember(&thread(&n.to_string()), PostView::Raw);
    }
    assert_eq!(memory.choices.len(), MAX_REMEMBERED);
    assert_eq!(memory.chosen(&thread("0")), None);
    assert_eq!(memory.chosen(&thread(&MAX_REMEMBERED.to_string())), Some(PostView::Raw));
}
