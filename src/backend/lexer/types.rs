use crate::backend::types::BObject;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operator {
    // Math
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,

    // Bitwise Operations
    LAssign,
    RAssign,
    Gt,
    Lt,
    Eq,
    Geq,
    Leq,
    Neq,
    Shl,
    Shr,
    // Other
    //Typeis, (for typing, will add later)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Keyword {
    // Variables
    Declare,

    // Conditionals
    If,
    Else,
    Endif,
    Then,
    True,
    False,

    // Case of
    Case,
    Of,

    // Loops
    For,
    Next,
    To,
    Repeat,
    Until,
    While,
    Endwhile,

    // functions
    Function,
    Endfunction,
    //Returns, (for typing, will add later)
    Return,
    Call,

    // Builtins
    Output,
    Input,
    // Modules
    //Use,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Separator {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LCurly,
    RCurly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenVariant {
    Operator(Operator),
    Keyword(Keyword),
    Separator(Separator),
    Ident(String),
    Literal(BObject),
    Invalid,
    Unrecognized,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub variant: TokenVariant,
    pub pos: usize,
}

impl Token {
    pub fn new(variant: TokenVariant, pos: usize) -> Self {
        Self { variant, pos }
    }
}
