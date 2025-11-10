#[cfg(all(feature = "derive", not(feature = "via_default_eq")))]
mod derive {
    use is_default::IsDefault;

    #[test]
    fn tuple_ref_structs() {
        #[derive(IsDefault)]
        struct Ref<'a>(&'a u8);

        #[derive(IsDefault)]
        struct RefMut<'a>(&'a u8);

        let mut zero = 0;
        let mut non_zero = 1;

        assert!(Ref(&zero).is_default());
        assert!(RefMut(&mut zero).is_default());
        assert!(!Ref(&non_zero).is_default());
        assert!(!RefMut(&mut non_zero).is_default());
    }
}
