use crate::domain::views::{PostView, ViewScope};

#[test]
fn every_view_round_trips_its_stored_key() {
    for view in PostView::ALL {
        assert_eq!(PostView::from_key(view.key()), Some(*view));
    }
    assert_eq!(PostView::from_key("cards"), Some(PostView::Cards));
    assert_eq!(PostView::from_key("outline"), Some(PostView::Outline));
    assert_eq!(PostView::from_key("argument"), Some(PostView::Argument));
}

#[test]
fn the_thread_offers_cards_outline_and_argument_in_order() {
    let thread = PostView::in_scope(ViewScope::Thread);
    assert_eq!(&thread[..3], &[PostView::Cards, PostView::Outline, PostView::Argument]);
}

#[test]
fn every_view_is_offered_somewhere_and_cards_everywhere() {
    for view in PostView::ALL {
        assert!(!view.scopes().is_empty(), "{view:?}");
    }
    assert!(PostView::Cards.offered_in(ViewScope::Thread) && PostView::Cards.offered_in(ViewScope::Profile));
}

#[test]
fn a_view_the_scope_does_not_offer_falls_back_to_cards() {
    assert_eq!(PostView::Argument.within(ViewScope::Profile), PostView::Cards);
    assert_eq!(PostView::Outline.within(ViewScope::Thread), PostView::Outline);
    assert!(!PostView::in_scope(ViewScope::Profile).contains(&PostView::Argument));
}

#[test]
fn the_toggle_swaps_cards_and_outline() {
    assert_eq!(PostView::default().toggled(), PostView::Outline);
    assert_eq!(PostView::Outline.toggled(), PostView::Cards);
    assert_eq!(PostView::Argument.toggled(), PostView::Cards);
}
