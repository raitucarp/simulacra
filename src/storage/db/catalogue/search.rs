//! Pure SQL building for catalogue searches: the FTS5 match expression, one `IN` clause per
//! chosen facet, and the sort order. [`Database::search_catalogue`] runs the result.

use super::search_sql::{facet_clause, order_clause, BASE_SELECT, FTS_JOIN, NETWORK_FILTER, NO_SCORE, TEXT_SCORE, WHERE};
use crate::domain::document::facets::Facet;
use crate::domain::document::query::CatalogueQuery;
use crate::domain::document::vocabulary::SubjectTree;
use crate::domain::document::SubjectId;
use crate::error::Result;
use crate::storage::db::{collect_rows, db_err, Database};
use rusqlite::params_from_iter;

/// A parameterised search statement (every parameter is text).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogueSearch {
    pub sql: String,
    pub params: Vec<String>,
}

/// The FTS5 expression for `query`'s terms: every term required, each as a quoted prefix
/// (`"spino"*`), so user text can never inject FTS syntax. `None` without terms.
pub fn fts_match_expression(query: &CatalogueQuery) -> Option<String> {
    let terms = query.terms();
    (!terms.is_empty()).then(|| terms.iter().map(|term| format!("\"{}\"*", term.replace('"', ""))).collect::<Vec<_>>().join(" "))
}

/// Values of `facet` as stored: chosen subjects widen to their narrower terms.
fn stored_values(query: &CatalogueQuery, facet: Facet, tree: &SubjectTree) -> Vec<String> {
    let chosen = query.selection.values(facet);
    match facet {
        Facet::Subject => {
            let mut all: Vec<String> = chosen
                .iter()
                .flat_map(|id| {
                    let below = tree.descendants(&SubjectId::new(*id));
                    if below.is_empty() { vec![id.to_string()] } else { below.iter().map(|s| s.as_str().to_string()).collect() }
                })
                .collect();
            all.sort();
            all.dedup();
            all
        }
        Facet::Language => chosen.iter().map(|value| value.to_lowercase()).collect(),
        _ => chosen.iter().map(|value| value.to_string()).collect(),
    }
}

/// The statement returning `(id, score)` of every match, in the query's order.
pub fn build_catalogue_search(query: &CatalogueQuery, tree: &SubjectTree) -> CatalogueSearch {
    let fts = fts_match_expression(query);
    let mut params = Vec::new();
    let mut sql = format!("{BASE_SELECT}{}", if fts.is_some() { TEXT_SCORE } else { NO_SCORE });
    match fts {
        Some(expression) => {
            sql.push_str(FTS_JOIN);
            params.push(expression);
        }
        None => sql.push_str(WHERE),
    }
    sql.push_str(NETWORK_FILTER);
    params.push(query.network_id.as_str().to_string());
    for facet in query.selection.facets() {
        let values = stored_values(query, facet, tree);
        sql.push_str(&facet_clause(facet, values.len()));
        params.extend(values);
    }
    let (key, descending) = query.effective_sort();
    sql.push_str(&order_clause(key, descending));
    CatalogueSearch { sql, params }
}

impl Database {
    /// Ids of the documents matching `query`, in its order (relevance is the FTS5 rank).
    pub fn search_catalogue(&self, query: &CatalogueQuery, tree: &SubjectTree) -> Result<Vec<String>> {
        let search = build_catalogue_search(query, tree);
        let mut stmt = self.conn.prepare(&search.sql).map_err(db_err)?;
        let rows = stmt.query_map(params_from_iter(search.params.iter()), |row| row.get::<_, String>(0)).map_err(db_err)?;
        collect_rows(rows)
    }
}
