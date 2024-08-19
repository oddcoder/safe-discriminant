// attempting to confuse Discriminant into thinking
// the fake repr is the real repr
use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[safe_discriminant_derive::repr(u8)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
