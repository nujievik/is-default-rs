#![cfg_attr(feature = "ascii_char", feature(ascii_char, ascii_char_variants))]
#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

use is_default::IsDefault;

macro_rules! test {
    ($ty:ident, $true:expr; $( $false:expr ),* ) => {
        test!($ty, $ty, $true; $( $false ),* );
    };

    ($fn:ident, $ty:ty, $true:expr; $( $false:expr ),* ) => {
        #[test]
        fn $fn() {
            assert!(<$ty>::default().is_default());
            assert!($true.is_default());
            $( assert!(!$false.is_default()) );*
        }
    };
}

test!(bool, false; true);
test!(char, '\x00'; 'a', char::MAX);

#[cfg(feature = "f16")]
test!(f16, 0f16; f16::MAX, 1f16, f16::MIN, f16::NAN);

test!(f32, 0f32; f32::MAX, 1f32, f32::MIN, f32::NAN);
test!(f64, 0f32; f64::MAX, 1f64, f64::MIN, f64::NAN);

#[cfg(feature = "f128")]
test!(f128, 0f128; f128::MAX, 1f128, f128::MIN, f128::NAN);

test!(i8, 0i8; i8::MAX, 1i8, i8::MIN);
test!(i16, 0i16; i16::MAX, 1i16, i16::MIN);
test!(i32, 0i32; i32::MAX, 1i32, i32::MIN);
test!(i64, 0i64; i64::MAX, 1i64, i64::MIN);
test!(i128, 0i128; i128::MAX, 1i128, i128::MIN);
test!(isize, 0isize; isize::MAX, 1isize, isize::MIN);

test!(u8, 0u8; u8::MAX, 1u8);
test!(u16, 0u16; u16::MAX, 1u16);
test!(u32, 0u32; u32::MAX, 1u32);
test!(u64, 0u64; u64::MAX, 1u64);
test!(u128, 0u128; u128::MAX, 1u128);
test!(usize, 0usize; usize::MAX, 1usize);

macro_rules! test_borrowed {
    ($fn:ident, $true:expr; $( $false:expr ),* ) => {
        #[test]
        fn $fn() {
            assert!($true.is_default());
            $( assert!(!$false.is_default()) );*
        }
    };
}

test_borrowed!(str, ""; "x");

test!(slice, &[u8], &[0u8]; &[1u8]);
test!(array, [u8; 0], [0u8]; [1u8]);

macro_rules! test_tuple {
    ($fn:ident, $( $Ts:tt)* ) => {
        #[test]
        fn $fn() {
            let mut tuple = ( $($Ts,)* );
            assert!(tuple.is_default());
            test_tuple!(@assert_false, tuple, $($Ts)*);
        }
    };
    (@assert_false, $tuple:expr, $T:tt) => {{
        $tuple.$T = 1;
        assert!(!$tuple.is_default());
        $tuple.$T = 0;
    }};
    (@assert_false, $tuple:expr, $T:tt $($Ts:tt)*) => {
        test_tuple!(@assert_false, $tuple, $T);
        test_tuple!(@assert_false, $tuple, $($Ts)*);
    };
}

test!(unit, (), (););

test_tuple!(tuple_t, 0);
test_tuple!(tuple_t_a, 0 0);
test_tuple!(tuple_t_a_b, 0 0 0);
test_tuple!(tuple_t_a_b_c_d, 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e, 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f, 0 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f_g, 0 0 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f_g_h, 0 0 0 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f_g_h_i, 0 0 0 0 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f_g_h_i_j, 0 0 0 0 0 0 0 0 0 0 0);
test_tuple!(tuple_t_a_b_c_d_e_f_g_h_i_j_k, 0 0 0 0 0 0 0 0 0 0 0 0);

#[cfg(feature = "ascii_char")]
mod ascii_char {
    use core::ascii::Char;
    test!(ascii_char, Char, Char::Null; Char::StartOfHeading, Char::MAX);
}

