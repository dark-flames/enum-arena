use std::{cell::UnsafeCell, ops::{Deref, DerefMut}};

use interface::{PureDeref, PureDerefMut, default_impl_pure_deref};

use crate::traits::*;


pub struct BasicArena<T> {
    chunks: UnsafeCell<Vec<Vec<T>>>,
    capacity: usize
}

pub struct BasicArenaRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    chunk: usize,
    elem: usize
}

pub struct BasicArenaMutRef<'arena, T> {
    arena: &'arena BasicArena<T>,
    chunk: usize,
    elem: usize
}


impl<T> BasicArena<T> {
    pub fn new(capacity: usize) -> Self {
        BasicArena {
            chunks: UnsafeCell::new(vec![Vec::with_capacity(capacity)]),
            capacity
        }
    }

    unsafe fn get(&self, chunk: usize, element: usize) -> &T {
        & *(self.get_raw(chunk, element))
    }

    unsafe fn get_mut(&self, chunk: usize, element: usize) -> &mut T {
        &mut *(self.get_raw(chunk, element))
    }

    unsafe fn get_raw(&self, chunk: usize, element: usize) -> *mut T {
        let chunks = &mut *(self.chunks.get());
        chunks.get_unchecked_mut(chunk).get_unchecked_mut(element) as *mut T
    }

    fn __alloc<'arena>(&'arena self, t: T) -> (usize, usize) {
        let chunks = unsafe {
            &mut *(self.chunks.get())
        };
        let chunks_count = chunks.len();
        let chunk = chunks.last_mut().unwrap();

        let (chunk_id, element_id, chunk) = if chunk.capacity() == 0 {
            chunks.push(Vec::with_capacity(self.capacity));
            (chunks_count, 0, chunks.last_mut().unwrap())
        } else {
            (chunks_count - 1, chunk.len() - 1, chunk)
        };

        chunk.push(t);

        (chunk_id, element_id)
    }
}

impl<T> Arena<T> for BasicArena<T> {
    type Ref<'arena> = BasicArenaRef<'arena, T> where T: 'arena;
    type MutRef<'arena>  = BasicArenaMutRef<'arena, T> where T: 'arena;

    fn alloc<'arena>(&'arena self, t: T) -> BasicArenaRef<'arena, T>{
        let (chunk, elem) = self.__alloc(t);

        BasicArenaRef {
            arena: self,
            chunk,
            elem
        }
    }

    fn alloc_mut<'arena>(&'arena self, t: T) -> BasicArenaMutRef<'arena, T> {
        let (chunk, elem) = self.__alloc(t);

        BasicArenaMutRef {
            arena: self,
            chunk,
            elem
        }
    }
    
    fn copy<'arena>(&'arena self, r: &BasicArenaRef<'arena, T>) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.alloc_mut(unsafe {
            self.get(r.chunk, r.elem)
        }.clone())
    }
    
    fn copy_mut<'arena>(&'arena self, r: &BasicArenaMutRef<'arena, T>) -> BasicArenaMutRef<'arena, T> where T: Clone {
        self.alloc_mut(unsafe {
            self.get(r.chunk, r.elem)
        }.clone())
    }
}

impl<'arena, T> Deref for BasicArenaRef<'arena, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.arena.get(self.chunk, self.elem) }
    }
}

impl<'arena, T> Deref for BasicArenaMutRef<'arena, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.arena.get(self.chunk, self.elem) }
    }
}

impl<'arena, T> DerefMut for BasicArenaMutRef<'arena, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.arena.get_mut(self.chunk, self.elem) }
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
        let BasicArenaMutRef { arena, chunk, elem: element } = self;

        BasicArenaRef {
            arena, chunk, elem: element
        }
    }
}

