pub mod lexer;

#[derive(Debug)]
pub enum Operator {
    // Math
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // Other
    Assign,
}

#[derive(Debug)]
pub enum TokenVariant {
    Operator(Operator),
    NotImplemented,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub variant: TokenVariant,
    pub pos: usize,
}

impl Token {
    pub fn new(variant: TokenVariant, pos: usize) -> Self {
        Self { variant, pos }
    }
}
