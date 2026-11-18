//! Cached chart geometry: built once per key (data generation, frame size, viewport) and
//! reused by every repaint and hit test until the key changes, so hovering never recomputes
//! positions.

use std::cell::RefCell;
use std::rc::Rc;

/// One cached value and the key it was built for.
pub struct GeometryCache<K, G> {
    slot: RefCell<Option<(K, Rc<G>)>>,
}

impl<K, G> Default for GeometryCache<K, G> {
    fn default() -> Self {
        Self { slot: RefCell::new(None) }
    }
}

impl<K: PartialEq + Clone, G> GeometryCache<K, G> {
    /// The value for `key`, building it with `build` only when the key changed.
    pub fn get(&self, key: &K, build: impl FnOnce() -> G) -> Rc<G> {
        if let Some((cached, value)) = self.slot.borrow().as_ref() {
            if cached == key {
                return Rc::clone(value);
            }
        }
        let value = Rc::new(build());
        *self.slot.borrow_mut() = Some((key.clone(), Rc::clone(&value)));
        value
    }

    /// The last built value, whatever its key.
    pub fn last(&self) -> Option<Rc<G>> {
        self.slot.borrow().as_ref().map(|(_, value)| Rc::clone(value))
    }

    /// Forgets the cached value.
    pub fn clear(&self) {
        *self.slot.borrow_mut() = None;
    }
}
