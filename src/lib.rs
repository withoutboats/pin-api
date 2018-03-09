//! Experiment with pinning self-referential structs.
#![cfg_attr(feature = "nightly", feature(fundamental, optin_builtin_traits, coerce_unsized, unsize))]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

macro_rules! nightly { ($($i:item)*) => { $(#[cfg(feature = "nightly")]$i)* } }

nightly! {
    #[cfg(feature = "std")]
    extern crate core;

    mod stack;
    mod pin;
    mod pin_mut;
    #[cfg(feature = "std")]
    mod pin_box;

    pub use stack::{pinned, StackPinned};
    pub use pin::Pin;
    pub use pin_mut::PinMut;
    #[cfg(feature = "std")]
    pub use pin_box::PinBox;

    /// The `Unpin` auto trait means that it is safe to move out of a `Pin`
    /// reference to this type.
    ///
    /// It is not implemented by self-referential types.
    pub unsafe auto trait Unpin { }
}
