[![Tests](https://github.com/nujievik/is-default-rs/actions/workflows/rust.yml/badge.svg)](
https://github.com/nujievik/is-default-rs/actions/workflows/rust.yml)

A trait for checking whether a value is the default, with convenient
derive support for custom types.

The default value is defined as the value returned by the `Default`
trait. Therefore, any implementation of `IsDefault` must ensure that
`self == &Self::default()` holds true.


## Features

| Feature             | Default | Description                   |
|---------------------|---------|-------------------------------|
| `derive`            | yes     | Derive trait for a type       |
| `std`               | yes     | Implements for std-types      |
| `via_default_eq`    | no      | Generic implementation via `Default` & `PartialEq` |

Nightly-only:

| Nightly Feature     | Default | Description                   |
|---------------------|---------|-------------------------------|
| `nightly`           | no      | Enable all below nightly features |
| `ascii_char`        | no      | Core `ascii_char`             |
| `f16`               | no      | Core `f16`                    |
| `f128`              | no      | Core `f128`                   |


### Derive

The `IsDefault` trait is already implemented for most core and std
types that implement `Default`. For custom types, you can derive
`IsDefault` using derive:

```toml
# Cargo.toml

[dependencies]
is_default = { version = "0.1", features = ["derive"] }
```

#### Structs

A struct can derive` IsDefault` if all of its fields implement
`IsDefault`:

```rust
# #[cfg(feature = "derive")] {
# use is_default::IsDefault;
#
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
# }
```

#### Enums

When deriving `IsDefault` for an enum, you must specify which unit
variant should be considered the default. This is done by applying
the `#[is_default]` or `#[default]` attribute to the variant:

```rust
# #[cfg(feature = "derive")] {
# use is_default::IsDefault;
#
#[derive(IsDefault)]
enum A {
    #[is_default]
    X,
    Y,
}
assert!(A::X.is_default());
assert!(!A::Y.is_default());
# }
```

`#[default]` attribute possible to derive both `Default` and
`IsDefault`:

```rust
# #[cfg(feature = "derive")] {
# use is_default::IsDefault;
#
#[derive(Default, IsDefault)]
enum B {
    X,
    #[default]
    Y,
}
assert!(!B::X.is_default());
assert!(B::Y.is_default());
assert!(matches!(B::default(), B::Y));
# }
```

You can also derive `IsDefault` for enums that implement both `Default`
and `PartialEq`. This approach is more general but may be less
efficient, since a new value must be allocated for comparison:

```rust
# #[cfg(all(feature = "derive", not(feature = "via_default_eq")))] {
# use is_default::IsDefault;
#
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
# }
```

### via_default_eq

By default, `IsDefault` is manually implemented for core and std types.
This approach is fast and has no trait dependencies but requires manual
implementation for custom types.

Alternatively, you can enable a generic implementation of `IsDefault`
for all types that implement both `Default` and `PartialEq`. This is
the simplest option, but it may be less efficient, as it allocates a
new value for comparison:

```toml
# Cargo.toml

[dependencies]
is_default = { version = "0.1.1", features = ["via_default_eq"] }
```


## no_std

For `no_std` builds, add `is_default` to your `Cargo.toml` with default
features disabled:

```toml
# Cargo.toml

[dependencies]
is_default = { version = "0.1.1", default-features = false, features = ["derive"] }
```
