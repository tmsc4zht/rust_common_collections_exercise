use std::collections::HashMap;

struct Employee {
  data: HashMap<String, Vec<String>>,
}

impl Employee {
  fn new() -> Employee {
    Employee {
      data: HashMap::new(),
    }
  }

  fn add(&mut self, department: &str, name: &str) {
    let member = self.data.entry(department.into()).or_insert(vec![]);
    member.push(name.into());
    member.sort();
  }

  fn list_member(&self, department: &str) -> Option<&Vec<String>> {
    self.data.get(department)
  }

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn add_test() {
    let mut data = Employee::new();

    data.add("Engineering", "Sally");
    data.add("Sales", "Amir");

    let mut want = HashMap::new();
    want.insert(String::from("Engineering"), vec![String::from("Sally")]);
    want.insert(String::from("Sales"), vec![String::from("Amir")]);

    assert_eq!(data.data, want);
  }

  #[test]
  fn list_member_test() {
    let mut data = Employee::new();

    data.add("Engineering", "Bob");
    data.add("Engineering", "Alice");
    data.add("Sales", "Amir");

    let want = vec![String::from("Alice"), String::from("Bob")];

    assert_eq!(data.list_member("Engineering").unwrap(), &want);
  }
}
