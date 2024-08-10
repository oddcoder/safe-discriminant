use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(i64)]
pub enum Foo {
    A { fst: u8, snd: u8 } = 0xffff1234,
    B { fst: u32, snd: u32 } = -0xffff1234,
}

fn main() {
    let a = Foo::A { fst: 0, snd: 1 };
    let b = Foo::B { fst: 2, snd: 3 };
    assert_eq!(a.discriminant(), 0xffff1234);
    assert_eq!(b.discriminant(), -0xffff1234);
}
