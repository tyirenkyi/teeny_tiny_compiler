mod lexer;
mod parser;
mod enums;
mod token;
mod emitter;

use std::error::Error;
use std::fs;

pub struct Config {
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 2 {
        return Err("no arguments provided");
    }
    let file_path = args[1].clone();

    Ok(Config { file_path })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let lexer = lexer::Lexer::build(contents);
  let emitter = emitter::Emitter::build(String::from("out.c"));
  let mut parser = parser::Parser::new(lexer, emitter);

  parser.program();
  println!("Compiling completed.");

  Ok(())
}
