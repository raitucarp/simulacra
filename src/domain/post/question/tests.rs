use super::*;
use crate::domain::post::PostExtras;

fn id(raw: &str) -> ProfileId {
    ProfileId::new(raw)
}

#[test]
fn meaning_routes_to_the_closest_agents_and_at_most_three() {
    let agents = vec![(id("optics"), vec![0.0, 1.0, 0.0]), (id("ethics"), vec![1.0, 0.0, 0.0]), (id("moral"), vec![0.9, 0.1, 0.0]), (id("cook"), vec![0.0, 0.0, 1.0])];
    let asked = route_by_meaning(&[1.0, 0.05, 0.0], &agents);
    let names: Vec<&str> = asked.iter().map(|a| a.agent.as_str()).collect();
    assert_eq!(names, vec!["ethics", "moral"], "only the agents close to the best fit");
    assert!(asked[0].fit > asked[1].fit && asked[0].fit <= 1000);
    let many: Vec<_> = (0..6).map(|n| (id(&format!("a{n}")), vec![1.0, 0.0])).collect();
    assert_eq!(route_by_meaning(&[1.0, 0.0], &many).len(), MAX_ASKED);
    assert!(route_by_meaning(&[1.0], &[]).is_empty());
}

#[test]
fn words_route_until_things_are_embedded_and_someone_is_always_asked() {
    let agents = vec![(id("cook"), "bread sourdough ovens".to_string()), (id("optics"), "lenses light refraction".to_string())];
    let asked = route_by_words("Why does light bend through lenses?", &agents);
    assert_eq!(asked.iter().map(|a| a.agent.as_str()).collect::<Vec<_>>(), vec!["optics"]);
    assert_eq!(route_by_words("Hmm?", &agents).len(), 1, "no shared words: the first agent still answers");
}

#[test]
fn the_accepted_answer_leads_then_fit_and_reception() {
    let answer = |post: &str, fit: u16, likes: usize| Answer { post: PostId::new(post), fit, likes, reactions: 0 };
    let ranked = rank_answers(vec![answer("a", 900, 0), answer("b", 700, 5), answer("c", 800, 0)], None);
    assert_eq!(ranked.iter().map(|a| a.post.as_str()).collect::<Vec<_>>(), vec!["b", "a", "c"]);
    let accepted = PostId::new("c");
    assert_eq!(rank_answers(ranked, Some(&accepted))[0].post, accepted);
}

#[test]
fn the_question_round_trips_and_older_rows_have_none() {
    let question = Question { asked: vec![Asked { agent: id("ada"), fit: 812 }], accepted: None };
    let extras = PostExtras { question: Some(Box::new(question.clone())), ..Default::default() };
    let json = serde_json::to_string(&extras).expect("json");
    assert_eq!(json, r#"{"question":{"asked":[{"agent":"ada","fit":812}]}}"#);
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("back"), extras);
    assert_eq!(serde_json::from_str::<PostExtras>("{}").expect("old").question, None);
    assert_eq!((question.fit_of(&id("ada")), question.fit_of(&id("bo"))), (812, 0));
    assert!(!extras.has_details());
}
