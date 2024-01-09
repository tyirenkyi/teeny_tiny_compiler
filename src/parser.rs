use std::collections::HashSet;

use crate::emitter::Emitter;
use crate::lexer::Lexer;
use crate::token::Token;
use crate::enums::TokenKind;


pub struct Parser {
  cur_token: Option<Token>,
  peek_token: Option<Token>,
  lexer: Lexer,
  emitter: Emitter,
  symbols: HashSet<String>,
  labels_gotoed: HashSet<String>,
  labels_declared: HashSet<String>
}

impl Parser {
  pub fn new(lexer: Lexer, emitter: Emitter) -> Parser {

    let mut parser = Parser {
      cur_token: None,
      peek_token: None,
      lexer,
      emitter,
      symbols: HashSet::new(),
      labels_declared: HashSet::new(),
      labels_gotoed: HashSet::new()
    };


    parser.next_token();
    parser.next_token();
    parser
  }

  pub fn check_token(&mut self, kind: TokenKind) -> bool {
    kind == self.cur_token.clone().unwrap_or(Token::default()).kind
  }

  pub fn check_peek(&mut self, kind: TokenKind) -> bool {
    kind == self.peek_token.clone().unwrap_or(Token::default()).kind
  }

  pub fn match_token(&mut self, kind: TokenKind) {
    if !self.check_token(kind) {
      let msg = format!("Expected {:?}, got {:?}", kind, self.cur_token.clone().unwrap_or(Token::default()).kind);
      self.abort(&msg);
    }
  }

