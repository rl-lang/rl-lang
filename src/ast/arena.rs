//! Generic index-based arena for AST nodes.
//!
//! Replaces `Box<T>` with a `Copy` integer handle (`Id<T>`) into a flat
//! `Vec<T>` owned by an `Arena<T>`. Every `Id<T>` is tagged with the id of
//! the arena it was allocated from, so using it against the wrong arena
//! panics immediately at the access site instead of silently reading
//! unrelated data or panicking later with a confusing out-of-bounds error.

use std::fmt;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU32, Ordering};

/// Global counter handing out a unique id to every `Arena<T>` that's ever
/// constructed, so `Id<T>`s can be checked against their origin arena.
static NEXT_ARENA_ID: AtomicU32 = AtomicU32::new(0);

/// A typed, `Copy` handle into a specific [`Arena<T>`].
///
/// Two `Id<T>`s are only meaningfully comparable if they came from the same
/// arena - `arena_id` is what lets `Arena::get`/`get_mut` detect and reject
/// a handle that was allocated somewhere else.
pub struct Id<T> {
    index: u32,
    arena_id: u32,
    _marker: PhantomData<T>,
}

impl<T> Id<T> {
    #[inline]
    fn new(index: u32, arena_id: u32) -> Self {
        Self {
            index,
            arena_id,
            _marker: PhantomData,
        }
    }

    #[inline]
    pub fn index(self) -> usize {
        self.index as usize
    }

    #[inline]
    pub fn arena_id(self) -> u32 {
        self.arena_id
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
        self.index == other.index && self.arena_id == other.arena_id
    }
}
impl<T> Eq for Id<T> {}
impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.arena_id.hash(state);
    }
}
impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({}@arena{})", self.index, self.arena_id)
    }
}

/// Flat backing store for `T`, indexed by [`Id<T>`].
#[derive(Debug)]
pub struct Arena<T> {
    id: u32,
    items: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            id: NEXT_ARENA_ID.fetch_add(1, Ordering::Relaxed),
            items: Vec::new(),
        }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            id: NEXT_ARENA_ID.fetch_add(1, Ordering::Relaxed),
            items: Vec::with_capacity(cap),
        }
    }

    /// This arena's unique id, e.g. for logging which arena an `Id` came from.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Insert a node, returning its id.
    pub fn alloc(&mut self, item: T) -> Id<T> {
        let index = self.items.len() as u32;
        self.items.push(item);
        Id::new(index, self.id)
    }

    pub fn get(&self, id: Id<T>) -> &T {
        self.check_owner(id, "get");
        let index = id.index();
        if index >= self.items.len() {
            panic!(
                "Arena<{}>: index {} out of bounds (len {})",
                std::any::type_name::<T>(),
                index,
                self.items.len()
            );
        }
        &self.items[index]
    }

    pub fn get_mut(&mut self, id: Id<T>) -> &mut T {
        self.check_owner(id, "get_mut");
        let index = id.index();
        let len = self.items.len();
        if index >= len {
            panic!(
                "Arena<{}>: index {} out of bounds (len {})",
                std::any::type_name::<T>(),
                index,
                len
            );
        }
        &mut self.items[index]
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Panics with a clear message if `id` didn't come from this arena.
    /// This is the check that turns cross-arena bugs into an obvious,
    /// immediate panic instead of silent corruption or a confusing
    /// out-of-bounds error somewhere else.
    #[inline]
    fn check_owner(&self, id: Id<T>, op: &str) {
        if id.arena_id != self.id {
            panic!(
                "Arena<{}>::{}: Id belongs to arena {} but this arena is {} - cross-arena access",
                std::any::type_name::<T>(),
                op,
                id.arena_id,
                self.id
            );
        }
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
