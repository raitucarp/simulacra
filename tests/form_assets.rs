//! Form asset guard: every catalog variant of every kind has its SVG (and every SVG under
//! `assets/illustrations/forms/` is in a catalog), bodies define the silhouette clip, form parts
//! use only palette + `{{primary}}`/`{{secondary}}` placeholders, and every form and part is
//! labelled in English (locale parity carries the labels to the other locales).

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use simulacra::domain::portrait::form::{FormCatalog, FormLayer, FORM_ASSET_ROOT, NONE_VARIANT};
use simulacra::domain::portrait::FormKind;

const ASSETS_DIR: &str = "assets";
const ENGLISH: &str = "locales/en.yml";
const CLIP_DEFINITION: &str = r#"<clipPath id="form-silhouette">"#;
const FORM_PLACEHOLDERS: [&str; 2] = ["primary", "secondary"];
const PORTRAIT_ONLY: [&str; 2] = ["skin", "hair"];
const PORTRAIT_DIR: &str = "assets/illustrations/portrait";

fn svgs_under(dir: &Path, out: &mut BTreeSet<String>) {
    for entry in fs::read_dir(dir).expect("readable dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            svgs_under(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "svg") {
            out.insert(path.strip_prefix(ASSETS_DIR).unwrap().to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Every (layer, asset path) the catalogs reference, `none` excluded.
fn catalog_assets() -> BTreeSet<(FormLayer, String)> {
    let mut out = BTreeSet::new();
    for kind in FormKind::ALL {
        let catalog = FormCatalog::of(kind);
        for (layer, entry) in &catalog.layers {
            for variant in entry.variants.iter().filter(|v| *v != NONE_VARIANT) {
                out.insert((*layer, catalog.asset_path(kind, *layer, variant)));
            }
        }
    }
    out
}

fn placeholders(svg: &str) -> BTreeSet<&str> {
    svg.split("{{").skip(1).filter_map(|rest| rest.split_once("}}").map(|(key, _)| key.trim())).collect()
}

#[test]
fn every_catalog_variant_has_an_asset_and_every_asset_is_catalogued() {
    let referenced: BTreeSet<String> = catalog_assets().into_iter().map(|(_, path)| path).collect();
    let mut on_disk = BTreeSet::new();
    svgs_under(&Path::new(ASSETS_DIR).join(FORM_ASSET_ROOT), &mut on_disk);
    let missing: Vec<_> = referenced.difference(&on_disk).collect();
    let orphans: Vec<_> = on_disk.difference(&referenced).collect();
    assert!(missing.is_empty(), "catalog variants without an SVG:\n{missing:#?}");
    assert!(orphans.is_empty(), "SVGs no catalog lists:\n{orphans:#?}");
    assert!(on_disk.len() > 150, "expected the full form art set, saw {}", on_disk.len());
}

#[test]
fn bodies_define_the_silhouette_and_parts_use_form_placeholders_only() {
    let palette: BTreeSet<String> = {
        let raw = fs::read_to_string("assets/themes/simulacra-midnight.json").unwrap();
        let json: serde_json::Value = serde_json::from_str(&raw).unwrap();
        json["colors"].as_object().unwrap().keys().cloned().collect()
    };
    let mut failures = Vec::new();
    for (layer, path) in catalog_assets() {
        let svg = fs::read_to_string(Path::new(ASSETS_DIR).join(&path)).unwrap();
        if layer == FormLayer::Body && !svg.contains(CLIP_DEFINITION) {
            failures.push(format!("{path}: no {CLIP_DEFINITION}"));
        }
        for key in placeholders(&svg) {
            if !palette.contains(key) && !FORM_PLACEHOLDERS.contains(&key) {
                failures.push(format!("{path}: placeholder {{{{{key}}}}}"));
            }
        }
    }
    let mut portrait = BTreeSet::new();
    svgs_under(Path::new(PORTRAIT_DIR), &mut portrait);
    for path in portrait {
        let svg = fs::read_to_string(Path::new(ASSETS_DIR).join(&path)).unwrap();
        if placeholders(&svg).iter().any(|key| FORM_PLACEHOLDERS.contains(key)) {
            failures.push(format!("{path}: form placeholder in a portrait part"));
        }
    }
    let skin_in_forms = catalog_assets().into_iter().any(|(_, path)| {
        let svg = fs::read_to_string(Path::new(ASSETS_DIR).join(&path)).unwrap();
        placeholders(&svg).iter().any(|key| PORTRAIT_ONLY.contains(key))
    });
    assert!(!skin_in_forms, "form parts must use {{{{primary}}}}/{{{{secondary}}}}, not skin/hair");
    assert!(failures.is_empty(), "form asset violations:\n{}", failures.join("\n"));
}

#[test]
fn every_kind_form_layer_and_variant_is_labelled() {
    let english = fs::read_to_string(ENGLISH).unwrap();
    let has = |key: &str| english.lines().any(|line| line.trim_start().starts_with(&format!("{key}:")));
    let mut missing = Vec::new();
    for kind in FormKind::ALL {
        missing.extend(Some(kind.label_key()).filter(|key| !has(key)));
        for (layer, entry) in &FormCatalog::of(kind).layers {
            missing.extend(Some(layer.label_key()).filter(|key| !has(key)));
            missing.extend(entry.variants.iter().map(|v| layer.variant_label_key(v)).filter(|key| !has(key)));
        }
    }
    missing.sort();
    missing.dedup();
    assert!(missing.is_empty(), "unlabelled form keys:\n{}", missing.join("\n"));
}
