use safe_discriminant::Discriminant;
use safe_discriminant_derive::do_nothing;

#[do_nothing]
#[repr(u8)]
#[derive(Discriminant)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {
    assert_eq!(Foo::A.discriminant(), 0);
    assert_eq!(Foo::B.discriminant(), 1);
}
