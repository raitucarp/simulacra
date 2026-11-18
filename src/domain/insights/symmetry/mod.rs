//! Group theory over threads and the post graph: which posts are structurally interchangeable,
//! how symmetric a thread is, and how the turn of speech rotates among its authors.
//!
//! What is exact and what is approximated:
//! - [`trees`]: a reply thread is a rooted tree. Its automorphism group (permutations of posts
//!   preserving "replies to" and the root) is computed **exactly** from canonical subtree
//!   classes (Aho–Hopcroft–Ullman): `|Aut(T)| = Π_v Π_c m(v, c)!`, where `m(v, c)` counts the
//!   children of `v` whose subtrees are isomorphic of class `c`. Orbits are exact too: two posts
//!   share an orbit iff the subtree classes along their paths from the root agree.
//! - [`refinement`]: on a general graph, exact orbits need canonical labelling (hard in the
//!   worst case), so colour refinement (1-dimensional Weisfeiler–Leman) **approximates** them.
//!   Its stable classes are never finer than the orbits: posts in different classes are surely
//!   not interchangeable; posts in one class are interchangeable on trees and forests (where
//!   refinement is exact) and usually, but not provably, elsewhere (regular graphs fool it).
//! - [`permutation`]: "whom does each author answer most" is a function on the authors of a
//!   debate. Restricted to its recurrent part it is a **permutation, exactly**: its cycle
//!   notation, order (least common multiple of cycle lengths) and sign are reported, and the
//!   authors that feed into a cycle without being on one are listed apart.

pub mod permutation;
pub mod refinement;
pub mod trees;

#[cfg(test)]
mod tests;

pub use permutation::{rotation, Rotation};
pub use refinement::{colour_refinement, equivalence_classes};
pub use trees::{tree_symmetry, TreeSymmetry};