#[cfg(feature = "std")]
mod std_types {
    use is_default::IsDefault;
    use std::{
        borrow::Cow,
        cell::{Cell, OnceCell, RefCell},
        collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque},
        ffi::{CString, OsStr, OsString},
        io::Cursor,
        marker::PhantomPinned,
        num::Wrapping,
        path::PathBuf,
        rc::Rc,
        sync::{Arc, OnceLock},
        time::Duration,
    };

    test!(wrapping, Wrapping::<u8>, Wrapping(0u8); Wrapping(1u8));
    test!(phantom_pinned, PhantomPinned, PhantomPinned;);

    test!(string, String, String::from(""); String::from("x"));
    test!(c_string, CString, CString::new("").unwrap(); CString::new("x").unwrap());

    test!(os_string, OsString, OsString::from(""); OsString::from("x"));
    test!(path_buf, PathBuf, PathBuf::from(""); PathBuf::from("x"));

    test_borrowed!(c_str, c""; c"x");
    test_borrowed!(os_str, OsStr::new(""); OsStr::new("x"));

    test!(duration, Duration, Duration::ZERO; Duration::new(1, 0));

    test!(option, Option::<u8>, None::<u8>; Some(0u8));

    test!(b_tree_set, BTreeSet::<u8>, BTreeSet::<u8>::new(); BTreeSet::from([0u8]));
    test!(hash_set, HashSet::<u8>, HashSet::<u8>::new(); HashSet::from([0u8]));
    test!(linked_list, LinkedList::<u8>, LinkedList::<u8>::new(); LinkedList::from([0u8]));

    test!(vec, Vec::<u8>, Vec::<u8>::new(); Vec::from([0u8]));
    test!(vec_deque, VecDeque::<u8>, VecDeque::<u8>::new(); VecDeque::from([0u8]));

    test!(b_tree_map, BTreeMap::<u8, u8>, BTreeMap::<u8, u8>::new(); BTreeMap::<u8, u8>::from([(0u8, 0u8)]));
    test!(hash_map, HashMap::<u8, u8>, HashMap::<u8, u8>::new(); HashMap::<u8, u8>::from([(0u8, 0u8)]));

    test!(arc, Arc::<u8>, Arc::new(0u8); Arc::new(1u8));
    test!(test_box, Box::<u8>, &Box::new(0u8); &Box::new(1u8));
    test!(rc, Rc::<u8>, Rc::new(0u8); Rc::new(1u8));

    test!(cow, Cow::<str>, Cow::from(""); Cow::from("x"));
    test!(cell, Cell::<u8>, Cell::new(0u8); Cell::new(1u8));
    test!(ref_cell, RefCell::<u8>, RefCell::new(0u8); RefCell::new(1u8));

    macro_rules! test_once {
        ($fn:ident, $ty:ty) => {
            #[test]
            fn $fn() {
                let v = <$ty>::default();
                assert!(v.is_default());
                v.set(0u8).unwrap();
                assert!(!v.is_default());
            }
        };
    }

    test_once!(once_cell, OnceCell::<u8>);
    test_once!(once_lock, OnceLock::<u8>);

    #[test]
    fn cursor() {
        let mut c = Cursor::new([0u8; 1]);
        assert!(c.is_default());
        c.set_position(1);
        assert!(!c.is_default());
    }

    #[cfg(not(feature = "via_default_eq"))]
    mod no_via_default_eq {
        use is_default::IsDefault;
        use std::{
            collections::BinaryHeap,
            path::Path,
            sync::{
                Mutex, RwLock,
                atomic::{
                    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8,
                    AtomicU16, AtomicU32, AtomicU64, AtomicUsize,
                },
            },
        };

        test!(atomic_bool, AtomicBool, AtomicBool::new(false); AtomicBool::new(true));

        test!(atomic_i8, AtomicI8, AtomicI8::new(0); AtomicI8::new(i8::MAX), AtomicI8::new(1));
        test!(atomic_i16, AtomicI16, AtomicI16::new(0); AtomicI16::new(i16::MAX), AtomicI16::new(1));
        test!(atomic_i32, AtomicI32, AtomicI32::new(0); AtomicI32::new(i32::MAX), AtomicI32::new(1));
        test!(atomic_i64, AtomicI64, AtomicI64::new(0); AtomicI64::new(i64::MAX), AtomicI64::new(1));
        test!(atomic_isize, AtomicIsize, AtomicIsize::new(0); AtomicIsize::new(isize::MAX), AtomicIsize::new(1));

        test!(atomic_u8, AtomicU8, AtomicU8::new(0); AtomicU8::new(u8::MAX), AtomicU8::new(1));
        test!(atomic_u16, AtomicU16, AtomicU16::new(0); AtomicU16::new(u16::MAX), AtomicU16::new(1));
        test!(atomic_u32, AtomicU32, AtomicU32::new(0); AtomicU32::new(u32::MAX), AtomicU32::new(1));
        test!(atomic_u64, AtomicU64, AtomicU64::new(0); AtomicU64::new(u64::MAX), AtomicU64::new(1));
        test!(atomic_usize, AtomicUsize, AtomicUsize::new(0); AtomicUsize::new(usize::MAX), AtomicUsize::new(1));

        test_borrowed!(path, Path::new(""); Path::new("x"));

        test!(binary_heap, BinaryHeap::<u8>, BinaryHeap::<u8>::new(); BinaryHeap::from([0u8]));

        test!(rw_lock, RwLock::<u8>, RwLock::new(0u8); RwLock::new(1u8));
        test!(mutex, Mutex::<u8>, Mutex::new(0u8); Mutex::new(1u8));

        macro_rules! test_weak {
            ($fn:ident, $up:ident, $mod:path, { $($imports:ident),+ }) => {
                #[test]
                fn $fn() {
                    use $mod::{ $($imports),+ };

                    assert!(Weak::<u8>::new().is_default());
                    let x = $up::new(0u8);
                    let xw = $up::downgrade(&x);
                    assert!(!xw.is_default());
                    drop(x);
                    assert!(xw.is_default());
                }
            };
        }

        test_weak!(rc_weak, Rc, std::rc, {Rc, Weak});
        test_weak!(arc_weak, Arc, std::sync, {Arc, Weak});
    }
}
