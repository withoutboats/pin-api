//! Experiment with pinning and anchoring self-referential structs.
#![feature(fundamental, optin_builtin_traits)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

macro_rules! with_std { ($($i:item)*) => ($(#[cfg(feature = "std")]$i)*) }

use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

with_std! {
    extern crate core;

    use std::ffi::{CString, OsString};
    use std::path::PathBuf;
    use std::rc::Rc;
    use std::sync::Arc;
}


/// The `MovePinned` auto trait means that it is safe to move out of a `PinMut`
/// reference to this type.
///
/// It is not implemented by self-referential types.
pub unsafe auto trait MovePinned { }

/// The `Own` trait means that this pointer has sole ownership over the data
/// which it points to.
pub unsafe trait Own: Deref { }

/// The `StableDeref` trait means that if all you do is move this pointer
/// around and dereference it immutably, the target data will never change
/// address.
pub unsafe trait StableDeref: Deref { }

/// The `StableDerefMut` trait is like `StableDeref`, but also allows that if
/// you dereference it mutably, the target data will still never change
/// address.
pub unsafe trait StableDerefMut: StableDeref + DerefMut { }

macro_rules! impls {
    ($ty:ident : $($trt:ident),+) => { $( unsafe impl $trt for $ty { } )+ };
    ($ty:ident<$p:ident> : $($trt:ident),+) => { $( unsafe impl<$p> $trt for $ty<$p> { } )+ };
    ($ty:ident<$p:ident: ?Sized> : $($trt:ident),+) => { $( unsafe impl<$p: ?Sized> $trt for $ty<$p> { } )+ };
    (&'a $p:ident: ?Sized : $($trt:ident),+) => { $( unsafe impl<'a, $p: ?Sized> $trt for &'a $p { } )+ };
    (&'a mut $p:ident: ?Sized : $($trt:ident),+) => { $( unsafe impl<'a, $p: ?Sized> $trt for &'a mut $p { } )+ };
}

impls!(&'a T: ?Sized        :       StableDeref,                    MovePinned);
impls!(&'a mut T: ?Sized    :       StableDeref,    StableDerefMut, MovePinned);

with_std! {
    impls!(Box<T: ?Sized>   : Own,  StableDeref,    StableDerefMut, MovePinned);
    impls!(Vec<T>           : Own,  StableDeref,    StableDerefMut, MovePinned);
    impls!(String           : Own,  StableDeref);
    impls!(OsString         : Own,  StableDeref);
    impls!(CString          : Own,  StableDeref);
    impls!(PathBuf          : Own,  StableDeref);
    impls!(Rc<T: ?Sized>    :       StableDeref,                    MovePinned);
    impls!(Arc<T: ?Sized>   :       StableDeref,                    MovePinned);
}

#[fundamental]
/// An anchor is a type that wraps a heap-allocated smart pointer. It
/// guarantees that data will not be moved out of it unless that data
/// implements the `MovePinned` trait.
pub struct Anchor<T: Own + StableDeref> {
    ptr: T,
}

with_std! {
    /// An Anchored Box type.
    pub type AnchoredBox<T> = Anchor<Box<T>>;

    impl<T> Anchor<Box<T>> {
        /// Construct an AnchoredBox.
        pub fn boxed(data: T) -> Anchor<Box<T>> {
            Anchor::new(Box::new(data))
        }
    }
}

impl<T: Own + StableDeref> Anchor<T> {
    /// Anchor a pointer.
    pub fn new(ptr: T) -> Anchor<T> {
        Anchor { ptr }
    }

    /// Get a pinned reference to the data in this anchor.
    pub fn as_pin<'a>(&'a self) -> Pin<'a, T::Target> {
        Pin::anchored(self)
    }
}

impl<T: Own + StableDerefMut> Anchor<T> {
    /// Get a pinned mutable reference to the data in this anchor.
    pub fn as_pin_mut<'a>(&'a mut self) -> PinMut<'a, T::Target> {
        PinMut::anchored(self)
    }
}

impl<T: Own + StableDeref> Deref for Anchor<T> {
    type Target = T::Target;
    fn deref(&self) -> &T::Target {
        &*self.ptr
    }
}

impl<T: Own + StableDerefMut> DerefMut for Anchor<T>
    where T::Target: MovePinned
{
    fn deref_mut(&mut self) -> &mut T::Target {
        &mut *self.ptr
    }
}

unsafe impl<T> MovePinned for Anchor<T> where
    T: StableDeref + Own + MovePinned
{ }

#[fundamental]
/// A pinned reference.
///
/// The value referenced by this is guaranteed never to move again, unless it
/// implements `MovePinned`.
pub struct Pin<'a, T: ?Sized + 'a> {
    inner: &'a T,
}

impl<'a, T: ?Sized> Pin<'a, T> {
    /// Construct a `Pin` from a stack pinned value, constructed
    /// using the `pinned` function.
    pub fn new(ptr: &'a StackPinned<'a, T>) -> Pin<'a, T> {
        Pin { inner: &ptr.data }
    }

    /// Construct a `Pin` from an anchored pointer into the heap.
    ///
    /// This is equivalent to calling `as_pin` on the `Anchor`.
    pub fn anchored<P>(anchor: &'a Anchor<P>) -> Pin<'a, T>
        where P: StableDeref<Target = T> + Own,
    {
        Pin { inner: &*anchor.ptr }
    }

    /// Construct a new `Pin` without checking that the data is actually
    /// pinned.
    ///
    /// You must guarantee that the data meets the requirements for
    /// constructing a `Pin`.
    ///
    /// An example use case is constructing a `Pin` of a field of this type:
    ///
    /// ```ignore
    /// let inner = unsafe { Pin::pinned_unchecked(&this.inner) };
    /// ````
    pub unsafe fn pinned_unchecked(ptr: &'a T) -> Pin<'a, T> {
        Pin { inner: ptr }
    }

    /// Get a Pin with a shorter lifetime
    pub fn as_ref<'b>(&'b mut self) -> Pin<'b, T> {
        Pin { inner: self.inner }
    }
}

impl<'a, T> Deref for Pin<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}

#[fundamental]
/// A pinned mutable reference.
///
/// The value referenced by this is guaranteed never to move again, unless it
/// implements `MovePinned`.
pub struct PinMut<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
}

impl<'a, T: ?Sized> PinMut<'a, T> {
    /// Construct a `PinMut` from a stack pinned value, constructed
    /// using the `pinned` function.
    pub fn new(ptr: &'a mut StackPinned<'a, T>) -> PinMut<'a, T> {
        PinMut { inner: &mut ptr.data }
    }

    /// Construct a `PinMut` from an anchored pointer into the heap.
    ///
    /// This is equivalent to calling `as_pin_mut` on the `Anchor`.
    pub fn anchored<P>(anchor: &'a mut Anchor<P>) -> PinMut<'a, T>
        where P: StableDerefMut<Target = T> + Own,
    {
        PinMut { inner: &mut *anchor.ptr }
    }

    /// Construct a new `PinMut` without checking that the data is actually
    /// pinned.
    ///
    /// You must guarantee that the data meets the requirements for
    /// constructing a `PinMut`.
    ///
    /// An example use case is constructing a `PinMut` of a field of this
    /// type:
    ///
    /// ```ignore
    /// let inner = unsafe {
    ///     let this = PinMut::get_mut(this);
    ///     PinMut::pinned_unchecked(&mut this.inner)
    /// };
    /// ````
    pub unsafe fn pinned_unchecked(ptr: &'a mut T) -> PinMut<'a, T> {
        PinMut { inner: ptr }
    }

    /// Get a mutable reference to the inner value, even when it doesn't
    /// implement `MovePinned`.
    ///
    /// For types that implement `MovePinned`, `PinMut` implements `DerefMut`.
    /// This is only necessary when attempting to gain mutable access to
    /// something which does not implement `MovePinned`.
    ///
    /// When calling this function, you must guarantee that the type is not
    /// moved out of the mutable reference you receive.
    pub unsafe fn get_mut<'b>(pin: &'b mut PinMut<'a, T>) -> &'b mut T {
        pin.inner
    }

    /// Get a Pin with a shorter lifetime
    pub fn as_ref<'b>(&'b mut self) -> Pin<'b, T> {
        Pin { inner: self.inner }
    }

    /// Get a PinMut with a shorter lifetime
    pub fn as_mut<'b>(&'b mut self) -> PinMut<'b, T> {
        PinMut { inner: self.inner }
    }
}

impl<'a, T: ?Sized> Deref for PinMut<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.inner
    }
}

impl<'a, T: MovePinned + ?Sized> DerefMut for PinMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner
    }
}

/// This struct is used for pinning data to the stack.
///
/// You can construct this struct using the `pinned` function.
///
/// This struct has no methods of its own and should only be used as part of
/// constructing a `Pin` or `PinMut` type.
pub struct StackPinned<'a, T: ?Sized + 'a> {
    _marker: PhantomData<&'a mut &'a ()>,
    data: T,
}

/// Pin data in the stack.
///
/// This is used as a part of constructing a `Pin` or `PinMut` without a heap
/// allocation.
///
/// ```
/// # extern crate anchor_experiment;
/// # use anchor_experiment::{Pin, pinned};
/// # fn main() {
///   let data = 0;
///   let pinned = pinned(0);
///   let data = Pin::new(&pinned);
/// # }
pub fn pinned<'a, T: 'a>(data: T) -> StackPinned<'a, T> {
    StackPinned {
        _marker: PhantomData,
        data,
    }
}
