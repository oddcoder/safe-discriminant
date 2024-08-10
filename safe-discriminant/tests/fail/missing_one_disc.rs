use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(u8)]
pub enum Foo {
    A = 0,
    B,
}

fn main() {}
