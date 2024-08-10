# safe-discriminant

`safe-discriminant` offers a minimalistic, `no_std` compatiable, zero cost trait + procedural macro for extraction of discriminant out of enums.

## Installation

This crate is available on crates.io and can be used by adding the following to your project's Cargo.toml:

```toml
[dependencies]
safe-discriminant = "0.1.0"
```
Or run this command in your cargo project:

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