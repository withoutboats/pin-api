//! Experiment with pinning self-referential structs.
#![feature(fundamental, optin_builtin_traits)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

mod stack;
#[cfg(feature = "std")]
mod heap;

use core::ops::{Deref, DerefMut};

pub use stack::{pinned, StackPinned};
#[cfg(feature = "std")]
pub use heap::PinBox;

/// The `MovePinned` auto trait means that it is safe to move out of a `PinMut`
/// reference to this type.
///
/// It is not implemented by self-referential types.
pub unsafe auto trait MovePinned { }

#[fundamental]
/// A pinned reference.
///
/// The value referenced by this is guaranteed never to move again, unless it
/// implements `MovePinned`.
pub struct Pin<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
}

impl<'a, T: MovePinned + ?Sized> Pin<'a, T> {
    /// Create a new Pin from a mutable reference to a moveable type.
    pub fn new(ptr: &'a mut T) -> Pin<'a, T> {
        Pin { inner: ptr }
    }
}

impl<'a, T: ?Sized> Pin<'a, T> {
    /// Construct a new `Pin` without checking that the data is actually
    /// pinned.
    ///
    /// You must guarantee that the data meets the requirements for
    /// constructing a `Pin`.
    ///
    /// An example use case is constructing a `Pin` of a field of this type:
    ///
    /// ```ignore
    /// let inner = unsafe { Pin::new_unchecked(&mut this.inner) };
    /// ````
    pub unsafe fn new_unchecked(ptr: &'a mut T) -> Pin<'a, T> {
        Pin { inner: ptr }
    }

    /// Get a Pin with a shorter lifetime
    pub fn borrow<'b>(this: &'b mut Pin<'a, T>) -> Pin<'b, T> {
        Pin { inner: this.inner }
    }

    /// Get a mutable reference to the data inside this type.
    ///
    /// This is unsafe, because you must guarantee that you do not move the
    /// data out of the mutable reference that this function returns.
    pub unsafe fn get_mut<'b>(this: &'b mut Pin<'a, T>) -> &'b mut T {
        this.inner
    }
}

impl<'a, T: ?Sized> Deref for Pin<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}

impl<'a, T: MovePinned + ?Sized> DerefMut for Pin<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner
    }
}
