use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(u8)]
pub enum Foo {
    A,
    B(u8),
    C { inner: u8 },
}

fn main() {}
