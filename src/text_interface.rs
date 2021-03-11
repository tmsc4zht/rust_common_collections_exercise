use crate::employee::Employee;

struct TextInterface {
  employee: Employee,
}

impl TextInterface {
  pub fn new() -> TextInterface {
    TextInterface {
      employee: Employee::new(),
    }
  }
}
