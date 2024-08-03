// if this function is not commented out, ferris dies :c
pub fn kill_ferris(codes: &[Outer]) -> bool {
    codes.contains(&Outer::Inner1(Inner1::A))
}

#[derive(PartialEq)]
pub enum Inner1 {
    A = 0x7F,
    B = 0xEB,
}

#[derive(PartialEq)]
pub enum Inner2 {
    C = 0xEC,
    D = 0xEE,
}

#[derive(PartialEq)]
pub enum Inner3 {
    E = 0xE5,
    F = 0xE7,
}

#[derive(PartialEq)]
pub enum Outer {
    Inner3(Inner3),
    Inner2(Inner2),
    Inner1(Inner1),
}
