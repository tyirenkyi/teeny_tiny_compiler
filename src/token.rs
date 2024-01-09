use crate::enums::TokenKind;

use std::fmt::Display;

#[derive(Copy, Clone)]
struct Keyword<'a>(&'a str, TokenKind);

const KEYWORDS: [Keyword; 11] = [
    Keyword("LABEL", TokenKind::LABEL), 
    Keyword("GOTO", TokenKind::GOTO),
    Keyword("PRINT", TokenKind::PRINT),
    Keyword("INPUT", TokenKind::INPUT),
    Keyword("LET", TokenKind::LET),
    Keyword("IF", TokenKind::IF),
    Keyword("THEN", TokenKind::THEN),
    Keyword("ENDIF", TokenKind::ENDIF),
    Keyword("WHILE", TokenKind::WHILE),
    Keyword("REPEAT", TokenKind::REPEAT),
    Keyword("ENDWHILE", TokenKind::ENDWHILE)
];

#[derive(Clone)]
pub struct Token {
  pub text: String,
  pub kind: TokenKind
}

impl Default for Token {
  fn default() -> Self {
      Self { text: Default::default(), kind: TokenKind::EOF }
  }
}

impl Token {
  pub fn check_if_keyword(token_text: &String) -> Option<TokenKind> {
      let mut index = 0;
      while index < KEYWORDS.len() {
          if token_text == KEYWORDS[index].0 {
              return Some(KEYWORDS[index].clone().1);
          }
          index += 1;
      }
      None
  }
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}, {:?}", self.text, self.kind)
  }
}
