use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(C, u8, i8)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
