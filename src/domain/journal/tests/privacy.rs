use super::entry;
use crate::domain::journal::privacy::{may_reach, palette_entries, JournalPrivacy, Surface};

#[test]
fn private_records_never_reach_the_stream_meaning_insights_or_agents() {
    let choices = [JournalPrivacy { include_in_search: false }, JournalPrivacy { include_in_search: true }];
    for privacy in choices {
        for surface in [Surface::Feed, Surface::SemanticSearch, Surface::Insights, Surface::AgentContext] {
            assert!(!may_reach(surface, privacy, true), "{surface:?} must stay closed, even with a click, whatever the setting");
        }
    }
}

#[test]
fn palette_search_only_when_opted_in() {
    let entries = vec![entry("a", 0, "private thoughts"), entry("b", 1, "more")];
    assert!(palette_entries(&entries, JournalPrivacy::default()).is_empty(), "off by default");
    assert_eq!(palette_entries(&entries, JournalPrivacy { include_in_search: true }).len(), 2);
    assert!(!JournalPrivacy::default().include_in_search);
}

#[test]
fn ai_only_on_an_explicit_click() {
    let privacy = JournalPrivacy { include_in_search: true };
    assert!(!may_reach(Surface::ExplicitAiAction, privacy, false));
    assert!(may_reach(Surface::ExplicitAiAction, privacy, true));
    assert_eq!(Surface::ALL.iter().filter(|s| may_reach(**s, JournalPrivacy::default(), false)).count(), 0, "nothing is open by default");
}
