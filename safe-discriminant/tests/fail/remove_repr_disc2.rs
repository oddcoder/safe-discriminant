use safe_discriminant::Discriminant;
use safe_discriminant_derive::remove_repr;

#[derive(Discriminant)]
#[remove_repr]
#[repr(u8)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
