use super::histogram::nice_width;
use super::*;
use crate::domain::insights::facts::PostFacts;

fn day(d: i8) -> Date {
    jiff::civil::date(2026, 3, d)
}

fn post(author: usize, kind: PostKind, d: i8, hour: usize, chars: usize) -> PostFacts {
    PostFacts {
        author,
        kind,
        created_at: crate::time::utc(2026, 3, d, hour as i8, 0, 0),
        day: day(d),
        weekday: day(d).weekday_index(),
        hour,
        chars,
        words: chars / 5,
        words_per_sentence: (chars / 5) as f64,
    }
}

trait WeekdayIndex {
    fn weekday_index(&self) -> usize;
}

impl WeekdayIndex for Date {
    fn weekday_index(&self) -> usize {
        self.weekday().to_monday_zero_offset() as usize
    }
}

#[test]
fn timeline_has_a_bar_per_day_and_buckets_long_spans() {
    let posts = [post(0, PostKind::Thought, 2, 9, 40), post(0, PostKind::Reply, 2, 10, 20), post(1, PostKind::Quote, 4, 23, 60)];
    let bars = timeline(&posts, None, day(5));
    assert_eq!(bars.len(), 4);
    assert_eq!(bars[0].total(), 2);
    assert_eq!(bars[0].counts[PostKind::Reply.index()], 1);
    assert_eq!(bars[2].total(), 1);
    assert_eq!(bars[3].end(), day(5));
    let year = timeline(&posts, Some(day(5) - crate::time::days(364)), day(5));
    assert!(year.len() <= timeline::MAX_BUCKETS);
    assert_eq!(year.iter().map(TimeBucket::total).sum::<usize>(), 3);
    assert!(timeline(&[], None, day(5)).is_empty());
}

#[test]
fn report_counts_authors_kinds_rhythm_and_traits() {
    let posts = [post(0, PostKind::Thought, 2, 9, 40), post(1, PostKind::Reply, 2, 9, 20), post(1, PostKind::Reply, 3, 22, 30)];
    let authors = [
        AuthorFacts { id: "a".into(), name: "A".into(), is_agent: false, traits: None },
        AuthorFacts { id: "b".into(), name: "B".into(), is_agent: true, traits: Some([10, 20, 30, 40, 50, 60]) },
    ];
    let report = ActivityReport::of(&posts, &authors, None, day(3));
    assert_eq!(report.authors[0].0, 1, "the busiest author leads");
    assert_eq!(report.kinds[PostKind::Reply.index()], 2);
    assert_eq!(report.rhythm[day(2).weekday_index()][9], 2);
    assert_eq!(report.peak(), Some((day(2).weekday_index(), 9, 2)));
    assert_eq!(report.traits, vec![(1, [10, 20, 30, 40, 50, 60])]);
    assert_eq!(report.lengths.total(), 3);
    assert_eq!(ActivityReport::default().peak(), None);
}

#[test]
fn nice_widths_and_histograms() {
    assert_eq!(nice_width(0.7), 1.0);
    assert_eq!(nice_width(13.0), 20.0);
    assert_eq!(nice_width(42.0), 50.0);
    assert_eq!(nice_width(0.0), 1.0);
    let values: Vec<f64> = (0..100).map(f64::from).collect();
    let h = histogram(&values);
    assert_eq!(h.total(), 100);
    assert_eq!(h.width, 10.0);
    assert_eq!(h.bin(1), (10.0, 20.0));
    assert_eq!(h.median, 50.0);
    assert!(histogram(&[]).counts.is_empty());
    assert_eq!(histogram(&[7.0, 7.0]).total(), 2);
}
