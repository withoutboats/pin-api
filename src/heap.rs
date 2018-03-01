use core::ops::{Deref, DerefMut};

use MovePinned;
use Pin;

#[fundamental]
/// A PinBox is a box that pins the data inside it. It guarantees that that
/// data will not be moved out of it unless that data implements the
/// `MovePinned` trait.
pub struct PinBox<T: ?Sized> {
    inner: Box<T>
}

impl<T> PinBox<T> {
    /// Pin a pointer to the heap.
    pub fn new(data: T) -> PinBox<T> {
        PinBox { inner: Box::new(data) }
    }

    /// Move the inner type out of the PinBox.
    ///
    /// This is unsafe because the type may be a type which is not safe to
    /// move.
    pub unsafe fn into_inner_unchecked(self) -> T {
        *self.inner
    }
}


impl<T: ?Sized> PinBox<T> {
    /// Get a pinned reference to the data in this PinBox.
    pub fn as_pin<'a>(&'a mut self) -> Pin<'a, T> {
        Pin { inner: &mut *self.inner }
    }

    /// Move the inner box out of this PinBox.
    ///
    /// This is unsafe because it is possible to move the interior type out of
    /// this box, and the interior type may be a type that is not safe to move.
    pub unsafe fn into_box_unchecked(self) -> Box<T> {
        self.inner
    }
}

impl<T: MovePinned> PinBox<T> {
    /// Move the data from this PinBox onto the stack.
    pub fn into_inner(self) -> T {
        *self.inner
    }
}

impl<T: MovePinned + ?Sized> PinBox<T> {
    /// Consume this PinBox and get the internal Box out of it.
    pub fn into_box(self) -> Box<T> {
        self.inner
    }
}

impl<T: ?Sized> Deref for PinBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<T: MovePinned + ?Sized> DerefMut for PinBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

unsafe impl<T> MovePinned for PinBox<T> where { }
