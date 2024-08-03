// if this function is not commented out, ferris dies :c
pub fn kill_ferris(a: &Outer, b: &Outer) -> bool {
    a == b
}

#[derive(PartialEq)]
pub enum Inner1 {
    A,
    B = 0x80,
}

#[derive(PartialEq)]
pub enum Inner2 {
    C = 0x81,
    D,
}

#[derive(PartialEq)]
pub enum Inner3 {
    E,
    F,
}

#[derive(PartialEq)]
pub enum Outer {
    Inner3(Inner3),
    Inner2(Inner2),
    Inner1(Inner1),
}
