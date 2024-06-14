use std::ops::{Deref, DerefMut};

use interface::{PureDeref, PureDerefMut, default_impl_pure_deref};

use crate::traits::*;
use crate::internal::*;


pub struct BasicArena<T> {
    inner: UnsafeArena<T>
}

pub struct BasicArenaRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    inner: UnsafeArenaRef<T>
}

pub struct BasicArenaMutRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    inner: UnsafeArenaRef<T>
}


impl<T> BasicArena<T> {
    pub fn new(capacity: usize) -> Self {
        BasicArena {
            inner: UnsafeArena::new(capacity)
        }
    }
}

impl<T> Arena<T> for BasicArena<T> {
    type Ref<'arena> = BasicArenaRef<'arena, T> where T: 'arena;
    type MutRef<'arena>  = BasicArenaMutRef<'arena, T> where T: 'arena;

    fn alloc<'arena>(&'arena self, t: T) -> BasicArenaRef<'arena, T>{
        BasicArenaRef {
            inner: self.inner.alloc(t),
            arena: self,
        }
    }

    fn alloc_mut<'arena>(&'arena self, t: T) -> BasicArenaMutRef<'arena, T> {
        BasicArenaMutRef {
            inner: self.inner.alloc(t),
            arena: self
        }
    }
    
    fn copy<'arena>(&'arena self, r: &BasicArenaRef<'arena, T>) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.alloc_mut(unsafe {
            self.inner.get(&r.inner)
        }.clone())
    }
    
    fn copy_mut<'arena>(&'arena self, r: &BasicArenaMutRef<'arena, T>) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.alloc_mut(unsafe {
            self.inner.get(&r.inner)
        }.clone())
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

default_impl_pure_deref!(unsafe impl<'arena, T> PureDeref for BasicArenaRef<'arena, T>);
default_impl_pure_deref!(unsafe impl<'arena, T> PureDeref for BasicArenaMutRef<'arena, T>);
default_impl_pure_deref!(unsafe impl<'arena, T> PureDerefMut for BasicArenaMutRef<'arena, T>);

impl<'arena, T> ArenaRef<'arena, T> for BasicArenaRef<'arena, T> {
    type In =  BasicArena<T>;

    fn make_mut(&self) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.arena.copy(self)
    }
} 

impl<'arena, T> ArenaRef<'arena, T> for BasicArenaMutRef<'arena, T> {
    type In = BasicArena<T>;

    fn make_mut(&self) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.arena.copy_mut(self)
    }
}

impl<'arena, T> ArenaImmutRef<'arena, T> for BasicArenaRef<'arena, T> {}

impl<'arena, T> ArenaMutRef<'arena, T> for BasicArenaMutRef<'arena, T> {
    fn freeze(self) -> BasicArenaRef<'arena, T> {
        let BasicArenaMutRef { arena, inner } = self;

        BasicArenaRef {
            arena, inner
        }
    }
}
