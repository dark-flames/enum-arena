use std::{cell::UnsafeCell, marker::PhantomData};

pub struct UnsafeArena<T> {
    chunks: UnsafeCell<Vec<Vec<T>>>,
    capacity: usize
}

pub struct UnsafeArenaRef<T> {
    chunk: usize,
    elem: usize,
    __marker: PhantomData<T>
}

impl<T> UnsafeArena<T> {
    pub fn new(capacity: usize) -> Self {
        UnsafeArena {
            chunks: UnsafeCell::new(vec![Vec::with_capacity(capacity)]),
            capacity
        }
    }

    pub unsafe fn get_raw(&self, r: &UnsafeArenaRef<T>) -> *mut T {
        let chunks = &mut *(self.chunks.get());
        chunks.get_unchecked_mut(r.chunk).get_unchecked_mut(r.elem) as *mut T
    }

    pub unsafe fn get<'arena>(&self, r: &UnsafeArenaRef<T>) -> &'arena T {
        & *self.get_raw(r)
    }

    pub unsafe fn get_mut<'arena>(&self, r: &UnsafeArenaRef<T>) -> &'arena mut T {
        &mut *self.get_raw(r)
    }

    pub fn alloc(&self, t: T) -> UnsafeArenaRef<T> {
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

        UnsafeArenaRef {
            chunk: chunk_id,
            elem: element_id,
            __marker: Default::default()
        }
    }
}