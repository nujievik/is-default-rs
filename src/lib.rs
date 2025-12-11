#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "ascii_char", feature(ascii_char, ascii_char_variants))]
#![cfg_attr(feature = "bstr", feature(bstr))]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

#[cfg(feature = "derive")]
extern crate is_default_derive;
#[cfg(feature = "derive")]
pub use is_default_derive::IsDefault;

/// Checks whether a value is equal to its type's default.
pub trait IsDefault {
    /// Returns `true` if `self` is equal to the default value for its type.
    ///
    /// Implementations must ensure that the condition `self == &Self::default()` holds.
    fn is_default(&self) -> bool;
}

#[cfg(not(feature = "via_default_eq"))]
mod not_via_default_eq;

#[cfg(feature = "via_default_eq")]
mod via_default_eq {
    use crate::IsDefault;

    impl<T> IsDefault for T
    where
        T: Default + PartialEq,
    {
        fn is_default(&self) -> bool {
            self == &Self::default()
        }
    }
}
