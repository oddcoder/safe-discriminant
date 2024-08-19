use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(u8)]
#[repr(u16)]
enum Foo {
    A = 1,
    B = 2,
}

fn main() {}
