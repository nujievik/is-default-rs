//! # IsDefault
//!
//! A trait for checking if a value is default, with easy derive support for custom types.
//!
//! Example, instead of `is_none` for [`Option`] and `is_empty` for [`Vec`]
//! can be used `is_default` for all.
//!
//! ```rust
//! assert!(None::<u8>.is_none());
//! assert!(Vec::<u8>::new().is_empty());
//!
//! use is_default::IsDefault;
//! assert!(None::<u8>.is_default());
//! assert!(Vec::<u8>::new().is_default());
//! ```
//!
//! The `IsDefault` trait is implemented for most standard types.
//! With the `derive` feature, you can easily generate implementations for your own types:
//!
//! ## Derive
//!
//! To use the derive macro, add the dependency with the `derive` feature
//! in your `Cargo.toml`:
//!
//! ```toml
//! # Cargo.toml
//!
//! [dependencies]
//! is_default = { version = "1", features = ["derive"] }
//! ```
//!
//! ### Structs
//!
//! A struct can derive `IsDefault` if all its fields implement `IsDefault`.
//!
//! ```rust
//! # #[cfg(feature = "derive")] {
//! use is_default::IsDefault;
//!
//! #[derive(IsDefault)]
//! struct Unit;
//! assert!(Unit.is_default());
//!
//! #[derive(IsDefault)]
//! struct Wrapper(u8);
//! assert!(Wrapper(0).is_default());
//! assert!(!Wrapper(1).is_default());
//!
//! #[derive(IsDefault)]
//! struct Point { x: i16, y: f32 }
//! assert!(Point{ x: 0, y: 0.0 }.is_default());
//! assert!(!Point{ x: 1, y: 0.0 }.is_default());
//! assert!(!Point{ x: 0, y: 1.1 }.is_default());
//! # }
//! ```
//!
//! ### Enums
//!
//! An enum can derive `IsDefault` using either the `#[is_default]` OR the
//! `#[default]` attribute. This makes it possible to derive both `Default`
//! and `IsDefault` using the same attribute.
//!
//! ```rust
//! # #[cfg(feature = "derive")] {
//! use is_default::IsDefault;
//!
//! #[derive(IsDefault)]
//! enum A {
//!     #[is_default]
//!     X,
//!     Y,
//! }
//! assert!(A::X.is_default());
//! assert!(!A::Y.is_default());
//!
//! #[derive(Default, IsDefault)]
//! enum B {
//!     X,
//!     #[default]
//!     Y,
//! }
//! assert!(!B::X.is_default());
//! assert!(B::Y.is_default());
//! assert!(matches!(B::default(), B::Y));
//! # }
//! ```
//!
//! An enum can also derive `IsDefault` if it implements both `Default` and
//! `PartialEq`. However, this implementation may be inefficient, since a
//! `Self` object must be allocated for comparison.
//!
//! ```rust
//! # #[cfg(feature = "derive")] {
//! use is_default::IsDefault;
//!
//! #[derive(PartialEq, IsDefault)]
//! enum C {
//!     X(u8),
//!     Y,
//! }
//! impl Default for C {
//!     fn default() -> C {
//!         C::X(0)
//!     }
//! }
//!
//! assert!(C::X(0).is_default());
//! assert!(!C::X(1).is_default());
//! # }
//! ```

#![cfg_attr(
    feature = "nightly",
    feature(ascii_char, ascii_char_variants, f16, f128)
)]

#[cfg(feature = "derive")]
extern crate is_default_derive;
#[cfg(feature = "derive")]
pub use is_default_derive::IsDefault;

/// Checks whether a value is equal to its type's default.
pub trait IsDefault {
    /// Returns `true` if `self` is equal to the default value for its type.
    fn is_default(&self) -> bool;
}

#[cfg(feature = "nightly")]
use std::ascii::Char;

