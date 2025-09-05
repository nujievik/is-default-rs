[![Cargo Build & Test](https://github.com/nujievik/rust_is_default/actions/workflows/rust.yml/badge.svg)](
https://github.com/nujievik/rust_is_default/actions/workflows/rust.yml)

A unified API for checking if a value is default, with easy derive
support for custom types.

Example, instead of `is_none` for `Option` and `is_empty` for `Vec`
can be used `is_default` for all.

```rust
assert!(None::<u8>.is_none());
assert!(Vec::<u8>::new().is_empty());

use is_default::IsDefault;
assert!(None::<u8>.is_default());
assert!(Vec::<u8>::new().is_default());
```

The `IsDefault` trait is implemented for most standard types.
With the `derive` feature, you can easily generate implementations for
your own types:

## Derive

To use derive macro add dependency with the feature `derive` in
Cargo.toml:
```
is_default = { version = "1", features = ["derive"] }
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
assert!(!Point{ x: 0, y: 0.1 }.is_default());
```

### Enums

Enums can derive `IsDefault` using the `#[is_default]` or `#[default]`
attribute. This allows deriving both `Default` and `IsDefault` using
the same attribute.

```rust
use is_default::IsDefault;

#[derive(IsDefault)]
enum X {
    A,
    #[is_default]
    B,
}
assert!(X::B.is_default());
assert!(!X::A.is_default());

#[derive(IsDefault)]
enum Y {
    #[default]
    A,
    B,
}
assert!(Y::A.is_default());
assert!(!Y::B.is_default());

#[derive(Default, IsDefault)]
enum Z {
    A,
    #[default]
    B,
}
assert!(Z::B.is_default());
assert!(!Z::A.is_default());
assert!(matches!(Z::default(), Z::B));
```
