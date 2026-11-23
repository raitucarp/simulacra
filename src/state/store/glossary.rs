//! N17: the glossary in the store. The user defines a term in one of their posts (never a
//! locked one; each term in one post only), or stops defining it there. The glossary the
//! user reads is built from every published post; the one an agent reads holds only the
//! definitions in posts it may read (N8).

use super::{require_persona, AppState};
use crate::domain::post::glossary::{clean_definition, clean_term, term_key, DefineProblem, Definition, Glossary};
use crate::domain::{PostId, ProfileId};
use jiff::Timestamp;

impl AppState {
    fn defining_position(&self, id: &PostId) -> Result<usize, DefineProblem> {
        let position = self.post_position(id).ok_or(DefineProblem::NotYours)?;
        require_persona(self.author_of(&self.posts[position])).map_err(|_| DefineProblem::NotYours)?;
        if self.posts[position].is_locked() {
            return Err(DefineProblem::Locked); // X2: a definition would speak past the lock
        }
        Ok(position)
    }

    fn save_definition(&mut self, position: usize, definition: Option<Definition>) -> Result<(), DefineProblem> {
        self.posts[position].extras.defines = definition.map(Box::new);
        match &self.db {
            Some(db) => db.save_post(&self.posts[position]).map_err(|err| DefineProblem::Storage(err.to_string())),
            None => Ok(()),
        }
    }

    /// Makes the user's post `id` where `term` is defined as `text`.
    pub fn define_term(&mut self, id: &PostId, term: &str, text: &str, now: Timestamp) -> Result<Definition, DefineProblem> {
        let position = self.defining_position(id)?;
        let term = clean_term(term).ok_or(DefineProblem::NoTerm)?;
        let text = clean_definition(text).ok_or(DefineProblem::NoDefinition)?;
        let key = term_key(&term);
        let taken = self.posts.iter().find(|post| post.id != *id && post.extras.defines.as_ref().is_some_and(|other| term_key(&other.term) == key));
        if let Some(other) = taken {
            return Err(DefineProblem::Taken(other.id.clone()));
        }
        let definition = Definition { term, text, at: now };
        self.save_definition(position, Some(definition.clone()))?;
        Ok(definition)
    }

    /// The user's post `id` no longer defines its term.
    pub fn undefine_term(&mut self, id: &PostId) -> Result<(), DefineProblem> {
        let position = self.post_position(id).ok_or(DefineProblem::NotYours)?;
        require_persona(self.author_of(&self.posts[position])).map_err(|_| DefineProblem::NotYours)?;
        self.save_definition(position, None)
    }

    /// Every defined term, as the user reads them.
    pub fn glossary(&self) -> Glossary {
        Glossary::from_posts(&self.posts)
    }

    /// The terms defined in posts `agent` may read.
    pub fn glossary_for(&self, agent: &ProfileId) -> Glossary {
        Glossary::from_posts(self.posts.iter().filter(|post| self.may_read(agent, post)))
    }
}