use std::{
    borrow::Cow,
    cell::{Cell, OnceCell, RefCell},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ffi::{CStr, CString, OsStr, OsString},
    io::Cursor,
    num::Wrapping,
    path::{Path, PathBuf},
    rc::Rc,
    ops::Deref,
    sync::{
        Arc, Mutex, OnceLock, RwLock, Weak,
        atomic::{
            AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8,
            AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
        },
    },
    time::Duration,
};

macro_rules! unit_impl {
    ($t:ty) => {
        impl IsDefault for $t {
            /// Always returns `true`.
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("assert!(", stringify!($t), ".is_default());")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                true
            }
        }
    };
}

unit_impl!(());
unit_impl!(std::marker::PhantomPinned);

macro_rules! matches_impl {
    ($t:ty, $v:expr; $( $not:expr ),* ) => {
        impl IsDefault for $t {
            #[doc = concat!("Returns `true` if self is `", stringify!($v), "`.")]
            /// ```
            /// use is_default::IsDefault;
            ///
            #[doc = concat!("assert!(", stringify!($v), ".is_default());")]
            #[doc = concat!("assert!(", stringify!($t), "::default().is_default());")]
            $( #[doc = concat!("assert!(!", stringify!($not), ".is_default());")] )*
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                matches!(self, $v)
            }
        }
    };
}

matches_impl!(bool, false; true);
matches_impl!(char, '\x00'; 'a', char::MAX);

matches_impl!(usize, 0usize; 1usize, usize::MAX);
matches_impl!(u8, 0u8; 1u8, u8::MAX);
matches_impl!(u16, 0u16; 1u16, u16::MAX);
matches_impl!(u32, 0u32; 1u32, u32::MAX);
matches_impl!(u64, 0u64; 1u64, u64::MAX);
matches_impl!(u128, 0u128; 1u128, u128::MAX);

matches_impl!(isize, 0isize; isize::MIN, 1isize, isize::MAX);
matches_impl!(i8, 0i8; i8::MIN, 1i8, i8::MAX);
matches_impl!(i16, 0i16; i16::MIN, 1i16, i16::MAX);
matches_impl!(i32, 0i32; i32::MIN, 1i32, i32::MAX);
matches_impl!(i64, 0i64; i64::MIN, 1i64, i64::MAX);
matches_impl!(i128, 0i128; i128::MIN, 1i128, i128::MAX);

matches_impl!(f32, 0f32; f32::MIN, 1f32, f32::MAX);
matches_impl!(f64, 0f64; f64::MIN, 1f64, f64::MAX);

impl<T> IsDefault for Wrapping<T>
where
    T: IsDefault,
{
    /// Returns `true` if the inner value is default.
    /// ```
    /// use is_default::IsDefault;
    /// use std::num::Wrapping;
    ///
    /// assert!(Wrapping(0u8).is_default());
    /// assert!(!Wrapping(1u8).is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

macro_rules! nightly_matches_impl {
    ($t:ty, $v:expr $(, $nightly_fs:literal)? ; $( $not:expr ),* ) => {
        #[cfg(feature = "nightly")]
        impl IsDefault for $t {
            #[doc = concat!("Returns `true` if self is `", stringify!($v), "`.")]
            /// ```
            $( #[doc = concat!("#![cfg_attr(feature = \"nightly\", feature(", $nightly_fs, "))]")] )?
            /// use is_default::IsDefault;
            ///
            #[doc = concat!("assert!(", stringify!($v), ".is_default());")]
            #[doc = concat!("assert!(", stringify!($t), "::default().is_default());")]
            $( #[doc = concat!("assert!(!", stringify!($not), ".is_default());")] )*
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                matches!(self, $v)
            }
        }
    };
}

nightly_matches_impl!(f16, 0f16, "f16"; f16::MIN, 1f16, f16::MAX);
nightly_matches_impl!(f128, 0f128, "f128"; f128::MIN, 1f128, f128::MAX);

#[cfg(feature = "nightly")]
impl IsDefault for Char {
    /// Returns `true` if self is `Char::Null`.
    /// ```
    /// #![cfg_attr(feature = "nightly", feature(ascii_char, ascii_char_variants))]
    /// use is_default::IsDefault;
    /// use std::ascii::Char;
    ///
    /// assert!(Char::Null.is_default());
    /// assert!(Char::default().is_default());
    /// assert!(!Char::StartOfHeading.is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        matches!(self, Char::Null)
    }
}

macro_rules! atomic_impl {
    ($t:ty, $v:expr; $( $not:expr ),* ) => {
        impl IsDefault for $t {
            #[doc = concat!("Returns `true` if the inner value is `", stringify!($v), "`.")]
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("use std::sync::atomic::{", stringify!($t), ", Ordering};")]
            ///
            #[doc = concat!("assert!(", stringify!($t), "::new(", stringify!($v), ").is_default());")]
            #[doc = concat!("assert!(", stringify!($t), "::default().is_default());")]
            $( #[doc = concat!("assert!(!", stringify!($t), "::new(", stringify!($not), ").is_default());")] )*
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                matches!(self.load(Ordering::Relaxed), $v)
            }
        }
    };
}

