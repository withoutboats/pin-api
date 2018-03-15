use core::fmt;
use core::marker::Unsize;
use core::ops::{CoerceUnsized, Deref, DerefMut};

use marker::Unpin;

#[fundamental]
pub struct Pin<'a, T: ?Sized + 'a> {
    inner: &'a mut T,
}

impl<'a, T: ?Sized + Unpin> Pin<'a, T> {
    pub fn new(reference: &'a mut T) -> Pin<'a, T> {
        Pin { inner: reference }
    }
}

impl<'a, T: ?Sized> Pin<'a, T> {
    pub unsafe fn new_unchecked(reference: &'a mut T) -> Pin<'a, T> {
        Pin { inner: reference }
    }

    pub fn borrow<'b>(this: &'b mut Pin<'a, T>) -> Pin<'b, T> {
        Pin { inner: this.inner }
    }

    pub unsafe fn get_mut<'b>(this: &'b mut Pin<'a, T>) -> &'b mut T {
        this.inner
    }

    pub unsafe fn map<'b, U, F>(this: &'b mut Pin<'a, T>, f: F) -> Pin<'b, U> where
        F: FnOnce(&mut T) -> &mut U
    {
        Pin { inner: f(this.inner) }
    }
}

impl<'a, T: ?Sized> Deref for Pin<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.inner
    }
}

impl<'a, T: ?Sized + Unpin> DerefMut for Pin<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner
    }
}

impl<'a, T: fmt::Debug + ?Sized> fmt::Debug for Pin<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: fmt::Display + ?Sized> fmt::Display for Pin<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized> fmt::Pointer for Pin<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&(&*self.inner as *const T), f)
    }
}

impl<'a, T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<Pin<'a, U>> for Pin<'a, T> {}
