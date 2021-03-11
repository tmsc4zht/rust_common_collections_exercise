use crate::employee::Employee;
use std::io::{self, stdout, Write};

pub struct TextInterface {
  employee: Employee,
}

impl TextInterface {
  pub fn new() -> TextInterface {
    TextInterface {
      employee: Employee::new(),
    }
  }

  pub fn run(&mut self) {
    println!("Employee management interface.");

    loop {
      println!("Please input instruction");
      print!("> ");
      stdout().flush().unwrap();

      let mut input = String::new();

      io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

      println!("{}", input)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
}
