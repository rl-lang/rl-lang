//! Generic index-based arena for AST nodes.
//!
//! Replaces `Box<T>` with a `Copy` integer id (`Id<T>`) into a flat `Vec<T>`
//! owned by an `Arena<T>`. This removes per-node heap allocations, makes AST
//! node references cheap to store/compare/hash, and allows in-place mutation
//! of nodes during passes (e.g. constant folding) without re-boxing.

use std::fmt;
use std::marker::PhantomData;

/// A typed, `Copy` index into an [`Arena<T>`].
pub struct Id<T>(u32, PhantomData<T>);

impl<T> Id<T> {
    #[inline]
    fn new(index: u32) -> Self {
        Self(index, PhantomData)
    }

    #[inline]
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Id<T> {}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

/// Flat backing store for `T`, indexed by [`Id<T>`].
#[derive(Debug)]
pub struct Arena<T> {
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            items: Vec::with_capacity(cap),
        }
    }

    /// Insert a node, returning its id.
    pub fn alloc(&mut self, item: T) -> Id<T> {
        let idx = self.items.len() as u32;
        self.items.push(item);
        Id::new(idx)
    }

    pub fn get(&self, id: Id<T>) -> &T {
        let index = id.index();
        if index >= self.items.len() {
            panic!(
                "Arena index out of bounds: trying to access index {} but arena only has {} items.",
                index,
                self.items.len()
            );
        }
        &self.items[index]
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        &mut self.items[id.index()]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> std::ops::Index<Id<T>> for Arena<T> {
    type Output = T;
    fn index(&self, id: Id<T>) -> &T {
        self.get(id)
    }
}

impl<T> std::ops::IndexMut<Id<T>> for Arena<T> {
    fn index_mut(&mut self, id: Id<T>) -> &mut T {
        self.get_mut(id)
    }
}
