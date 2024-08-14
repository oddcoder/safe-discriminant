use safe_discriminant::Discriminant;

// TODO
// pub enum Foo<'a, T: 'a> will not work

#[derive(Discriminant)]
#[repr(i64)]
pub enum Foo<'a, T>
where
    T: 'a,
{
    A(T) = 1,
    B(&'a str) = -1,
}

fn main() {
    let a = Foo::A(1);
    let b: Foo<'_, u8> = Foo::B("hello world");
    assert_eq!(a.discriminant(), 1);
    assert_eq!(b.discriminant(), -1);
}
