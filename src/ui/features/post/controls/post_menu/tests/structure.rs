//! U3.8: the post menu keeps the structure rules on every card, in the section order of the
//! architecture, and every label it shows is translated in every locale.

use super::*;
use crate::i18n::Language;
use crate::ui::features::post::controls::post_menu::post_menu_spec;
use crate::ui::menus::{post_ids as ids, rules};

#[test]
fn every_card_keeps_the_structure_rules() {
    for (name, card) in every_card() {
        let problems = rules::violations(&post_menu_spec(&card));
        assert!(problems.is_empty(), "{name}: {problems:#?}");
    }
}

#[test]
fn every_label_is_translated_in_every_locale() {
    for (name, card) in every_card() {
        for key in rules::label_keys(&post_menu_spec(&card)) {
            for language in Language::ALL {
                assert!(crate::_rust_i18n_try_translate(language.code(), &key).is_some(), "{name}: {key} missing in {}", language.code());
            }
        }
    }
}

#[test]
fn the_top_level_follows_the_section_order() {
    let spec = post_menu_spec(&own());
    let top: Vec<&str> = spec.entries().map(|entry| entry.id().as_ref()).collect();
    assert_eq!(top, [ids::REPLY, ids::QUOTE, ids::MARGIN, ids::THINK, ids::STUDY, ids::ORGANIZE, ids::VIEW, ids::SHARE, ids::EDIT_HISTORY, ids::DELETE]);
    assert!(spec.entries().last().is_some_and(|entry| entry.is_destructive()), "Delete is last and red");
    let others = post_menu_spec(&super::others());
    assert!(others.entries().count() <= 8, "someone else's post shows about eight entries");
}

#[test]
fn the_ai_actions_nest_by_group() {
    let spec = post_menu_spec(&others());
    let path = spec.path_of(&format!("{}steelman", ids::AI_PREFIX)).expect("steelman");
    assert_eq!(path.len(), 3, "Think with AI › Challenge › Steelman: {path:?}");
    assert_eq!(spec.depth(), 3);
}
