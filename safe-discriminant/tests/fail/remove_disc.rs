use safe_discriminant::Discriminant;
use safe_discriminant_derive::remove_disc;
#[derive(Discriminant)]
#[repr(u8)]
pub enum Foo {
    #[remove_disc]
    A = 0,
    B = 1,
}

fn main() {}
