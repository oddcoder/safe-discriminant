#![no_std]
//! The [Discriminant] trait, along with its corresponding derive macro,
//! offers a constant-time and safe method for extracting the primitive
//! form of the discriminant value for enums.
//!
//! The trait is derivable only when the following conditions are met:
//! 1.  The enum's is accompanied by `#[repr(x)]` attribute,
//!     where `x` must be one of the supported primitive
//!     types: `u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`,
//!     or `i64`.
//!
//! 2. All enum variants have explicit discriminants.
//!
//! # Usage
//!
//! To use this macro, simply annotate your enum with `#[derive(Discriminant)]` as follows:
//!
//! ```rust
//! use safe_discriminant::Discriminant;
//! #[derive(Discriminant)]
//! #[repr(u8)]
//! enum Foo {
//!     A = 1,
//!     B(u8) = 2,
//!     C{inner: u8} = 3,
//! }
//! let a = Foo::A;
//! let b = Foo::B(5);
//! let c = Foo::C{inner: 6};
//! assert_eq!(a.discriminant(), 1);
//! assert_eq!(b.discriminant(), 2);
//! assert_eq!(c.discriminant(), 3);
//! ```
pub use safe_discriminant_derive::Discriminant;
/// <div class="warning">
///
/// # Hazmat
/// This trait is safe to derive and use but is unsafe to implement manually.
/// Consider using `#[derive(Discriminant)]` macro instead.
/// </div>
///
/// # Safety
/// According to
/// [rust documentation](https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant)
/// This trait is only safe to implement if the enum has
/// [primitive representation](https://doc.rust-lang.org/reference/type-layout.html#primitive-representations)
/// for its discriminant.
/// That cannot be done for enums using the default representation,
/// however, as it’s undefined what layout the discriminant has
/// and where it’s stored — it might not evenbe stored at all!
/// The derive macro should take care of checking that it is always safe to call this function.

pub unsafe trait Discriminant {
    /// The type of *discriminant*, it is often represented by the type inside
    /// `#[repr(u*)]` or `#[repr(i*)]`.
    type Selector: Copy;
    /// Returns the discriminant value of enum variant we are using.
    fn discriminant(&self) -> Self::Selector {
        unsafe { *<*const _>::from(self).cast() }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_discriminant() {
        let t = trybuild::TestCases::new();
        t.pass("tests/pass/*.rs");
        t.compile_fail("tests/fail/*.rs")
    }
}
