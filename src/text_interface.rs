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

      let input = input.trim();

      match parse_instruction(&input) {
        Ok(instruction) => match instruction {
          Instruction::Add(department, member) => (),
          Instruction::List(department) => (),
          Instruction::ListAll => (),
          Instruction::Exit => {
            println!("exit program.");
            break;
          }
        },
        Err(msg) => {
          eprintln!("{}", msg)
        }
      }
    }
  }
}

fn parse_instruction(input: &str) -> Result<Instruction, &'static str> {
  if input == "exit" {
    return Ok(Instruction::Exit);
  };

  if input == "Listall" {
    return Ok(Instruction::ListAll);
  }

  let mut tokens = input.split_ascii_whitespace();

  let first = tokens.next();
  if let None = first {
    return Err("no instruciton");
  };

  let first = first.unwrap();

  if first == "Add" {
    let member = tokens.next();
    if let None = member {
      return Err("`Add` instruction needs member as a second word");
    }
    let member = member.unwrap();
    let to = tokens.next();
    if let None = to {
      return Err("`Add` instruction needs `to` as a third word");
    }
    if to.unwrap() != "to" {
      return Err("`Add` instruction needs `to` as a third word");
    }
    let department = tokens.next();
    if let None = department {
      return Err("`Add` instruction needs department as a fourth word");
    }
    let department = department.unwrap();
    return Ok(Instruction::Add(department.to_owned(), member.to_owned()));
  }

  if first == "List" {
    let department = tokens.next();
    if let None = department {
      return Err("`List` instruction needs department as a second word");
    }
    let department = department.unwrap();
    return Ok(Instruction::List(department.to_owned()));
  }

  return Err("unknwon instraction");
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_instruction() {
    let input = "exit";
    let result = parse_instruction(input).unwrap();

    assert_eq!(result, Instruction::Exit);

    let input = "Listall";
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