atomic_impl!(AtomicBool, false; true);

atomic_impl!(AtomicI8, 0; i8::MIN, 1, i8::MAX);
atomic_impl!(AtomicI16, 0; i16::MIN, 1, i16::MAX);
atomic_impl!(AtomicI32, 0; i32::MIN, 1, i32::MAX);
atomic_impl!(AtomicI64, 0; i64::MIN, 1, i64::MAX);
atomic_impl!(AtomicIsize, 0; isize::MIN, 1, isize::MAX);

atomic_impl!(AtomicU8, 0; 1, u8::MAX);
atomic_impl!(AtomicU16, 0; 1, u16::MAX);
atomic_impl!(AtomicU32, 0; 1, u32::MAX);
atomic_impl!(AtomicU64, 0; 1, u64::MAX);
atomic_impl!(AtomicUsize, 0; 1, usize::MAX);

macro_rules! str_impl {
    ($t:ty, $v:expr $(, $use:literal)? ; $not:expr) => {
        impl IsDefault for $t {
            /// Returns `true` if self is empty.
            /// ```
            /// use is_default::IsDefault;
            $( #[doc = concat!("use std::ffi::", $use, ";")] )?
            ///
            #[doc = concat!("assert!(", stringify!($v), ".is_default());")]
            #[doc = concat!("assert!(!", stringify!($not), ".is_default());")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

str_impl!(str, ""; "x");
str_impl!(String, String::from(""); String::from("x"));
str_impl!(CStr, c""; c"x");
str_impl!(CString, CString::from(c""), "CString"; CString::from(c"x"));
str_impl!(OsStr, OsStr::new(""), "OsStr"; OsStr::new("x"));
str_impl!(OsString, OsString::from(""), "OsString"; OsString::from("x"));

macro_rules! path_impl {
    ($t:ty, $new:literal) => {
        impl IsDefault for $t {
            /// Returns `true` if self is empty.
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("use std::path::", stringify!($t), ";")]
            ///
            #[doc = concat!("assert!(", stringify!($t), "::", $new, "(\"\").is_default());\n")]
            #[doc = concat!("assert!(!", stringify!($t), "::", $new, "(\"x\").is_default());\n")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                self.as_os_str().is_empty()
            }
        }
    };
}

path_impl!(Path, "new");
path_impl!(PathBuf, "from");

impl IsDefault for Duration {
    /// Returns `true` if self is zero.
    /// ```
    /// use is_default::IsDefault;
    /// use std::time::Duration;
    ///
    /// assert!(Duration::ZERO.is_default());
    /// assert!(Duration::default().is_default());
    /// assert!(!Duration::new(1, 0).is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.is_zero()
    }
}

impl<T> IsDefault for Option<T> {
    /// Returns `true` if self is none.
    /// ```
    /// use is_default::IsDefault;
    ///
    /// assert!(None::<u8>.is_default());
    /// assert!(Option::<u8>::default().is_default());
    /// assert!(!Some(0u8).is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

macro_rules! collection_impl {
    ($t:ident $(, $use:literal)? ) => {
        impl<T> IsDefault for $t<T> {
            /// Returns `true` if self is empty.
            /// ```
            /// use is_default::IsDefault;
            $( #[doc = concat!("use std::collections::", $use, ";")] )?
            ///
            #[doc = concat!("assert!(", stringify!($t), "::<u8>::default().is_default());")]
            #[doc = concat!("assert!(!", stringify!($t), "::from([0u8]).is_default());")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

collection_impl!(Vec);
collection_impl!(BTreeSet, "BTreeSet");
collection_impl!(BinaryHeap, "BinaryHeap");
collection_impl!(HashSet, "HashSet");
collection_impl!(LinkedList, "LinkedList");
collection_impl!(VecDeque, "VecDeque");

macro_rules! map_impl {
    ($t:ident) => {
        impl<K, V> IsDefault for $t<K, V> {
            /// Returns `true` if self is empty.
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("use std::collections::", stringify!($t), ";")]
            ///
            #[doc = concat!("assert!(", stringify!($t), "::<u8, u8>::default().is_default());")]
            #[doc = concat!("assert!(!", stringify!($t), "::from([(0u8, 0u8)]).is_default());")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

map_impl!(BTreeMap);
map_impl!(HashMap);

macro_rules! pointer_impl {
    ($t:ident $(, $use:literal)? ) => {
        impl<T> IsDefault for $t<T>
        where
            T: IsDefault,
        {
            /// Returns `true` if the inner value is default.
            /// ```
            /// use is_default::IsDefault;
            $( #[doc = concat!("use ", $use, ";")] )?
            ///
            #[doc = concat!("assert!(", stringify!($t), "::new(0u8).is_default());")]
            #[doc = concat!("assert!(", stringify!($t), "::<u8>::default().is_default());")]
            #[doc = concat!("assert!(!", stringify!($t), "::new(1u8).is_default());")]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                (**self).is_default()
            }
        }
    };
}

pointer_impl!(Arc, "std::sync::Arc");
pointer_impl!(Box);
pointer_impl!(Rc, "std::rc::Rc");

impl<T> IsDefault for Cow<'_, T>
where
    T: IsDefault + ToOwned + ?Sized,
{
    /// Returns `true` if the inner value is default.
    /// ```
    /// use is_default::IsDefault;
    /// use std::borrow::Cow;
    ///
    /// assert!(Cow::from("").is_default());
    /// assert!(!Cow::from("x").is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.deref().is_default()
    }
}

macro_rules! lock_impl {
    ($t:ident, $lock:ident, $use:literal) => {
        impl<T> IsDefault for $t<T>
        where
            T: IsDefault,
        {
            /// Returns `true` if the inner value is default.
            #[doc = concat!("Always return false if `self.", stringify!($lock), "()` returns an error.")]
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("use ", $use, "::", stringify!($t), ";")]
            ///
            #[doc = concat!(
                "assert!(", stringify!($t), "::new(0u8).", stringify!($lock) , "().unwrap().is_default());"
            )]
            #[doc = concat!(
                "assert!(", stringify!($t), "::<u8>::default().", stringify!($lock) , "().unwrap().is_default());"
            )]
            #[doc = concat!(
                "assert!(!", stringify!($t), "::new(1u8).", stringify!($lock) , "().unwrap().is_default());"
            )]
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                self.$lock().map_or(false, |v| v.is_default())
            }
        }
    };
}

