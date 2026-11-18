//! Which statements stand: the grounded labelling of the map's argumentation framework.
//!
//! **Semantics.** Every node is an argument of a Dung framework whose attack relation is
//! the map's attack edges plus *supported attacks* (Cayrol and Lagasquie-Schiex, 2005): if
//! `a` supports `b`, directly or along a chain of supports, and `b` attacks `c`, then `a`
//! attacks `c` too. Support edges change nothing else.
//!
//! The labelling is the grounded one (Dung 1995, in Caminada's labelling form): an argument
//! is *accepted* (IN) once every attacker is rejected, *rejected* (OUT) once an accepted
//! argument attacks it; the steps repeat until nothing changes, and whatever is left is
//! *undecided* (UNDEC), for example both sides of an unbroken mutual attack. The accepted
//! set is the grounded extension: the least fixed point of the characteristic function, the
//! most sceptical position every reasonable reader shares.

use super::graph::Relations;
use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// Where a statement stands under the grounded semantics.
    Standing {
        /// In the grounded extension: every attack on it is answered.
        Accepted => "accepted",
        /// Defeated by an accepted statement.
        Rejected => "rejected",
        /// Neither (an unresolved conflict).
        Undecided => "undecided",
    }
}

/// The grounded labelling, one [`Standing`] per node index.
pub fn grounded_labelling(relations: &Relations) -> Vec<Standing> {
    let attacks = relations.effective_attacks();
    let attackers = Relations::attackers(&attacks);
    let mut labels: Vec<Option<Standing>> = vec![None; relations.len()];
    loop {
        let accept: Vec<usize> = (0..labels.len())
            .filter(|n| labels[*n].is_none() && attackers[*n].iter().all(|a| labels[*a] == Some(Standing::Rejected)))
            .collect();
        accept.iter().for_each(|n| labels[*n] = Some(Standing::Accepted));
        let reject: Vec<usize> = (0..labels.len())
            .filter(|n| labels[*n].is_none() && attackers[*n].iter().any(|a| labels[*a] == Some(Standing::Accepted)))
            .collect();
        reject.iter().for_each(|n| labels[*n] = Some(Standing::Rejected));
        if accept.is_empty() && reject.is_empty() {
            break;
        }
    }
    labels.into_iter().map(|label| label.unwrap_or(Standing::Undecided)).collect()
}
