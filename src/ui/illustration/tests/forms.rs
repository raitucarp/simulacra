use super::super::form::*;
use super::super::*;
use super::{default_colors, parse};
use crate::domain::portrait::form::{FormCatalog, NONE_VARIANT};
use crate::domain::portrait::{AvatarMode, FormColor, FormKind, FormLayer, FormSpec};

/// Seeds composed per kind by the composition test.
const SAMPLE_SEEDS: u64 = 6;
const GROUP_OPEN: &str = r#"<g fill="none" stroke-linecap"#;

#[test]
fn composed_forms_fill_and_parse_for_every_form() {
    let colors = default_colors();
    for kind in FormKind::ALL {
        for form in FormCatalog::of(kind).variants(FormLayer::Body) {
            for seed in 0..SAMPLE_SEEDS {
                let spec = FormSpec::random(kind, form, seed);
                let template = compose_form_template(&spec);
                assert_eq!(template.matches(GROUP_OPEN).count(), spec.layer_paths().len(), "{kind:?}/{form}: a part is missing");
                let filled = fill_template(&template, &colors, &TemplateExtras::form(&spec));
                assert!(!filled.contains("{{"), "{kind:?}/{form}");
                assert!(filled.contains(spec.hex(FormColor::Primary)), "{kind:?}/{form}: primary applied");
                parse(&filled, &format!("{kind:?}/{form}/{seed}"));
            }
        }
    }
}

#[test]
fn clipped_layers_use_the_silhouette_and_anchored_layers_follow_the_face() {
    let spec = FormSpec::new(FormKind::Object, "chair")
        .with_part(FormLayer::Finish, "rust")
        .with_part(FormLayer::Markings, "polka")
        .with_part(FormLayer::Accessory, "monocle");
    let template = compose_form_template(&spec);
    let clip = format!(r#"clip-path="url(#{FORM_CLIP_ID})""#);
    assert_eq!(template.matches(&clip).count(), 2, "finish and markings are clipped");
    let defined = template.find(&format!(r#"<clipPath id="{FORM_CLIP_ID}""#)).expect("the body defines the silhouette");
    assert!(defined < template.find(&clip).unwrap(), "the silhouette is defined before the clipped layers");
    assert_eq!(template.matches(r#"transform="matrix("#).count(), 3, "eyes, mouth and accessory follow the anchor");
    let plain = compose_form_template(&FormSpec::new(FormKind::Object, "tin_can"));
    assert!(!plain.contains("matrix("), "the default anchor adds no transform");
}

#[test]
fn layers_paint_in_catalog_order() {
    let spec = FormSpec::new(FormKind::Plant, "oak_tree").with_part(FormLayer::Aura, "pollen");
    let template = compose_form_template(&spec);
    let order: Vec<usize> = ["back/plain", "aura/pollen", "body/oak_tree", "mouth/smile", "eyes/dot"]
        .iter()
        .map(|part| {
            let dir = if part.starts_with("aura") || part.starts_with("body") { "plant" } else { "shared" };
            let path = format!("illustrations/forms/{dir}/{part}.svg");
            let text = crate::ui::assets::template_text(&path).unwrap_or_else(|| panic!("{path}"));
            let body = portrait::layer_body(&text).unwrap();
            let probe = &body[..body.len().min(60)];
            template.find(probe).unwrap_or_else(|| panic!("{path} not composed"))
        })
        .collect();
    assert!(order.windows(2).all(|w| w[0] < w[1]), "{order:?}");
    let bare = spec.with_part(FormLayer::Eyes, NONE_VARIANT);
    assert_eq!(compose_form_template(&bare).matches(GROUP_OPEN).count(), bare.layer_paths().len(), "`none` draws nothing");
}

#[test]
fn form_sources_ignore_seed_and_colors_and_forms_render_as_avatars() {
    let spec = FormSpec::random(FormKind::Element, "water", 2);
    let same_look = FormSpec { seed: 77, ..spec.clone() }.with_color(FormColor::Primary, "ember");
    assert_eq!(form_source(&spec), form_source(&same_look));
    assert_ne!(form_source(&spec), form_source(&spec.clone().with_part(FormLayer::Body, "fire")));
    assert_ne!(TemplateExtras::form(&spec), TemplateExtras::form(&same_look), "colors travel in the extras");
    let image = form_image(&spec, Some(64));
    assert!(String::from_utf8_lossy(&image.bytes).contains(r#"width="64""#));
    assert!(avatar_source(&AvatarMode::Form(spec), gpui_kit::px(40.0)).is_some());
}
