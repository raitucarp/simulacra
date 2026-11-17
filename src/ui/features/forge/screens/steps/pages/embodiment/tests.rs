use super::{BODY, STANCE};
use crate::state::forge::{fields_for_step, ForgeStep, TraitField};

#[test]
fn every_embodiment_field_is_on_the_page_once() {
    let mut shown: Vec<&str> = vec![TraitField::BodyKind.path(), TraitField::Form.path()];
    shown.extend(BODY.iter().chain(STANCE.iter()).map(|field| field.path()));
    let mut owned = fields_for_step(ForgeStep::Embodiment);
    owned.sort_unstable();
    shown.sort_unstable();
    assert_eq!(shown, owned);
}
