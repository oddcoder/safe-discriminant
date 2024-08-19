use safe_discriminant::Discriminant;
use safe_discriminant_derive::do_nothing;

#[repr(u8)]
#[derive(Discriminant)]
#[do_nothing]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
