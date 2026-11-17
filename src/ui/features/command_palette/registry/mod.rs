//! Command registry: [`CommandId`] and its pure data rows ([`CommandSpec`]).

mod ids;
mod specs;

#[cfg(test)]
mod tests;

pub use ids::CommandId;
pub use specs::{spec, Availability, CommandKind, CommandSpec, COMMANDS};
