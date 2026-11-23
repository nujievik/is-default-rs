extern crate alloc;

use crate::IsDefault;
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque},
    ffi::CString,
    rc::{self, Rc},
    string::String,
    sync::{self, Arc},
    vec::Vec,
};

impl<B> IsDefault for Cow<'_, B>
where
    B: ToOwned<Owned: IsDefault> + ?Sized,
{
    /// Returns `true` if self is `Owned` with default value.
    /// ```
    /// # extern crate alloc;
    /// #
    /// use alloc::borrow::Cow;
    /// use is_default::IsDefault;
    ///
    /// assert!(Cow::<str>::default().is_default());
    /// assert!(Cow::<str>::Owned(String::new()).is_default());
    ///
    /// // not default
    /// assert!(!Cow::<str>::Borrowed("").is_default());
    /// assert!(!Cow::<str>::Owned(String::from("x")).is_default());
    /// ```
    #[inline]
    fn is_default(&self) -> bool {
        matches!(self, Cow::Owned(v) if v.is_default())
    }
}

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

deref_impl_t!(Box);
deref_impl_t!(Rc);
deref_impl_t!(Arc);

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

is_empty_impl!(CString);
is_empty_impl!(String);

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
is_empty_impl_t!(LinkedList);
is_empty_impl_t!(Vec);
is_empty_impl_t!(VecDeque);

impl<K, V> IsDefault for BTreeMap<K, V> {
    /// Returns `true` if self is empty.
    #[inline]
    fn is_default(&self) -> bool {
        self.is_empty()
    }
}

#[cfg(feature = "bstr")]
mod feature_bstr {
    use super::*;
    is_empty_impl!(alloc::bstr::ByteString);
}
