use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokensType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub enum ValueType {
    Number(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokensType,
    pub lexeme: String,
    pub line: u8,
    pub column: u8,
    pub literal: Option<ValueType>,
}

#[macro_export]
macro_rules! hash_map_negative {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($v,$k);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! hash_map {
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($k,$v);
            )*
            map
        }
    };
}

pub fn init_tokens<'a>() -> HashMap<&'a str, TokensType> {
    hash_map_negative! {
        TokensType::And => "and",
        TokensType::Class => "class",
        TokensType::Else => "else",
        TokensType::False => "false",
        TokensType::Fun => "fun",
        TokensType::For => "for",
        TokensType::If => "if",
        TokensType::Nil => "nil",
        TokensType::Or => "or",
        TokensType::Print => "print",
        TokensType::Return => "return",
        TokensType::Super => "super",
        TokensType::This => "this",
        TokensType::True => "true",
        TokensType::Var => "var",
        TokensType::While => "while"
    }
}
