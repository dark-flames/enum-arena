use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

/// Types can be dereferenced purely.
///
/// This trait has been implemented for primitive referneces, `std::boxed::Box`, `std::rc::Rc`, and `std::sync::Arc`.
///
/// # Safety
/// The implementation is safe if the `pure_deref` has no side effect
pub unsafe trait PureDeref {
    type Target: ?Sized;

    fn pure_deref(&self) -> &Self::Target;
}

/// Types can be mutablely dereferenced purely.
///
/// This trait has been implemented for primitive refernece and `std::boxed::Box`.
///
/// # Safety
/// The implementation is safe if the `pure_deref_mut` has no side effect
pub unsafe trait PureDerefMut: PureDeref {
    fn pure_deref_mut(&mut self) -> &mut Self::Target;
}

#[macro_export]
macro_rules! default_impl_pure_deref {
    (unsafe impl<$($generics: tt),*> PureDeref for $ty:ty) => {
        unsafe impl<$($generics),*> PureDeref for $ty {
            type Target = <$ty as Deref>::Target;

            fn pure_deref(&self) -> &Self::Target {
                self.deref()
            }
        }
    };

    (unsafe impl PureDeref for $ty:ty) => {
        unsafe impl PureDeref for $ty {
            type Target = <$ty as Deref>::Target;

            fn pure_deref(&self) -> &Self::Target {
                self.deref()
            }
        }
    };

    (unsafe impl<$($generics: tt),*> PureDerefMut for $ty:ty) => {
        unsafe impl<$($generics),*> PureDerefMut for $ty {
            fn pure_deref_mut(&mut self) -> &mut Self::Target {
                self.deref_mut()
            }
        }
    };

    (unsafe impl PureDerefMut for $ty:ty) => {
        unsafe impl PureDerefMut for $ty {
            fn pure_deref_mut(&mut self) -> &mut Self::Target {
                self.deref_mut()
            }
        }
    };
}

default_impl_pure_deref!(unsafe impl<'a, T> PureDeref for &'a T);
default_impl_pure_deref!(unsafe impl<'a, T> PureDeref for &'a mut T);
default_impl_pure_deref!(unsafe impl<'a, T> PureDerefMut for &'a mut T);
default_impl_pure_deref!(unsafe impl<T> PureDeref for Box<T>);
default_impl_pure_deref!(unsafe impl<T> PureDerefMut for Box<T>);
default_impl_pure_deref!(unsafe impl<T> PureDeref for Rc<T>);
default_impl_pure_deref!(unsafe impl<T> PureDeref for Arc<T>);
