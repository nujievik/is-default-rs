macro_rules! unit_impl {
    ($t:ty) => {
        impl crate::IsDefault for $t {
            /// Always returns `true`.
            #[inline(always)]
            fn is_default(&self) -> bool {
                true
            }
        }
    };
}

macro_rules! matches_impl {
    ($t:ty, $v:expr) => {
        impl crate::IsDefault for $t {
            #[doc = concat!("Returns `true` if self is `", stringify!($v), "`.")]
            #[inline]
            fn is_default(&self) -> bool {
                matches!(self, $v)
            }
        }
    };
}

macro_rules! is_empty_impl {
    ($t:ty) => {
        impl IsDefault for $t {
            /// Returns `true` if self is empty.
            #[inline]
            fn is_default(&self) -> bool {
                self.is_empty()
            }
        }
    };
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

mod core_types;

#[cfg(feature = "alloc")]
mod alloc_types;
#[cfg(feature = "std")]
mod std_types;
