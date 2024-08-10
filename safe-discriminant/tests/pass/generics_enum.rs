use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(i64)]
pub enum Foo<T> {
    A(T) = 1,
    B(T) = -1,
}

fn main() {
    let a = Foo::A(1);
    let b = Foo::B(5);
    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), -1);
}
