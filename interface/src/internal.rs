use std::{cell::UnsafeCell, marker::PhantomData};

pub struct UnsafeArena<T> {
    chunks: UnsafeCell<Vec<Vec<T>>>,
    capacity: usize,
}

pub struct UnsafeArenaRef<T> {
    chunk: usize,
    elem: usize,
    __marker: PhantomData<T>,
}

impl<T> UnsafeArena<T> {
    pub fn new(capacity: usize) -> Self {
        UnsafeArena {
            chunks: UnsafeCell::new(vec![Vec::with_capacity(capacity)]),
            capacity,
        }
    }

    /// Get raw pointer over `UnsafeArenaRef`.
    /// # Safety
    /// The calling is safe if `r` is constructed by the same arena.
    pub unsafe fn get_raw(&self, r: &UnsafeArenaRef<T>) -> *mut T {
        let chunks = &mut *(self.chunks.get());
        chunks.get_unchecked_mut(r.chunk).get_unchecked_mut(r.elem) as *mut T
    }

    /// Get primitive reference over `UnsafeArenaRef`.
    /// # Safety
    /// The calling is safe if `r` is constructed by the same arena.
    pub unsafe fn get<'arena>(&self, r: &UnsafeArenaRef<T>) -> &'arena T {
        &*self.get_raw(r)
    }

    /// Get mutable primitive reference over `UnsafeArenaRef`.
    /// # Safety
    /// The calling is safe if `r` is constructed by the same arena.
    pub unsafe fn get_mut<'arena>(&self, r: &UnsafeArenaRef<T>) -> &'arena mut T {
        &mut *self.get_raw(r)
    }

    pub fn alloc(&self, t: T) -> UnsafeArenaRef<T> {
        let chunks = unsafe { &mut *(self.chunks.get()) };
        let chunks_count = chunks.len();
        let chunk = chunks.last_mut().unwrap();

        let (chunk_id, element_id, chunk) = if chunk.capacity() == chunk.len() {
            chunks.push(Vec::with_capacity(self.capacity));
            (chunks_count, 0, chunks.last_mut().unwrap())
        } else {
            (chunks_count - 1, chunk.len(), chunk)
        };

        chunk.push(t);

        UnsafeArenaRef {
            chunk: chunk_id,
            elem: element_id,
            __marker: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        let chunks = unsafe { &*(self.chunks.get()) };
        let chunks_count = chunks.len();
        let chunk = chunks.last().unwrap();

        self.capacity * (chunks_count - 1) + chunk.len()
    }

    pub fn is_empty(&self) -> bool {
        let chunks = unsafe { &*(self.chunks.get()) };
        chunks.len() == 1 && chunks.last().unwrap().is_empty()
    }

    pub fn capacity(&self) -> usize {
        let chunks = unsafe { &*(self.chunks.get()) };
        let chunk = chunks.last().unwrap();

        chunk.capacity()
    }
}
