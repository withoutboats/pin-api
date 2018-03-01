use core::marker::PhantomData;

use Pin;

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

impl<'a, T: ?Sized + 'a> StackPinned<'a, T> {
    /// Convert this type to a Pin.
    pub fn as_pin(&'a mut self) -> Pin<'a, T> {
        Pin { inner: &mut self.data }
    }
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
///   let mut pinned = pinned(0);
///   let data = pinned.as_pin();
/// # }
pub fn pinned<'a, T: 'a>(data: T) -> StackPinned<'a, T> {
    StackPinned {
        _marker: PhantomData,
        data,
    }
}
