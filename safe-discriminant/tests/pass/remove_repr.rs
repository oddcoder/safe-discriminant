// this test makes sure that remove repr actually removes repr
use safe_discriminant_derive::remove_repr;

#[remove_repr]
#[repr(FOO_BAR_TYPE_DOES_NOT_EXIST)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
