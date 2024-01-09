use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

pub struct Emitter {
  full_path: String,
  header: String,
  code: String
}

impl Emitter {
  pub fn build(full_path: String) -> Emitter {
    let emitter = Emitter {
      full_path,
      header: String::from(""),
      code: String::from("")
    };

    emitter
  }

  pub fn emit(&mut self, code: String) {
    self.code += &code;
  }

  pub fn emit_line(&mut self, code: String) {
    let new_code = format!("{}{}{}", self.code, code, '\n');
    self.code = new_code;
  }

  pub fn header_line(&mut self, code: String) {
    let new_header = format!("{}{}{}", self.header, code, '\n');
    self.header = new_header;
  }

  pub fn write_file(&mut self) {
    let path = Path::new(&self.full_path);
    let display = Path::display(&path);

    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file,
    };

    match File::write_all(&mut file, (self.header.clone() + self.code.as_str()).as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(..) => println!("successfully wrote to {}", display)
    }
  }
}
