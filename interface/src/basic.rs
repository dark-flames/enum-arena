use std::ops::{Deref, DerefMut};

use crate::internal::*;
use crate::traits::*;

pub struct BasicArena<T> {
    inner: UnsafeArena<T>,
}

pub struct BasicArenaRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    inner: UnsafeArenaRef<T>,
}

pub struct BasicArenaMutRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    inner: UnsafeArenaRef<T>,
}

impl<T> BasicArena<T> {
    pub fn new(capacity: usize) -> Self {
        BasicArena {
            inner: UnsafeArena::new(capacity),
        }
    }
}

impl<T> Arena<T> for BasicArena<T> {
    type Ref<'arena> = BasicArenaRef<'arena, T> where T: 'arena;
    type MutRef<'arena>  = BasicArenaMutRef<'arena, T> where T: 'arena;

    fn alloc(&self, t: T) -> BasicArenaRef<'_, T> {
        BasicArenaRef {
            inner: self.inner.alloc(t),
            arena: self,
        }
    }

    fn alloc_mut(&self, t: T) -> BasicArenaMutRef<'_, T> {
        BasicArenaMutRef {
            inner: self.inner.alloc(t),
            arena: self,
        }
    }

    fn copy<'arena>(&'arena self, r: &BasicArenaRef<'arena, T>) -> BasicArenaMutRef<'arena, T>
    where
        T: Clone,
    {
        self.alloc_mut(unsafe { self.inner.get(&r.inner) }.clone())
    }

    fn copy_mut<'arena>(
        &'arena self,
        r: &BasicArenaMutRef<'arena, T>,
    ) -> BasicArenaMutRef<'arena, T>
    where
        T: Clone,
    {
        self.alloc_mut(unsafe { self.inner.get(&r.inner) }.clone())
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn capacity(&self) -> usize {
        self.inner.capacity()
    }
}

impl<'arena, T> Deref for BasicArenaRef<'arena, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.arena.inner.get(&self.inner) }
    }
}

impl<'arena, T> Deref for BasicArenaMutRef<'arena, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.arena.inner.get(&self.inner) }
    }
}

impl<'arena, T> DerefMut for BasicArenaMutRef<'arena, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.arena.inner.get_mut(&self.inner) }
    }
}

impl<'arena, T> ArenaRef<'arena, T> for BasicArenaRef<'arena, T> {
    type In = BasicArena<T>;

    fn make_mut(&self) -> BasicArenaMutRef<'arena, T>
    where
        T: Clone,
    {
        self.arena.copy(self)
    }
}

impl<'arena, T> ArenaRef<'arena, T> for BasicArenaMutRef<'arena, T> {
    type In = BasicArena<T>;

    fn make_mut(&self) -> BasicArenaMutRef<'arena, T>
    where
        T: Clone,
    {
        self.arena.copy_mut(self)
    }
}

impl<'arena, T> ArenaImmutRef<'arena, T> for BasicArenaRef<'arena, T> {}

impl<'arena, T> ArenaMutRef<'arena, T> for BasicArenaMutRef<'arena, T> {
    fn freeze(self) -> BasicArenaRef<'arena, T> {
        let BasicArenaMutRef { arena, inner } = self;

        BasicArenaRef { arena, inner }
    }
}
