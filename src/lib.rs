// if this function is not commented out, ferris dies :c
pub fn kill_ferris(a: &Outer, b: &Outer) -> bool {
    a == b
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
