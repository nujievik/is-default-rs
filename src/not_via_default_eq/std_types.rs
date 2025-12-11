use crate::IsDefault;
use std::{
    borrow::Cow,
    cell::{Cell, OnceCell, Ref, RefCell, RefMut},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    ffi::{CStr, CString, OsStr, OsString},
    io::Cursor,
    marker::PhantomPinned,
    num::Wrapping,
    path::{Path, PathBuf},
    rc::{self, Rc},
    sync::{
        self, Arc, Mutex, OnceLock, RwLock,
        atomic::{
            AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8,
            AtomicU16, AtomicU32, AtomicU64, AtomicUsize, Ordering,
        },
    },
    time::Duration,
};

impl<T> IsDefault for Wrapping<T>
where
    T: IsDefault,
{
    /// Returns `true` if the inner value is default.
    #[inline]
    fn is_default(&self) -> bool {
        self.0.is_default()
    }
}

unit_impl!(PhantomPinned);

macro_rules! atomic_impl {
    ($t:ty, $v:expr) => {
        impl IsDefault for $t {
            #[doc = concat!("Returns `true` if self is `", stringify!($v), "`.")]
            #[inline]
            fn is_default(&self) -> bool {
                matches!(self.load(Ordering::Relaxed), $v)
            }
        }
    };
}

atomic_impl!(AtomicBool, false);

atomic_impl!(AtomicI8, 0i8);
atomic_impl!(AtomicI16, 0i16);
atomic_impl!(AtomicI32, 0i32);
atomic_impl!(AtomicI64, 0i64);
atomic_impl!(AtomicIsize, 0isize);

atomic_impl!(AtomicU8, 0u8);
atomic_impl!(AtomicU16, 0u16);
atomic_impl!(AtomicU32, 0u32);
atomic_impl!(AtomicU64, 0u64);
atomic_impl!(AtomicUsize, 0usize);

is_empty_impl!(String);
is_empty_impl!(CStr);
is_empty_impl!(CString);
is_empty_impl!(OsStr);
is_empty_impl!(OsString);

macro_rules! path_impl {
    ($t:ty) => {
        impl IsDefault for $t {
            /// Returns `true` if self is empty.
            #[inline]
            fn is_default(&self) -> bool {
                self.as_os_str().is_empty()
            }
        }
    };
}

path_impl!(Path);
path_impl!(PathBuf);

impl IsDefault for Duration {
    /// Returns `true` if self is zero.
    #[inline]
    fn is_default(&self) -> bool {
        self.is_zero()
    }
}

impl<T> IsDefault for Option<T> {
    /// Returns `true` if self is `None`.
    #[inline]
    fn is_default(&self) -> bool {
        self.is_none()
    }
}

macro_rules! is_empty_impl_t {
    ($t:ident) => {
        impl<T> IsDefault for $t<T> {
            /// Returns `true` if self is empty.
            #[inline]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

is_empty_impl_t!(BTreeSet);
is_empty_impl_t!(BinaryHeap);
is_empty_impl_t!(HashSet);
is_empty_impl_t!(LinkedList);
is_empty_impl_t!(Vec);
is_empty_impl_t!(VecDeque);

macro_rules! is_empty_impl_k_v {
    ($t:ident) => {
        impl<K, V> IsDefault for $t<K, V> {
            /// Returns `true` if self is empty.
            #[inline]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
}

is_empty_impl_k_v!(BTreeMap);
is_empty_impl_k_v!(HashMap);

macro_rules! deref_impl_t {
    ($t:ident) => {
        impl<T> IsDefault for $t<T>
        where
            T: IsDefault + ?Sized,
        {
            /// Returns `true` if the inner value is default.
            #[inline]
            fn is_default(&self) -> bool {
                (**self).is_default()
            }
        }
    };
}

deref_impl_t!(Arc);
deref_impl_t!(Box);
deref_impl_t!(Rc);

impl<T> IsDefault for Cow<'_, T>
where
    T: IsDefault + ToOwned + ?Sized,
{
    /// Returns `true` if the inner value is default.
    #[inline]
    fn is_default(&self) -> bool {
        (**self).is_default()
    }
}

macro_rules! lock_impl {
    ($t:ident, $lock:ident) => {
        impl<T> IsDefault for $t<T>
        where
            T: IsDefault + ?Sized,
        {
            /// Returns `true` if the inner value is default.
            #[doc = concat!("Always return false if `self.", stringify!($lock), "()` returns an error.")]
            #[inline]
            fn is_default(&self) -> bool {
                self.$lock().map_or(false, |v| v.is_default())
            }
        }
    };
}

lock_impl!(RefCell, try_borrow);
lock_impl!(RwLock, try_read);
lock_impl!(Mutex, try_lock);

impl<T> IsDefault for Cell<T>
where
    T: Copy + IsDefault,
{
    /// Returns `true` if the inner value is default.
    #[inline]
    fn is_default(&self) -> bool {
        self.get().is_default()
    }
}

macro_rules! once_impl {
    ($t:ident) => {
        impl<T> IsDefault for $t<T> {
            /// Returns `true` if self is uninitialized.
            #[inline]
            fn is_default(&self) -> bool {
                matches!(self.get(), None)
            }
        }
    };
}

once_impl!(OnceCell);
once_impl!(OnceLock);

impl<T: ?Sized> IsDefault for rc::Weak<T> {
    /// Returns true if the [Weak::upgrade](rc::Weak::upgrade) returns `None`.
    #[inline]
    fn is_default(&self) -> bool {
        matches!(self.upgrade(), None)
    }
}

impl<T: ?Sized> IsDefault for sync::Weak<T> {
    /// Returns true if the [Weak::upgrade](sync::Weak::upgrade) returns `None`.
    #[inline]
    fn is_default(&self) -> bool {
        matches!(self.upgrade(), None)
    }
}

impl<T> IsDefault for Cursor<T> {
    /// Returns `true` if cursor position is `0`.
    #[inline]
    fn is_default(&self) -> bool {
        matches!(self.position(), 0u64)
    }
}

ref_impl!(Ref<'_, T>);
ref_impl!(RefMut<'_, T>);
