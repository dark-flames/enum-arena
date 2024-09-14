use std::ops::{Deref, DerefMut};

pub trait ArenaRef<'arena, T>: Deref {
    type In: Arena<T> + 'arena;

    fn make_mut(&self) -> <Self::In as Arena<T>>::MutRef<'arena>
    where
        T: Clone;
}

pub trait ArenaImmutRef<'arena, T>: ArenaRef<'arena, T> {}

pub trait ArenaMutRef<'arena, T>: ArenaRef<'arena, T> + DerefMut {
    fn freeze(self) -> <Self::In as Arena<T>>::Ref<'arena>;
}

pub trait Arena<T> {
    type Ref<'arena>: ArenaImmutRef<'arena, T, In = Self>
    where
        Self: 'arena;
    type MutRef<'arena>: ArenaMutRef<'arena, T, In = Self>
    where
        Self: 'arena;

    fn alloc(&self, t: T) -> Self::Ref<'_>;

    fn alloc_mut(&self, t: T) -> Self::MutRef<'_>;

    fn copy<'arena>(&'arena self, r: &Self::Ref<'arena>) -> Self::MutRef<'arena>
    where
        T: Clone;

    fn copy_mut<'arena>(&'arena self, r: &Self::MutRef<'arena>) -> Self::MutRef<'arena>
    where
        T: Clone;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn capacity(&self) -> usize;
}

pub trait EnumRef<'arena, E, T>: ArenaRef<'arena, T>
where
    <Self as ArenaRef<'arena, T>>::In: EnumArena<E> + 'arena,
{
}

pub trait EnumImmutRef<'arena, E, T>: EnumRef<'arena, E, T> + ArenaImmutRef<'arena, T>
where
    <Self as ArenaRef<'arena, T>>::In: EnumArena<E> + 'arena,
{
}

pub trait EnumMutRef<'arena, E, T>: EnumRef<'arena, E, T> + ArenaMutRef<'arena, T>
where
    <Self as ArenaRef<'arena, T>>::In: EnumArena<E> + 'arena,
{
}

pub trait EnumArena<E>: Arena<E> {
    type RefOf<'arena, T>: EnumImmutRef<'arena, E, T, In = Self>
    where
        Self: Arena<T, Ref<'arena> = Self::RefOf<'arena, T>> + 'arena;
    type MutRefOf<'arena, T>: EnumMutRef<'arena, E, T, In = Self>
    where
        Self: Arena<T, MutRef<'arena> = Self::MutRefOf<'arena, T>> + 'arena;

    fn alloc<'arena, T>(&'arena self, t: T) -> Self::RefOf<'arena, T>
    where
        Self: Arena<T, Ref<'arena> = Self::RefOf<'arena, T>>,
    {
        Arena::<T>::alloc(self, t)
    }

    fn alloc_mut<'arena, T>(&'arena self, t: T) -> Self::MutRefOf<'arena, T>
    where
        Self: Arena<T, MutRef<'arena> = Self::MutRefOf<'arena, T>>,
    {
        Arena::<T>::alloc_mut(self, t)
    }

    fn copy<'arena, T>(&'arena self, r: &Self::RefOf<'arena, T>) -> Self::MutRefOf<'arena, T>
    where
        T: Clone,
        Self: Arena<T, Ref<'arena> = Self::RefOf<'arena, T>>,
        Self: Arena<T, MutRef<'arena> = Self::MutRefOf<'arena, T>>,
    {
        Arena::<T>::copy(self, r)
    }

    fn copy_mut<'arena, T>(&'arena self, r: &Self::MutRefOf<'arena, T>) -> Self::MutRefOf<'arena, T>
    where
        T: Clone,
        Self: Arena<T, MutRef<'arena> = Self::MutRefOf<'arena, T>>,
    {
        Arena::<T>::copy_mut(self, r)
    }
}
