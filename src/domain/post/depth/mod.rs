//! N5: the depth dial. An agent re-renders a post simpler or more expert; each rendering is
//! kept as a version of the post (V epic) edited from the version it renders, so renderings
//! sit beside that version's other edits in its tree and never become new posts. The
//! version's record names the depth ([`Depth`]); the version the post shows is unchanged.
//! A locked post is never rendered: its text is sealed, and agents only ever see the
//! placeholder.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// How a rendering reads compared with the post as written. The key is stored in the
    /// version's record: never rename one.
    Depth {
        /// Plainer words, shorter sentences, one concrete picture.
        Simpler => "simpler",
        /// Denser and technical, for someone who knows the field.
        Expert => "expert",
    }
}

#[cfg(test)]
mod tests;
