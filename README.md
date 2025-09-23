[![Cargo Build & Test](https://github.com/nujievik/is-default-rs/actions/workflows/rust.yml/badge.svg)](
https://github.com/nujievik/is-default-rs/actions/workflows/rust.yml)

A trait for checking if a value is default, with easy derive support
for custom types.

Example, instead of `is_none` for `Option` and `is_empty` for `Vec`
can be used `is_default` for all.

```rust
assert!(None::<u8>.is_none());
assert!(Vec::<u8>::new().is_empty());

use is_default::IsDefault;
assert!(None::<u8>.is_default());
assert!(Vec::<u8>::new().is_default());
```

The `IsDefault` trait is implemented for most standard types that
has `Default` impl. With the `derive` feature, you can easily generate
implementations for your own types:

## Derive

To use the derive macro, add the dependency with the `derive` feature
in your `Cargo.toml`:

```toml
# Cargo.toml

[dependencies]
is_default = { version = "0.1", features = ["derive"] }
```

### Structs

A struct can derive `IsDefault` if all its fields implement `IsDefault`.

```rust
use is_default::IsDefault;

#[derive(IsDefault)]
struct Unit;
assert!(Unit.is_default());

#[derive(IsDefault)]
struct Wrapper(u8);
assert!(Wrapper(0).is_default());
assert!(!Wrapper(1).is_default());

#[derive(IsDefault)]
struct Point { x: i16, y: f32 }
assert!(Point{ x: 0, y: 0.0 }.is_default());
assert!(!Point{ x: 1, y: 0.0 }.is_default());
assert!(!Point{ x: 0, y: 1.1 }.is_default());
```

### Enums

When using #[derive(IsDefault)] on an enum, you need to choose which
unit variant will be default. You do this by placing the #[is_default]
OR #[default] attribute on the variant.

This makes it possible to derive both `Default` and `IsDefault` using
the same attribute.

```rust
use is_default::IsDefault;

#[derive(IsDefault)]
enum A {
    #[is_default]
    X,
    Y,
}
assert!(A::X.is_default());
assert!(!A::Y.is_default());

#[derive(Default, IsDefault)]
enum B {
    X,
    #[default]
    Y,
}
assert!(!B::X.is_default());
assert!(B::Y.is_default());
assert!(matches!(B::default(), B::Y));
```

Also #[derive(IsDefault)] on an enum possible if it implements both
`Default` and `PartialEq`. However, this implementation may be
inefficient, since a new `Self` object must be allocated for comparison.

```rust
use is_default::IsDefault;

#[derive(PartialEq, IsDefault)]
enum C {
    X(u8),
    Y,
}
impl Default for C {
    fn default() -> C {
        C::X(0)
    }
}

assert!(C::X(0).is_default());
assert!(!C::X(1).is_default());
```
