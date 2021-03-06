use super::tokens::{init_tokens, Token, TokensType, ValueType};

use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;
use std::str::Chars;

#[derive(Debug, Clone)]
pub struct Error {
    pub line: u8,
    pub column: u8,
    pub message: String,
}

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: Chars<'a>,
    pub tokens: VecDeque<Rc<Token>>,
    start: u8,
    current: u8,
    line: u8,
    peeked: VecDeque<char>,
    token_map: BTreeMap<&'a str, TokensType>,
    errors: Vec<Error>,
    lexeme_cache: BTreeMap<Rc<String>, Rc<String>>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a String) -> Self {
        let mut peeked: VecDeque<char> = VecDeque::with_capacity(4);

        for source_char in source.chars() {
            peeked.push_back(source_char)
        }

        Scanner {
            source: source.chars(),
            tokens: VecDeque::new(),
            start: 1,
            current: 1,
            line: 1,
            peeked,
            token_map: init_tokens(),
            errors: Vec::new(),
            lexeme_cache: BTreeMap::new(),
        }
    }

    pub fn scan(&mut self) {
        loop {
            self.start = self.current;
            if !self.scan_tokens() {
                break;
            }
        }
        self.add_token(TokensType::Eof, String::from(""), None);

        if self.errors.len() != 0 {
            let errors = self
                .errors
                .iter()
                .map(|err| {
                    format!(
                        "{} in line {} column {} \n",
                        err.message, err.line, err.column
                    )
                })
                .collect::<String>();
            panic!(
                "\n\n******\nOops! scan tokens errors:\n{}******\n\n",
                errors
            );
        }
    }

    fn scan_tokens(&mut self) -> bool {
        let code = self.advance();
        match code {
            Some(code) => {
                match code {
                    '(' => self.add_token(TokensType::LeftParen, code.to_string(), None),
                    ')' => self.add_token(TokensType::RightParen, code.to_string(), None),
                    '{' => self.add_token(TokensType::LeftBrace, code.to_string(), None),
                    '}' => self.add_token(TokensType::RightBrace, code.to_string(), None),
                    ',' => self.add_token(TokensType::Comma, code.to_string(), None),
                    '.' => self.add_token(TokensType::Dot, code.to_string(), None),
                    '-' => self.add_token(TokensType::Minus, code.to_string(), None),
                    '+' => self.add_token(TokensType::Plus, code.to_string(), None),
                    ';' => self.add_token(TokensType::Semicolon, code.to_string(), None),
                    '*' => self.add_token(TokensType::Star, code.to_string(), None),
                    '!' => {
                        if self.match_char('=') {
                            let c = self.advance().unwrap();
                            self.add_token(
                                TokensType::BangEqual,
                                code.to_string() + &*c.to_string(),
                                None,
                            );
                        } else {
                            self.add_token(TokensType::Bang, code.to_string(), None);
                        }
                    }
                    '=' => {
                        if self.match_char('=') {
                            let c = self.advance().unwrap();
                            self.add_token(
                                TokensType::EqualEqual,
                                code.to_string() + &*c.to_string(),
                                None,
                            );
                        } else {
                            self.add_token(TokensType::Equal, code.to_string(), None);
                        }
                    }
                    '<' => {
                        if self.match_char('=') {
                            let c = self.advance().unwrap();
                            self.add_token(
                                TokensType::LessEqual,
                                code.to_string() + &*c.to_string(),
                                None,
                            );
                        } else {
                            self.add_token(TokensType::Less, code.to_string(), None);
                        }
                    }
                    '>' => {
                        if self.match_char('=') {
                            let c = self.advance().unwrap();
                            self.add_token(
                                TokensType::GreaterEqual,
                                code.to_string() + &*c.to_string(),
                                None,
                            );
                        } else {
                            self.add_token(TokensType::Greater, code.to_string(), None);
                        }
                    }
                    '/' => {
                        if self.match_char('/') {
                            'comment: loop {
                                let peek = self.peek();
                                match peek {
                                    Some(c) => {
                                        if c == '\n' {
                                            break 'comment;
                                        }
                                        self.advance();
                                    }
                                    _ => {
                                        break 'comment;
                                    }
                                }
                            }
                        } else {
                            self.add_token(TokensType::Slash, code.to_string(), None);
                        }
                    }
                    ' ' | '\r' | '\t' => {}
                    '\n' => {
                        self.line += 1;
                        self.current = 1;
                    }
                    '"' | '\'' => self.handle_string(code),
                    '0'..='9' => self.handle_digit(code),
                    'a'..='z' | 'A'..='Z' | '\u{4E00}'..='\u{9FA5}' | '_' => {
                        self.handle_alpha(code)
                    }
                    _ => self.errors.push(Error {
                        line: self.line,
                        //TODO:
                        column: self.start,
                        message: String::from("Unexpected character"),
                    }),
                }
                true
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<char> {
        let front = self.peeked.pop_front();
        if let Some(_) = front {
            self.current += 1;
        }
        front
    }

    fn add_token(&mut self, token_type: TokensType, lexeme: String, literal: Option<ValueType>) {
        let lexeme_name;

        if self.lexeme_cache.contains_key(&lexeme) {
            lexeme_name = self.lexeme_cache.get(&lexeme).unwrap().clone();
        } else {
            lexeme_name = Rc::new(lexeme);

            self.lexeme_cache
                .insert(lexeme_name.clone(), lexeme_name.clone());
        }

        let token = Token {
            token_type,
            lexeme: lexeme_name,
            literal,
            line: self.line,
            column: self.start,
        };
        self.tokens.push_back(Rc::new(token));
    }

    fn match_char(&mut self, expected: char) -> bool {
        match self.peek() {
            Some(c) => {
                if c == expected {
                    return true;
                }
                return false;
            }
            _ => return false,
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.peeked.front().cloned()
    }

    fn peek_next(&mut self) -> Option<char> {
        if self.peeked.len() >= 2 {
            return Some(self.peeked[1].clone());
        }
        None
    }

    fn handle_string(&mut self, code: char) {
        if code == '"' || code == '\"' {
            let mut lox_string = String::new();
            loop {
                match self.peek() {
                    Some(c) => {
                        if c != code {
                            if c == '\n' {
                                self.line += 1;
                            }
                            let s = &*self.advance().unwrap().to_string();
                            lox_string += s;
                        } else {
                            self.advance();
                            break;
                        }
                    }
                    _ => {
                        self.errors.push(Error {
                            line: self.line,
                            column: self.start,
                            message: String::from("Unterminated string"),
                        });
                        break;
                    }
                }
            }
            self.add_token(
                TokensType::String,
                code.to_string() + &*lox_string + &*code.to_string(),
                Some(ValueType::String(lox_string)),
            );
        }
    }

    fn handle_digit(&mut self, code: char) {
        let mut lox_digit = String::from(code);
        loop {
            match self.peek() {
                Some(c) => {
                    if ('0'..='9').contains(&c) {
                        let s = &*self.advance().unwrap().to_string();
                        lox_digit += s;
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }

        if let Some(c) = self.peek() {
            if c == '.' {
                if let Some(c_next) = self.peek_next() {
                    if ('0'..='9').contains(&c_next) {
                        let s = &*self.advance().unwrap().to_string();
                        lox_digit += s;

                        while let Some(peek_char) = self.peek() {
                            if ('0'..='9').contains(&peek_char) {
                                let s = &*self.advance().unwrap().to_string();
                                lox_digit += s;
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
        let value: f64 = lox_digit.parse().unwrap();
        self.add_token(
            TokensType::Number,
            lox_digit,
            Some(ValueType::Number(value)),
        );
    }

    fn handle_alpha(&mut self, code: char) {
        let mut lox_alpha = String::from(code);

        while let Some(c) = self.peek() {
            match c {
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '\u{4E00}'..='\u{9FA5}' | '_' => {
                    let s = &*self.advance().unwrap().to_string();
                    lox_alpha += s;
                }
                _ => break,
            }
        }

        if let Some(&token_type) = self.token_map.get(&*lox_alpha) {
            self.add_token(token_type, lox_alpha, None);
        } else {
            self.add_token(TokensType::Identifier, lox_alpha, None);
        }
    }
}
