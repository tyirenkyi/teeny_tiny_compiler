mod lexer;
mod token;
mod enums;
mod emitter;

use teeny_tiny_compiler::Config;

use std::{env, process};

fn main() {
    println!("Teeny Tiny Compiler");

    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = teeny_tiny_compiler::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use crate::{lexer, token, enums};

    #[test]
    fn test_peek() {
        let source = String::from("LET foobar = 123 \n");
        let lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.peek(), 'E');
    }

    #[test]
    fn test_next() {
        let source = String::from("LET foobar = 123");
        let lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.cur_char, 'L');
    }

    #[test]
    fn test_token() {
        let source = String::from("> */");
        let mut lexer = lexer::Lexer::build(source);

        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::GT);
    }

    #[test]
    fn test_whitespace() {
        let source = String::from(" - */");
        let mut lexer = lexer::Lexer::build(source);

        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::MINUS);
    }

    #[test]
    fn test_comment() {
        let source = String::from("# This is a comment!\n - */");
        let mut lexer = lexer::Lexer::build(source);

        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::NEWLINE);
    }

    #[test]
    fn test_string() {
        let source = String::from("\"This is Timothy\" - */");
        let mut lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::STRING);
    }

    #[test]
    fn test_number() {
        let source = String::from("1.90 - */");
        let mut lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::NUMBER);
    }

    #[test]
    fn test_identifier() {
        let source = String::from("foo 1.90");
        let mut lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::IDENT);
    }

    #[test]
    fn test_keywords() {
        let source = String::from("WHILE 1.90");
        let mut lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::WHILE);
    }

    #[test]
    fn test_operators() {
        let source = String::from("= +");
        let mut lexer = lexer::Lexer::build(source);
        assert_eq!(lexer.get_token().unwrap_or(token::Token::default()).kind, enums::TokenKind::EQ);
    }
}


