# safe-discriminant

`safe-discriminant` provides a minimalistic, `no_std` compatible trait and
procedural macro for extracting discriminants from enums at zero cost. It
automatically generates `unsafe { ... }` blocks, ensuring semantic safety so
you don’t have to worry about it.

## Installation

This crate is available on [crates.io](crates.io) and can be easily included in
your project by:
* Adding the following line to your Cargo.toml:
  ```toml
  [dependencies]
  safe-discriminant = "0.2.0"
  ```
* Or runing this command in your cargo project:
  ```sh
  $ cargo add safe-discriminant
  ```

## Usage

```rust
use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(i64)]
pub enum Foo<T> {
    A = 1,
    B(T) = -1,
    C { fst: T, snd: T } = -2,
}

fn main() {
    let a: Foo<u8> = Foo::A;
    let b = Foo::B(5);
    let c = Foo::C { fst: 2, snd: 3 };
    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), -1);
    assert_eq!(c.discriminant(), -2);
}
```

## Similar Projects

* [strum](https://crates.io/crates/strum) provides a collection of macros
  designed to simplify working with enums. Among these macros is
  [`EnumDiscriminants`](https://docs.rs/strum_macros/latest/strum_macros/derive.EnumDiscriminants.html),
  which extracts the name of each variant from the enum and organizes them into
  a separate enum.
