use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(i8)]
pub enum Foo {
    A(u8, u8) = 0,
    B(u32, u32) = -1,
}

fn main() {
    let a = Foo::A(0, 1);
    let b = Foo::B(2, 3);
    assert_eq!(a.discriminant(), 0);
    assert_eq!(b.discriminant(), -1);
}
