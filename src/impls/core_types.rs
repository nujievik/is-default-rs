use crate::IsDefault;

matches_impl!(bool, false);
matches_impl!(char, '\x00');

#[cfg(feature = "f16")]
matches_impl!(f16, 0f16);

matches_impl!(f32, 0f32);
matches_impl!(f64, 0f64);

#[cfg(feature = "f128")]
matches_impl!(f128, 0f128);

matches_impl!(i8, 0i8);
matches_impl!(i16, 0i16);
matches_impl!(i32, 0i32);
matches_impl!(i64, 0i64);
matches_impl!(i128, 0i128);
matches_impl!(isize, 0isize);

matches_impl!(u8, 0u8);
matches_impl!(u16, 0u16);
matches_impl!(u32, 0u32);
matches_impl!(u64, 0u64);
matches_impl!(u128, 0u128);
matches_impl!(usize, 0usize);

is_empty_impl!(str);

ref_impl!(&T);
ref_impl!(&mut T);

impl<T> IsDefault for [T]
where
    T: IsDefault,
{
    /// Returns `true` if:
    /// - slice is empty
    /// - all slice elements is default
    #[inline]
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
    /// Returns `true`:
    /// - for array [T; 0]
    /// - if all array elements is default
    #[inline]
    fn is_default(&self) -> bool {
        self.as_slice().is_default()
    }
}

macro_rules! tuple_impls {
    () => {
        unit_impl!(());
    };

    ($T:ident $( $Ts:ident)*) => {
        maybe_tuple_doc! {
            $T $($Ts)* @
            #[allow(non_snake_case)]
            impl<$T: IsDefault, $($Ts: IsDefault),*> IsDefault for ($T, $($Ts,)*) {
                /// Returns `true` if all tuple fields is default.
                fn is_default(&self) -> bool {
                    let ($T, $($Ts,)*) = self;
                    $T.is_default() $( && $Ts.is_default() )*
                }
            }
        }

        tuple_impls!($($Ts)*);
    };
}

// If this is a unary tuple, it adds a doc comment.
// Otherwise, it hides the docs entirely.
macro_rules! maybe_tuple_doc {
    ($a:ident @ #[$meta:meta] $item:item) => {
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        #[$meta]
        $item
    };
    ($a:ident $($rest_a:ident)+ @ #[$meta:meta] $item:item) => {
        #[doc(hidden)]
        #[$meta]
        $item
    };
}

tuple_impls!(K J I H G F E D C B A T);

#[cfg(feature = "ascii_char")]
mod ascii_char {
    use core::ascii::Char;

    impl crate::IsDefault for Char {
        /// Returns `true` if self is `Char::Null`.
        #[inline]
        fn is_default(&self) -> bool {
            matches!(self, Char::Null)
        }
    }
}
