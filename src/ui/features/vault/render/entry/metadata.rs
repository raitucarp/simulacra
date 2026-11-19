//! The abstract and the descriptive metadata table of an entry.

use super::super::common::{card, section_heading, titled_section};
use crate::domain::document::text::reading_minutes;
use crate::domain::GroundingDocument;
use crate::i18n::t_count;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::{kind_label, role_label, scheme_label};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use crate::ui::features::vault::text::tr;
use rust_i18n::t;

/// Date format of the "added" row.
const ADDED_FORMAT: &str = "%Y-%m-%d";
const LIST_JOINER: &str = ", ";

/// (label key, value) rows of `doc`, empty values left out.
fn metadata_rows(doc: &GroundingDocument) -> Vec<(&'static str, String)> {
    let record = &doc.record;
    let creators = record.creators.iter().map(|c| format!("{} ({})", c.name, role_label(c.role).to_lowercase())).collect::<Vec<_>>().join(LIST_JOINER);
    let identifier = record.identifier.scheme.filter(|_| !record.identifier.is_empty()).map(|s| format!("{} {}", scheme_label(s), record.identifier.value));
    let extent = [
        record.extent.pages.map(|p| t_count("vault.count.pages", p as usize)),
        Some(t_count("vault.count.words", record.extent.words)),
        (record.extent.words > 0).then(|| t!("vault.minutes_read", minutes = reading_minutes(record.extent.words)).to_string()),
    ];
    let added_by = record.provenance.added_by.trim();
    let added = doc.created_at.strftime(ADDED_FORMAT).to_string();
    let origin = tr(format!("vault.origin.{}", record.provenance.origin.key()));
    let source = record.provenance.source.trim();
    vec![
        ("type", kind_label(record.kind)),
        ("creators", creators),
        ("date", record.date.clone()),
        ("publisher", record.publisher.clone()),
        ("language", record.language.to_uppercase()),
        ("format", record.effective_format().to_string()),
        ("identifier", identifier.unwrap_or_default()),
        ("extent", extent.into_iter().flatten().collect::<Vec<_>>().join(LIST_JOINER)),
        ("rights", record.rights.clone()),
        ("added", if added_by.is_empty() { added } else { t!("vault.added_by", date = added, name = added_by).to_string() }),
        ("origin", if source.is_empty() { origin } else { format!("{origin}{LIST_JOINER}{source}") }),
        ("checksum", record.provenance.checksum.clone()),
    ]
    .into_iter()
    .filter(|(_, value)| !value.trim().is_empty())
    .collect()
}

impl VaultPanel {
    pub(super) fn render_metadata(&self, doc: &GroundingDocument) -> impl IntoElement {
        let theme = &self.theme;
        let rows = metadata_rows(doc).into_iter().map(|(key, text)| {
            let mono = key == "checksum" || key == "format";
            let value = div().flex_1().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary);
            let value = if mono { value.font_family(theme.font_family_mono.clone()).text_color(theme.text_secondary) } else { value };
            div()
                .flex()
                .gap(theme.control_gap)
                .py(theme.spacing_2xs)
                .border_b(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .child(div().w(theme.vault_meta_label_width).flex_shrink_0().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(format!("vault.meta.{key}"))))
                .child(value.child(text))
        });
        card(theme).w_full().child(section_heading(t!("vault.record").to_string(), theme)).children(rows)
    }

    pub(super) fn render_abstract(&self, doc: &GroundingDocument) -> Option<impl IntoElement> {
        let theme = &self.theme;
        let text = doc.record.description.trim();
        (!text.is_empty()).then(|| {
            div().flex().flex_col().gap(theme.stack_gap).child(titled_section(LucideIcon::Quote, t!("vault.abstract").to_string(), theme)).child(
                div()
                    .pl(theme.card_padding)
                    .py(theme.spacing_sm)
                    .border_l(theme.border_width_threadline)
                    .border_color(theme.accent_dialectic)
                    .bg(theme.bg_callout)
                    .rounded(theme.radius_md)
                    .font_family(theme.font_family_body.clone())
                    .text_size(theme.font_size_md)
                    .line_height(relative(theme.line_height_reading))
                    .text_color(theme.text_primary)
                    .child(text.to_string()),
            )
        })
    }
}
