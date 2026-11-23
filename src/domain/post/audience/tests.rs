use super::*;
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::domain::post::PostExtras;

fn post(audience: Option<Audience>) -> Post {
    let mut post = Post::create("p", "n1", "user_main", "A thought for some readers.").expect("valid");
    post.extras.audience = audience.map(Box::new);
    post
}

fn id(raw: &str) -> ProfileId {
    ProfileId::new(raw)
}

#[test]
fn everyone_only_me_and_chosen_agents_decide_who_reads() {
    assert!(readable_by(&post(None), &id("ada")), "no scope: everyone");
    assert!(!readable_by(&post(Some(Audience::OnlyMe)), &id("ada")));
    let chosen = post(Some(Audience::Agents { agents: vec![id("ada")] }));
    assert!(readable_by(&chosen, &id("ada")) && !readable_by(&chosen, &id("bo")));
    assert!(readable_by(&post(Some(Audience::OnlyMe)), &id("user_main")), "the author reads their own post");
    assert!(!readable_by(&post(Some(Audience::Agents { agents: Vec::new() })), &id("ada")), "no agent chosen: the user alone");
}

#[test]
fn no_agent_reads_a_locked_post_whatever_its_scope() {
    let mut sealed = post(Some(Audience::Agents { agents: vec![id("ada")] }));
    sealed.lock_with(&LockPassword::new("seven tidy lanterns"), KdfParams { memory_kib: 8, iterations: 1, lanes: 1 }).expect("locked");
    assert!(!readable_by(&sealed, &id("ada")));
}

#[test]
fn toggling_an_agent_adds_or_removes_it() {
    let one = Audience::OnlyMe.toggled(&id("ada"));
    assert_eq!(one, Audience::Agents { agents: vec![id("ada")] });
    assert_eq!(one.toggled(&id("bo")).toggled(&id("ada")), Audience::Agents { agents: vec![id("bo")] });
    assert_eq!((Audience::OnlyMe.key(), one.key()), ("only_me", "agents"));
}

#[test]
fn the_scope_round_trips_and_older_rows_read_as_everyone() {
    let extras = PostExtras { audience: Some(Box::new(Audience::Agents { agents: vec![id("ada")] })), ..Default::default() };
    let json = serde_json::to_string(&extras).expect("json");
    assert_eq!(json, r#"{"audience":{"scope":"agents","agents":["ada"]}}"#);
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("back"), extras);
    assert_eq!(serde_json::from_str::<PostExtras>(r#"{"audience":{"scope":"only_me"}}"#).expect("only me").audience.as_deref(), Some(&Audience::OnlyMe));
    assert_eq!(serde_json::from_str::<PostExtras>("{}").expect("old").audience, None);
    assert!(!extras.has_details());
}
