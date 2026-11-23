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

macro_rules! ref_impl {
    ($t:ty) => {
        impl<T> IsDefault for $t
        where
            T: IsDefault + ?Sized,
        {
            /// Returns `true` if `T` is default.
            #[inline]
            fn is_default(&self) -> bool {
                (**self).is_default()
            }
        }
    };
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

mod core_types;

#[cfg(feature = "alloc")]
mod alloc_types;
#[cfg(feature = "std")]
mod std_types;
