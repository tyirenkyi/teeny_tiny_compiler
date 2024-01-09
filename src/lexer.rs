use crate::token::Token;
use crate::enums::TokenKind;

use core::fmt;

#[derive(Debug, Clone)]
pub struct LexingError;

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid lexing token")
    }
}


pub struct Lexer {
    pub source: String,
    pub cur_char: char,
    pub cur_pos: usize,
    pub at_start: bool,
}

impl Default for Lexer {
    fn default() -> Self {
        Self { source: Default::default(), cur_char: Default::default(), cur_pos: 0, at_start: true }
    }
}

impl Lexer {
    pub fn build(source: String) -> Lexer {
        let source_plus_newline = format!("{}\n", source);
        let mut lexer = Lexer {
            source: source_plus_newline,
            ..Default::default()
        };
        lexer.next_char();
        lexer
    }

    pub fn next_char(&mut self) {
        if !self.at_start {
            self.cur_pos += 1;
        }
        if self.cur_pos >= self.source.chars().count() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.source[self.cur_pos..self.cur_pos+1].chars().next().unwrap();
        }
        if self.at_start {
            self.at_start = false;
        }

    }

    pub fn peek(& self) -> char {
        if self.cur_pos + 1 >= self.source.chars().count() {
            return '\0';
        }
        
        self.source[self.cur_pos + 1..self.cur_pos + 2].chars().next().unwrap()
    }

    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    pub fn get_token(&mut self) -> Result<Token, LexingError> {
        self.skip_whitespace();
        self.skip_comment();
        
        let mut token: Token = Token::default();
        
        match self.cur_char.is_ascii_digit() {
            true => {
                let start_pos = self.cur_pos;
                while self.peek().is_ascii_digit() {
                    self.next_char();
                }
                if self.peek() == '.' {
                    self.next_char();

                    if !self.peek().is_ascii_digit() {
                        return Err(LexingError);
                    }

                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                }

                let mut token_text = String::new();
                self.source[start_pos..self.cur_pos + 1].clone_into(&mut token_text);
                token = Token {
                    text: token_text,
                    kind: TokenKind::NUMBER
                }
            },
            _ => {
                match self.cur_char.is_alphabetic() {
                    true => {
                        let start_pos = self.cur_pos;
                        while self.peek().is_alphanumeric() {
                            self.next_char();
                        }
                        let mut text = String::new();
                        self.source[start_pos..self.cur_pos + 1].clone_into(&mut text);
                        let keyword = Token::check_if_keyword(&text);
                        match keyword {
                            Some(kind) => {
                                token = Token {
                                    text,
                                    kind
                                }
                            },
                            None => {
                                token = Token {
                                    text,
                                    kind: TokenKind::IDENT
                                }
                            }
                        }
                        
                    },
                    _ => {
                        match self.cur_char {
                            '+' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::PLUS
                                }
                            },
                            '-' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::MINUS
                                }
                            },
                            '*' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::ASTERISK
                                }
                            },
                            '/' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::SLASH
                                }
                            },
                            '=' => {
                                if self.peek() == '=' {
                                    let previous_char = self.cur_char;
                                    self.next_char();
                                    let mut text = String::with_capacity(2);
                                    text.insert(0, previous_char);
                                    text.insert(1, self.cur_char);
                                    token = Token {
                                        text,
                                        kind: TokenKind::EQEQ
                                    }
                                } else {
                                    token = Token {
                                        text: String::from(self.cur_char),
                                        kind: TokenKind::EQ
                                    }
                                }
                            },
                            '>' => {
                                if self.peek() == '=' {
                                    let previous_char = self.cur_char;
                                    self.next_char();
                                    let mut text = String::with_capacity(2);
                                    text.insert(0, previous_char);
                                    text.insert(1, self.cur_char);
                                    token = Token {
                                        text,
                                        kind: TokenKind::GTEQ
                                    }
                                } else {
                                    token = Token  {
                                        text: String::from(self.cur_char),
                                        kind: TokenKind::GT
                                    }
                                }
                            },
                            '<' => {
                                if self.peek() == '=' {
                                    let previous_char = self.cur_char;
                                    self.next_char();
                                    let mut text = String::with_capacity(2);
                                    text.insert(0, previous_char);
                                    text.insert(1, self.cur_char);
                                    token = Token {
                                        text,
                                        kind: TokenKind::LTEQ
                                    }
                                } else {
                                    token = Token {
                                        text: String::from(self.cur_char),
                                        kind: TokenKind::LT
                                    }
                                }
                            },
                            '!' => {
                                if self.peek() == '=' {
                                    let previous_char = self.cur_char;
                                    self.next_char();
                                    let mut text = String::with_capacity(2);
                                    text.insert(0, previous_char);
                                    text.insert(1, self.cur_char);
                                    token = Token {
                                        text,
                                        kind: TokenKind::NOTEQ
                                    }
                                } else {
                                    return Err(LexingError);
                                }
                            },
                            '\"' => {
                                self.next_char();
                                let start_pos = self.cur_pos;
                
                                while self.cur_char != '\"' {
                                    if self.cur_char == '\r' || self.cur_char == '\n'
                                        || self.cur_char == '\t' || self.cur_char == '\\'
                                        || self.cur_char == '%' {
                                            return Err(LexingError);
                                        }
                                    self.next_char();
                                }
                
                                let mut token_text = String::new();
                                self.source[start_pos..self.cur_pos].clone_into(&mut token_text);
                                token = Token {
                                    text: token_text,
                                    kind: TokenKind::STRING
                                }
                            },
                
                            '\n' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::NEWLINE
                                }
                            },
                            '\0' => {
                                token = Token {
                                    text: String::from(self.cur_char),
                                    kind: TokenKind::EOF
                                }
                            },
                            _ => return Err(LexingError)
                        }
                    }
                }
            }
        }


        self.next_char();
        Ok(token)
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '#' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }

}