  pub fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = Some(self.lexer.get_token().unwrap_or(Token::default()));
  }

  pub fn abort(& self, msg: &str) {
    panic!("{}", msg);
  }

  // program ::= {statement}
  pub fn program(&mut self) {
    self.emitter.header_line(String::from("#include <stdio.h>"));
    self.emitter.header_line(String::from("int main(void){"));

    // Since some newlines are required in our grammar, need to skip the excess.
    while self.check_token(TokenKind::NEWLINE) {
        self.next_token();
    }

    // Parse all the statements in the program.
    while !self.check_token(TokenKind::EOF) {
      self.statement();
    }

    // Wrap things up
    self.emitter.emit_line(String::from("return 0;"));
    self.emitter.emit_line(String::from("}"));

    // Check that each label referenced in a GOTO is declared.
    for label in self.labels_gotoed.iter() {
      if !self.labels_declared.contains(label) {
        let msg = format!("Attempting to GOTO to undeclared label: {}", label);
        self.abort(&msg);
      }
    }

    self.emitter.write_file();
  }

  pub fn statement(&mut self) {
    if self.check_token(TokenKind::PRINT) {
      self.next_token();

      if self.check_token(TokenKind::STRING) {
        self.emitter.emit_line(format!("printf(\"{}\\n\");", self.cur_token.clone().unwrap().text));
        self.next_token();
      } else {
        // Expect an expression.
        self.emitter.emit(String::from("printf(\"%.2f\\n\", (float)("));
        self.expression();
        self.emitter.emit_line(String::from("));"));
      }
    } else if self.check_token(TokenKind::IF) {
      self.next_token();
      self.emitter.emit(String::from("if("));
      self.comparison();

      self.match_token(TokenKind::THEN);
      self.next_token();
      self.nl();
      self.emitter.emit_line(String::from("}{"));
      while !self.check_token(TokenKind::ENDIF) {
        self.statement();
      }
      self.match_token(TokenKind::ENDIF);
      self.emitter.emit_line(String::from("}"));
      self.next_token();
    } else if self.check_token(TokenKind::WHILE) {
      self.next_token();
      self.emitter.emit(String::from("while("));
      self.comparison();

      self.match_token(TokenKind::REPEAT);
      self.next_token();
      self.nl();
      self.emitter.emit_line(String::from("){"));
      while !self.check_token(TokenKind::ENDWHILE) {
        self.statement();
      }
      self.match_token(TokenKind::ENDWHILE);
      self.emitter.emit_line(String::from("}"));
      self.next_token();
    } else if self.check_token(TokenKind::LABEL) {
      println!("STATEMENT-LABEL");
      self.next_token();

      if !self.labels_declared.contains(&self.cur_token.clone().unwrap().text) {
        let msg = format!("Label already exists: {}", self.cur_token.clone().unwrap().text);
        self.abort(&msg);
      }
      self.labels_declared.insert(self.cur_token.clone().unwrap().text);
      self.emitter.emit_line(format!("{}:", self.cur_token.clone().unwrap().text));
      self.match_token(TokenKind::IDENT);
      self.next_token();
    } else if self.check_token(TokenKind::GOTO) {
      self.next_token();
      self.labels_gotoed.insert(self.cur_token.clone().unwrap().text);
      self.emitter.emit_line(format!("goto {};", self.cur_token.clone().unwrap().text));
      self.match_token(TokenKind::IDENT);
      self.next_token();

    } else if self.check_token(TokenKind::LET) {
      self.next_token();
      if !self.symbols.contains(&self.cur_token.as_ref().unwrap().text) {
        self.symbols.insert(self.cur_token.clone().unwrap().text);
        self.emitter.header_line(format!("float {};", self.cur_token.clone().unwrap().text));
      }
      self.emitter.emit(format!("{} = ", self.cur_token.clone().unwrap().text));
      self.match_token(TokenKind::IDENT);
      self.next_token();
      self.match_token(TokenKind::EQ);
      self.next_token();
      self.expression();
      self.emitter.emit_line(String::from(";"))
    } else if self.check_token(TokenKind::INPUT) {
      self.next_token();
      if !self.symbols.contains(&self.cur_token.as_ref().unwrap().text) {
        self.symbols.insert(self.cur_token.clone().unwrap().text);
        self.emitter.header_line(format!("float {};", self.cur_token.clone().unwrap().text));
      }
      self.emitter.emit_line(format!("if(0 == scanf(\"%f\", &{})) {{", self.cur_token.clone().unwrap().text));
      self.emitter.emit_line(format!("{} = 0;", self.cur_token.clone().unwrap().text));
      self.emitter.emit(String::from("scanf(\"&"));
      self.emitter.emit_line(String::from("*s\");"));
      self.emitter.emit_line(String::from("}"));
      self.match_token(TokenKind::IDENT);
      self.next_token();
    } else {
      let msg = format!("Invalid statement at {} ({:?})", self.cur_token.clone().unwrap().text, self.cur_token.clone().unwrap().kind);
      self.abort(&msg);
    }

    self.nl()
  }
  
  pub fn nl(&mut self) {
    // Require at least one newline
    self.match_token(TokenKind::NEWLINE);
    while self.check_token(TokenKind::NEWLINE) {
      self.next_token();
    }
  }
  
  pub fn expression(&mut self) {
    self.term();

    while self.check_token(TokenKind::PLUS) || self.check_token(TokenKind::MINUS) {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
      self.term();
    }
  }

  pub fn term(&mut self) {
    self.unary();

    while self.check_token(TokenKind::ASTERISK) || self.check_token(TokenKind::SLASH) {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
      self.unary();
    }
  }

  pub fn unary(&mut self) {
    if self.check_token(TokenKind::PLUS) || self.check_token(TokenKind::MINUS) {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
    }

    self.primary();
  }

  pub fn primary(&mut self) {
    if self.check_token(TokenKind::NUMBER) {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
    } else if self.check_token(TokenKind::IDENT) {
      if !self.symbols.contains(&self.cur_token.as_ref().unwrap().text) {
        let msg = format!("Referencing variable before assignment: {}", self.cur_token.as_ref().unwrap().text);
        self.abort(&msg)
      }
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
    } else {
      let msg = format!("Unexpected token at {}", self.cur_token.clone().unwrap().text);
      self.abort(&msg);
    }
  }

  pub fn is_comparison_operator(&mut self) -> bool { 
    self.check_token(TokenKind::GT) || self.check_token(TokenKind::GTEQ) 
      || self.check_token(TokenKind::LT) || self.check_token(TokenKind::LTEQ)
      || self.check_token(TokenKind::EQEQ) || self.check_token(TokenKind::NOTEQ)
  }

  pub fn comparison(&mut self) {
    self.expression();

    if self.is_comparison_operator() {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
      self.expression();
    } else {
      let msg = format!("Expected comparison operator at: {}", self.cur_token.clone().unwrap().text);
      self.abort(&msg);
    }

    while self.is_comparison_operator() {
      self.emitter.emit(format!("{}", self.cur_token.clone().unwrap().text));
      self.next_token();
      self.expression();
    }
  }
}
