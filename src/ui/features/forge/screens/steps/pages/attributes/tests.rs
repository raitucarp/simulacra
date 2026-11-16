use super::budget::{BudgetMeter, BIG_FIVE, HEXACO};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::attributes::{BUDGET_FIELDS, PERSONALITY_POINT_BUDGET};
use crate::state::forge::{fields_for_step, ForgeStep};

fn with_all(score: u8) -> AgentConfig {
    let mut cfg = AgentConfig::default();
    for field in BUDGET_FIELDS {
        *field.of_mut(&mut cfg) = score;
    }
    cfg
}

#[test]
fn the_cards_cover_every_attributes_field_once() {
    let shown: Vec<&str> = BIG_FIVE.iter().chain(HEXACO.iter()).map(|field| field.path()).collect();
    assert_eq!(shown, BUDGET_FIELDS.map(|field| field.path()).to_vec());
    let mut owned = fields_for_step(ForgeStep::Attributes);
    let mut sorted = shown.clone();
    owned.sort_unstable();
    sorted.sort_unstable();
    assert_eq!(sorted, owned);
}

#[test]
fn the_meter_counts_points_left_and_overspending() {
    let meter = BudgetMeter::of(&with_all(50));
    assert_eq!(meter.spent, 300);
    assert_eq!(meter.remaining(), PERSONALITY_POINT_BUDGET - 300);
    assert_eq!(meter.over_by(), None);
    assert!((meter.fill() - 300.0 / f32::from(PERSONALITY_POINT_BUDGET)).abs() < f32::EPSILON);

    let full = BudgetMeter::of(&with_all(70));
    assert_eq!((full.remaining(), full.over_by()), (0, None));

    let over = BudgetMeter::of(&with_all(90));
    assert_eq!(over.remaining(), 0);
    assert_eq!(over.over_by(), Some(540 - PERSONALITY_POINT_BUDGET));
    assert!((over.fill() - 1.0).abs() < f32::EPSILON, "the bar never overflows");
}

#[test]
fn an_empty_budget_reads_as_full() {
    assert!((BudgetMeter { spent: 0, budget: 0 }.fill() - 1.0).abs() < f32::EPSILON);
}
