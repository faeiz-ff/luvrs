#[derive(Debug)]
pub enum Token {
    Id(usize, usize),
    Int(usize, usize),
    Float(usize, usize),
    Plus(usize),
    Minus(usize),
    Star(usize),
    Slash(usize),
    If(usize),
    Error(usize),
}
