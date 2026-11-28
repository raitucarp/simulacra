//! Command registry: [`CommandId`] and its pure data rows ([`CommandSpec`]).

mod category;
mod ids;
mod specs;

#[cfg(test)]
mod tests;

pub use category::CommandCategory;
pub use ids::CommandId;
pub use specs::{spec, Availability, CommandKind, CommandSpec, COMMANDS};
