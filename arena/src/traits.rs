use interface::{PureDeref, PureDerefMut};

use std::ops::{DerefMut, Deref};

pub trait ArenaRef<'arena, T>: Deref + PureDeref {
    type In: Arena<T> + 'arena;

    fn make_mut(&self) -> <Self::In as Arena<T>>::MutRef<'arena> where T: Clone;
}

pub trait ArenaImmutRef<'arena, T> : ArenaRef<'arena, T>
    where <Self as ArenaRef<'arena, T>>::In: Arena<T,  Ref<'arena> = Self>
{

}

pub trait ArenaMutRef<'arena, T> : ArenaRef<'arena, T> + DerefMut + PureDerefMut
    where <Self as ArenaRef<'arena, T>>::In: Arena<T,  MutRef<'arena> = Self>
{
    fn freeze(self) -> <Self::In as Arena<T>>::Ref<'arena>;
}

pub trait Arena<T> {
    type Ref<'arena>: ArenaImmutRef<'arena, T, In = Self> where Self: 'arena;
    type MutRef<'arena>: ArenaMutRef<'arena, T, In = Self> where Self: 'arena;

    fn alloc<'arena>(&'arena self, t: T) -> Self::Ref<'arena>;

    fn alloc_mut<'arena>(&'arena self, t: T) -> Self::MutRef<'arena>;

    fn copy<'arena>(&'arena self, r: &Self::Ref<'arena>) -> Self::MutRef<'arena> where T: Clone;

    fn copy_mut<'arena>(&'arena self, r: &Self::MutRef<'arena>) -> Self::MutRef<'arena> where T: Clone;
}