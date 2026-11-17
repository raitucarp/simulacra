//! Running the catalogue query: full-text search and ordering in SQLite (FTS5) when the
//! vault is attached, the same query in memory otherwise; facet counts in memory.

use super::VaultPanel;
use crate::domain::document::facets::{facet_counts, Facet, FacetCount};
use crate::domain::document::query::{run, CatalogueQuery, RankingSignals};
use crate::domain::GroundingDocument;
use gpui_kit::Context;
use std::collections::HashSet;

impl VaultPanel {
    /// Recomputes the ordered results and the text matches of the current query.
    pub(in crate::ui::features::vault) fn refresh_results(&mut self) {
        let text_only = CatalogueQuery { selection: Default::default(), ..self.query.clone() };
        self.results = self.ids_for(&self.query);
        self.text_matches = (!self.query.terms().is_empty()).then(|| self.ids_for(&text_only).into_iter().collect());
    }

    fn ids_for(&self, query: &CatalogueQuery) -> Vec<String> {
        let searched = self.data.vault.as_ref().and_then(|vault| vault.search_catalogue(query, &self.data.tree).ok());
        searched.unwrap_or_else(|| {
            let signals = RankingSignals { link_counts: Some(&self.data.link_counts) };
            run(&self.data.documents, query, &self.data.tree, signals).into_iter().map(|d| d.id.clone()).collect()
        })
    }

    /// Updates the search text and the results.
    pub(in crate::ui::features::vault) fn set_search_text(&mut self, text: String, cx: &mut Context<Self>) {
        if self.query.text != text {
            self.query.text = text;
            self.refresh_results();
            cx.notify();
        }
    }

    /// Replaces the query (facets, sort) and refreshes the results.
    pub(in crate::ui::features::vault) fn set_query(&mut self, query: CatalogueQuery, cx: &mut Context<Self>) {
        self.query = query;
        self.refresh_results();
        cx.notify();
    }

    /// The result documents, in order.
    pub(in crate::ui::features::vault) fn result_documents(&self) -> Vec<&GroundingDocument> {
        self.results.iter().filter_map(|id| self.data.document(id)).collect()
    }

    /// The documents the facet counts are taken over (all, or those matching the text).
    fn count_base(&self) -> Vec<&GroundingDocument> {
        let matches: Option<&HashSet<String>> = self.text_matches.as_ref();
        self.data.documents.iter().filter(|doc| matches.is_none_or(|ids| ids.contains(&doc.id))).collect()
    }

    /// Values of `facet` with their counts under the rest of the query.
    pub(in crate::ui::features::vault) fn facet_values(&self, facet: Facet) -> Vec<FacetCount> {
        facet_counts(&self.count_base(), &self.query.selection, facet, &self.data.tree)
    }
}
