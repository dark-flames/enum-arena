use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub unsafe trait PureDeref {
    type Target: ?Sized;

    fn pure_deref(&self) -> &Self::Target;
}

pub unsafe trait PureDerefMut: PureDeref {
    fn pure_deref_mut(&mut self) -> &mut Self::Target;
}

macro_rules! default_impl {
    (impl<$($generics: tt),*> PureDeref for $ty:ty) => {
        unsafe impl<$($generics),*> PureDeref for $ty {
            type Target = <$ty as Deref>::Target;

            fn pure_deref(&self) -> &Self::Target {
                self.deref()
            }
        }
    };

    (impl PureDeref for $ty:ty) => {
        unsafe impl PureDeref for $ty {
            type Target = <$ty as Deref>::Target;

            fn pure_deref(&self) -> &Self::Target {
                self.deref()
            }
        }
    };

    (impl<$($generics: tt),*> PureDerefMut for $ty:ty) => {
        unsafe impl<$($generics),*> PureDerefMut for $ty {
            fn pure_deref_mut(&mut self) -> &mut Self::Target {
                self.deref_mut()
            }
        }
    };

    (impl PureDerefMut for $ty:ty) => {
        unsafe impl<> PureDerefMut for $ty {
            fn pure_deref_mut(&mut self) -> &mut Self::Target {
                self.deref_mut()
            }
        }
    };
}

default_impl!(impl<'a, T> PureDeref for &'a T);
default_impl!(impl<'a, T> PureDeref for &'a mut T);
default_impl!(impl<'a, T> PureDerefMut for &'a mut T);
default_impl!(impl<T> PureDeref for Box<T>);
default_impl!(impl<T> PureDerefMut for Box<T>);
default_impl!(impl<T> PureDeref for Rc<T>);
default_impl!(impl<T> PureDeref for Arc<T>);