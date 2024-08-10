use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(u8)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {
    assert_eq!(Foo::A.discriminant(), 0);
    assert_eq!(Foo::B.discriminant(), 1);
}
