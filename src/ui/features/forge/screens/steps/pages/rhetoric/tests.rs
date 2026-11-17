use super::{SIGNS, STYLE};
use crate::state::forge::{fields_for_step, ForgeStep, TraitField, TraitSet};

#[test]
fn every_rhetoric_field_is_on_the_page_once() {
    let mut shown: Vec<&str> = vec![TraitField::RhetoricalMode.path(), TraitSet::RhetoricalDevices.path(), TraitSet::LexicalFields.path()];
    shown.extend(STYLE.iter().chain(SIGNS.iter()).map(|field| field.path()));
    let mut owned = fields_for_step(ForgeStep::Rhetoric);
    owned.sort_unstable();
    shown.sort_unstable();
    assert_eq!(shown, owned);
}
