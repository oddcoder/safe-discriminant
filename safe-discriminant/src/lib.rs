
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
    /// The type of *discriminant* often represented by the type inside
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
