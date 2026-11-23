//! N16: lenses, their prompt lines, and which agent takes which lens.

use super::*;
use crate::domain::ProfileId;

fn room() -> Vec<(ProfileId, String)> {
    [("ada", "Ada historian of medieval trade"), ("bo", "Bo moral philosopher and ethicist"), ("cy", "Cy market economist")].map(|(id, text)| (ProfileId::new(id), text.to_string())).to_vec()
}

#[test]
fn every_lens_round_trips_its_key_and_names_itself_in_its_prompt() {
    for lens in Lens::ALL {
        assert_eq!(Lens::from_key(lens.key()), Some(*lens));
        assert!(lens.prompt_line().contains(lens.key()));
    }
    assert_eq!(serde_json::to_string(&Lens::Ethics).expect("json"), "\"ethics\"");
}

#[test]
fn each_lens_goes_to_the_agent_that_knows_its_field() {
    let fan = assign(&Lens::DEFAULTS, &room());
    let pairs: Vec<(&str, Lens)> = fan.iter().map(|entry| (entry.agent.as_str(), entry.lens)).collect();
    assert_eq!(pairs, vec![("bo", Lens::Ethics), ("ada", Lens::History), ("cy", Lens::Economics)]);
}

#[test]
fn an_agent_takes_one_lens_and_a_fan_is_never_wider_than_the_room() {
    let fan = assign(&[Lens::Law, Lens::Ecology, Lens::Science, Lens::Culture], &room()[..2]);
    assert_eq!(fan.len(), 2);
    assert_ne!(fan[0].agent, fan[1].agent);
    assert!(assign(&Lens::DEFAULTS, &[]).is_empty());
}

#[test]
fn a_fan_reads_back_and_an_unknown_lens_reads_as_no_fan() {
    let fan = LensFan { entries: assign(&Lens::DEFAULTS, &room()), at: jiff::Timestamp::UNIX_EPOCH };
    let json = serde_json::to_string(&fan).expect("json");
    assert_eq!(serde_json::from_str::<LensFan>(&json).expect("fan"), fan);
    let stored = r#"{"fan":{"entries":[{"agent":"a","lens":"astrology"}],"at":"2026-01-01T00:00:00Z"},"lens":"astrology","mood":"calm"}"#;
    let extras: crate::domain::post::PostExtras = serde_json::from_str(stored).expect("extras");
    assert_eq!((extras.fan, extras.lens, extras.mood.as_deref()), (None, None, Some("calm")), "the rest of the extras survive");
}