lock_impl!(RefCell, try_borrow, "std::cell");
lock_impl!(RwLock, try_read, "std::sync");
lock_impl!(Mutex, try_lock, "std::sync");

impl<T> IsDefault for Cell<T>
where
    T: Copy + IsDefault,
{
    /// Returns `true` if the inner value is default.
    /// ```
    /// use is_default::IsDefault;
    /// use std::cell::Cell;
    ///
    /// assert!(Cell::new(0u8).is_default());
    /// assert!(!Cell::new(1u8).is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.get().is_default()
    }
}

macro_rules! once_impl {
    ($t:ident, $use:literal) => {
        impl<T> IsDefault for $t<T> {
            /// Returns `true` if self is uninitialized.
            /// ```
            /// use is_default::IsDefault;
            #[doc = concat!("use ", $use, "::", stringify!($t), ";")]
            ///
            #[doc = concat!("let v = ", stringify!($t), "::<u8>::new();")]
            /// assert!(v.is_default());
            /// v.set(0).unwrap();
            /// assert!(!v.is_default());
            /// ```
            #[inline(always)]
            fn is_default(&self) -> bool {
                matches!(self.get(), None)
            }
        }
    };
}

once_impl!(OnceCell, "std::cell");
once_impl!(OnceLock, "std::sync");

