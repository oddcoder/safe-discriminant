use safe_discriminant::Discriminant;

#[derive(Discriminant)]
#[repr(C)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
