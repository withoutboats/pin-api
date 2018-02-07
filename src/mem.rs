use std::ffi::{CString, OsString};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;

/// A pointer type which guarantees a stable address.
///
/// This type must guarantee:
/// - It owns the data which is the target of its Deref impl.
/// - The memory address of that data will never be moved between immutable
///   dereferences.
pub unsafe trait StableDeref: Deref { }

unsafe impl<T> StableDeref for Box<T> { }
unsafe impl<T> StableDeref for Vec<T> { }

unsafe impl<T> StableDeref for Rc<T> { }
unsafe impl<T> StableDeref for Arc<T> { }
unsafe impl StableDeref for String { }
unsafe impl StableDeref for PathBuf { }
unsafe impl StableDeref for OsString { }
unsafe impl StableDeref for CString { }

/// A pointer type which guarantees a stable address under mutable conditions.
///
/// This type must guarantee:
/// - It owns the data which is the target of its Deref impl.
/// - The memory address of that data will never be moved between immutable
///   dereferences.
pub unsafe trait StableDerefMut: DerefMut { }

unsafe impl<T> StableDerefMut for Box<T> { }
unsafe impl<T> StableDerefMut for Vec<T> { }

pub struct Pin<'a, T: ?Sized> {
    _marker: PhantomData<&'a mut &'a ()>,
    data: T,
}

/// Pin an object in place.
///
/// This is intended to be combined with an Anchor to keep data from moving out
/// of its location on the stack.
pub fn pin<'a, T>(data: T) -> Pin<'a, T> {
    Pin {
        _marker: PhantomData,
        data
    }
}

/// Anchor a pointer in place.
/// 
/// This type makes it unsafe to move out of or mutably access the pointer
/// that it is wrapped around. This way, the value the pointer refers to is
/// guaranteed never to move again.
///
/// This can be applied to any pointer type that implements `StableDeref`, but
/// it can also be applied to a reference using pinning.
pub struct Anchor<Ptr: ?Sized> {
    ptr: Ptr,
}

impl<Ptr: StableDeref> Anchor<Ptr> {
    /// Construct a new Anchor from any StableDeref pointer.
    pub fn new(ptr: Ptr) -> Anchor<Ptr> {
        Anchor { ptr }
    }

    /// Convert this anchor to an anchored reference.
    pub fn as_ref<'a>(&'a self) -> Anchor<&'a Ptr::Target> {
        Anchor {
            ptr: &*self.ptr
        }
    }
}

impl<Ptr: StableDerefMut> Anchor<Ptr> {
    /// Convert this anchor to an anchored mutable reference.
    pub fn as_mut<'a>(&'a mut self) -> Anchor<&'a mut Ptr::Target> {
        Anchor {
            ptr: &mut *self.ptr
        }
    }
}

impl<'a, T> Anchor<&'a mut T> {
    /// Construct an anchor from pinned data.
    ///
    /// ```
    /// Anchor::pinned(&mut pin(0));
    /// ```
    pub fn pinned(data: &'a mut Pin<'a, T>) -> Anchor<&'a mut T> {
        Anchor { 
            ptr: &mut data.data
        }
    }
}

impl<'a, T> Anchor<&'a T> {
    /// Construct an anchor from pinned data.
    ///
    /// ```
    /// Anchor::pinned(&pin(0));
    /// ```
    pub fn pinned(data: &'a Pin<'a, T>) -> Anchor<&'a T> {
        Anchor { 
            ptr: &data.data
        }
    }
}

impl<Ptr> Anchor<Ptr> {
    /// Access a mutable reference to the underlying pointer. This method is
    /// unsafe.
    ///
    /// You must guarantee that you never move out of the reference you get
    /// when you call this.
    pub unsafe fn get_mut(&mut self) -> &mut Ptr {
        &mut self.ptr
    }
}

impl<Ptr> Deref for Anchor<Ptr> {
    type Target = Ptr;
    fn deref(&self) -> &Ptr {
        &self.ptr
    }
}