impl<T> IsDefault for Weak<T> {
    /// Returns `true` if the [`Weak::upgrade`] returns None.
    /// ```
    /// use is_default::IsDefault;
    /// use std::sync::{Arc, Weak};
    ///
    /// assert!(Weak::<u8>::new().is_default());
    /// let x = Arc::new(0u8);
    /// let xw: Weak<u8> = Arc::downgrade(&x);
    /// assert!(!xw.is_default());
    /// drop(x);
    /// assert!(xw.is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        matches!(self.upgrade(), None)
    }
}

impl<T> IsDefault for Cursor<T> {
    /// Returns `true` if cursor position is `0`.
    /// ```
    /// use is_default::IsDefault;
    /// use std::io::Cursor;
    ///
    /// let mut c = Cursor::new([0u8; 1]);
    /// assert!(c.is_default());
    /// c.set_position(1);
    /// assert!(!c.is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        matches!(self.position(), 0u64)
    }
}

impl<T> IsDefault for [T]
where
    T: IsDefault
{
    /// Always returns `true` if slice is empty.
    /// Otherwise, returns `true` if all slice elements is default.
    /// ```
    /// use is_default::IsDefault;
    ///
    /// assert!(<&[i32]>::default().is_default());
    /// assert!(&[0].is_default());
    /// assert!(!&[1].is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        if self.is_empty() {
            true
        } else {
            self.iter().all(|x| x.is_default())
        }
    }
}

impl<T, const N: usize> IsDefault for [T; N]
where
    T: IsDefault,
{
    /// Always returns `true` for array [T; 0].
    /// Otherwise, returns `true` if all array elements is default.
    /// ```
    /// use is_default::IsDefault;
    ///
    /// assert!([1u8; 0].is_default());
    /// assert!([0u8].is_default());
    /// assert!(![1u8].is_default());
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        self.as_slice().is_default()
    }
}

impl<T> IsDefault for &T
where
    T: IsDefault + ?Sized,
{
    /// Delegates to the implementation of [`IsDefault`] for `T`.
    ///
    /// This allows automatically deriving [`IsDefault`] for structs
    /// that contain `&T` fields.
    ///
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use is_default::IsDefault;
    ///
    /// #[derive(IsDefault)]
    /// struct Ref<'a>(&'a u8);
    ///
    /// let x = 0u8;
    /// assert!(Ref(&x).is_default());
    /// let y = 1u8;
    /// assert!(!Ref(&y).is_default());
    /// # }
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}

impl<T> IsDefault for &mut T
where
    T: IsDefault + ?Sized,
{
    /// Delegates to the implementation of [`IsDefault`] for `T`.
    ///
    /// This allows automatically deriving [`IsDefault`] for structs
    /// that contain `&mut T` fields.
    ///
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use is_default::IsDefault;
    ///
    /// #[derive(IsDefault)]
    /// struct MutRef<'a>(&'a mut u8);
    ///
    /// let mut a = 0u8;
    /// assert!(MutRef(&mut a).is_default());
    /// let mut b = 1u8;
    /// assert!(!MutRef(&mut b).is_default());
    /// # }
    /// ```
    #[inline(always)]
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}
