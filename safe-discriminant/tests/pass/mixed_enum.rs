use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(C, i64)]
pub enum Foo<T> {
    A = 1,
    B(T) = -1,
    C { fst: T, snd: T } = -2,
}

fn main() {
    let a: Foo<u8> = Foo::A;
    let b = Foo::B(5);
    let c = Foo::C { fst: 2, snd: 3 };
    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), -1);
    assert_eq!(c.discriminant(), -2);
}
