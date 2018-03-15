//! Experiment with pinning self-referential structs.
#![cfg_attr(feature = "nightly", feature(fundamental, optin_builtin_traits, coerce_unsized, unsize))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate core;

#[cfg(feature = "nightly")]
pub mod marker;
#[cfg(feature = "nightly")]
pub mod mem;
#[cfg(all(feature = "nightly", feature = "std"))]
pub mod boxed;
