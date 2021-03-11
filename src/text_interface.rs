use crate::employee::Employee;
use std::io::{self, stdout, Write};

#[derive(Debug, PartialEq)]
enum Instruction {
  Add(String, String),
  List(String),
  ListAll,
  Exit,
}

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
fn parse_instruction(input: &str) -> Result<Instruction, &'static str> {
  if input == "exit" {
    Ok(Instruction::Exit)
  } else {
    Err("not implemented")
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_instruction() {
    let input = "exit";
    let result = parse_instruction(input).unwrap();

    assert_eq!(result, Instruction::Exit);

    let input = "list";
    let result = parse_instruction(input).unwrap();

    assert_eq!(result, Instruction::ListAll);

    let input = "Add Sally to Engineering";
    let result = parse_instruction(input).unwrap();

    assert_eq!(
      result,
      Instruction::Add("Engineering".into(), "Sally".into())
    );

    let input = "List Engineering";
    let result = parse_instruction(input).unwrap();

    assert_eq!(result, Instruction::List("Engineering".into()));
  }
}
