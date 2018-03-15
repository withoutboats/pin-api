use core::fmt;
use core::marker::Unsize;
use core::ops::{CoerceUnsized, Deref, DerefMut};

use marker::Unpin;
use mem::Pin;

#[fundamental]
pub struct PinBox<T: ?Sized> {
    inner: Box<T>,
}

impl<T> PinBox<T> {
    pub fn new(data: T) -> PinBox<T> {
        PinBox { inner: Box::new(data) }
    }
}

impl<T: ?Sized> PinBox<T> {
    pub fn as_pin<'a>(&'a mut self) -> Pin<'a, T> {
        unsafe { Pin::new_unchecked(&mut *self.inner) }
    }

    pub unsafe fn get_mut<'a>(this: &'a mut PinBox<T>) -> &'a mut T {
        &mut *this.inner
    }

    pub unsafe fn unpin(this: PinBox<T>) -> Box<T> {
        this.inner
    }
}

impl<T: ?Sized> From<Box<T>> for PinBox<T> {
    fn from(boxed: Box<T>) -> PinBox<T> {
        PinBox { inner: boxed }
    }
}

#[allow(incoherent_fundamental_impls)]
impl<T: Unpin + ?Sized> Into<Box<T>> for PinBox<T> {
    fn into(self) -> Box<T> {
        self.inner
    }
}

impl<T: ?Sized> Deref for PinBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<T: Unpin + ?Sized> DerefMut for PinBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.inner
    }
}

impl<T: fmt::Display + ?Sized> fmt::Display for PinBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self.inner, f)
    }
}

impl<T: fmt::Debug + ?Sized> fmt::Debug for PinBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self.inner, f)
    }
}

impl<T: ?Sized> fmt::Pointer for PinBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr: *const T = &*self.inner;
        fmt::Pointer::fmt(&ptr, f)
    }
}

impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<PinBox<U>> for PinBox<T> {}
