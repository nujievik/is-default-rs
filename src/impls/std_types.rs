use crate::IsDefault;
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    io::{Cursor, Empty, Sink},
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock, RwLock},
};

unit_impl!(Empty);
unit_impl!(Sink);

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

is_empty_impl_t!(HashSet);
is_empty_impl_k_v!(HashMap);

lock_impl!(RwLock, try_read);
lock_impl!(Mutex, try_lock);

once_impl!(OnceLock);

impl<T> IsDefault for Cursor<T> {
    /// Returns `true` if cursor position is `0`.
    #[inline]
    fn is_default(&self) -> bool {
        matches!(self.position(), 0u64)
    }
}

#[cfg(feature = "bstr")]
mod feature_bstr {
    use crate::IsDefault;
    use std::bstr::{ByteStr, ByteString};

    is_empty_impl!(ByteStr);
    is_empty_impl!(ByteString);
}
