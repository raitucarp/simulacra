use super::{day, noon};
use crate::domain::journal::capsule::notify::{announced, in_list_order, to_announce};
use crate::domain::journal::capsule::{opening_day, parse_until, seal, CapsuleState, SealProblem, SealUntil, TimeCapsule, MAX_SEAL_DAYS};
use crate::domain::journal::RecordId;
use crate::domain::media::ImageAttachment;

fn sealed(id: &str, days: u32) -> TimeCapsule {
    seal(RecordId::new(id), "Dear me", vec![ImageAttachment::at("a.png")], SealUntil::Days(days), noon(0), day(0)).unwrap()
}

#[test]
fn sealing_checks_the_text_and_the_day() {
    let empty = seal(RecordId::new("x"), "  ", vec![], SealUntil::Days(3), noon(0), day(0));
    assert_eq!(empty, Err(SealProblem::Empty));
    assert_eq!(seal(RecordId::new("x"), "hi", vec![], SealUntil::Date(day(0)), noon(0), day(0)), Err(SealProblem::OutOfRange), "not today");
    assert_eq!(seal(RecordId::new("x"), "hi", vec![], SealUntil::Days(0), noon(0), day(0)), Err(SealProblem::OutOfRange));
    assert_eq!(opening_day(SealUntil::Days(MAX_SEAL_DAYS + 1), day(0)), None);
    assert_eq!(opening_day(SealUntil::Date(day(30)), day(0)), Some(day(30)));
    assert_eq!(sealed("a", 10).opens_on, day(10));
}

#[test]
fn contents_stay_hidden_until_opened() {
    let capsule = sealed("a", 10);
    assert_eq!(capsule.state(day(3)), CapsuleState::Sealed { days_left: 7 });
    assert_eq!((capsule.body(), capsule.attachments().is_none(), capsule.attachment_count()), (None, true, 1));
    assert_eq!(capsule.opened(noon(3), day(3)), None, "cannot open early");
    assert_eq!(capsule.replied("too soon"), None, "cannot reply while sealed");
}

#[test]
fn opening_on_its_day_then_replying() {
    let capsule = sealed("a", 10);
    assert_eq!(capsule.state(day(10)), CapsuleState::Ready);
    let open = capsule.opened(noon(12), day(12)).unwrap();
    assert_eq!((open.state(day(12)), open.body(), open.attachments().map(<[_]>::len)), (CapsuleState::Opened, Some("Dear me"), Some(1)));
    assert_eq!(open.opened(noon(20), day(20)).unwrap().opened_at, open.opened_at, "opening twice keeps the first time");
    let replied = open.replied("  Still here. ").unwrap();
    assert_eq!(replied.reply.as_deref(), Some("Still here."));
    assert_eq!(open.replied("   ").unwrap().reply, None);
    assert_eq!(capsule.days_sealed(), 10);
}

#[test]
fn ready_capsules_are_announced_once() {
    let list = vec![sealed("soon", 2), sealed("later", 20)];
    assert!(to_announce(&list, day(1)).is_empty());
    let due: Vec<&str> = to_announce(&list, day(5)).iter().map(|c| c.id.as_str()).collect();
    assert_eq!(due, ["soon"]);
    let told = vec![announced(&list[0]), list[1].clone()];
    assert!(to_announce(&told, day(5)).is_empty(), "announced capsules are not told again");
}

#[test]
fn list_order_puts_ready_first() {
    let opened = sealed("opened", 1).opened(noon(2), day(2)).unwrap();
    let list = vec![sealed("later", 30), opened, sealed("ready", 3), sealed("sooner", 10)];
    let order: Vec<&str> = in_list_order(&list, day(5)).iter().map(|c| c.id.as_str()).collect();
    assert_eq!(order, ["ready", "sooner", "later", "opened"]);
}

#[test]
fn the_opening_field_takes_days_or_a_day() {
    assert_eq!(parse_until(" 30 "), Some(SealUntil::Days(30)));
    assert_eq!(parse_until("2027-01-31"), Some(SealUntil::Date(chrono::NaiveDate::from_ymd_opt(2027, 1, 31).unwrap())));
    assert_eq!(parse_until("next year"), None);
}
