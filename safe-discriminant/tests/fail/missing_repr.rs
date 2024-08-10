use safe_discriminant::Discriminant;

#[derive(Discriminant)]
pub enum Foo {
    A = 0,
    B = 1,
}

fn main() {}
