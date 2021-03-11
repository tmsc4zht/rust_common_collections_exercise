use std::collections::HashMap;

pub mod employee;
pub mod text_interface;

pub fn mean(input: &[i32]) -> f64 {
  if input.len() == 0 {
    panic!("input lenght is zero")
  }
  let mut sum = 0.0;
  for x in input {
    sum += *x as f64;
  }
  return sum / input.len() as f64;
}

pub fn median(input: &[i32]) -> f64 {
  if input.len() == 0 {
    panic!("input lenght is zero")
  }

  let mut sorted = vec![];
  sorted.extend_from_slice(input);

  if input.len() % 2 == 0 {
    let idx = input.len() / 2;
    (sorted[idx - 1] + sorted[idx]) as f64 / 2.0
  } else {
    let idx = input.len() / 2;
    sorted[idx] as f64
  }
}

pub fn mode(input: &[i32]) -> Vec<i32> {
  if input.len() == 0 {
    panic!("input lenght is zero")
  }
  let mut freq = HashMap::new();
  for x in input {
    let count = freq.entry(x).or_insert(0);
    *count += 1;
  }
  let mut max = 0;
  let mut result = Vec::new();
  for (key, val) in freq {
    if val > max {
      result.clear();
      result.push(*key);
      max = val;
    } else if val == max {
      result.push(*key)
    }
  }
  result.sort_unstable();
  return result;
}

pub fn pig_latin(input: &str) -> String {
  let mut chars = input.chars();
  let first_char = chars.next();

  if let None = first_char {
    return String::new();
  };

  let first_char = first_char.unwrap();

  match first_char {
    'a' | 'i' | 'u' | 'e' | 'o' => format!("{}hay", input),
    _ => {
      let s: String = chars.collect();
      format!("{}{}ay", s, first_char)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn mean_test() {
    let input = vec![1, 2, 3];
    let result = mean(&input);
    assert_eq!(result, 2.0);

    let input = vec![0];
    let result = mean(&input);
    assert_eq!(result, 0.0);
  }

  #[test]
  #[should_panic]
  fn mean_panic_test() {
    let input = vec![];
    mean(&input);
  }

  #[test]
  fn median_test() {
    let input = vec![1, 2, 3];
    let result = median(&input);
    assert_eq!(result, 2.0);

    let input = vec![1, 1, 2, 2];
    let result = median(&input);
    assert_eq!(result, 1.5);

    let input = vec![4, 3, 3];
    let result = median(&input);
    assert_eq!(result, 3.0);
  }

  #[test]
  #[should_panic]
  fn median_panic_test() {
    let input = vec![];
    median(&input);
  }

  #[test]
  fn mode_test() {
    let input = vec![1, 1, 2];
    let result = mode(&input);
    assert_eq!(result, vec![1]);

    let input = vec![1, 2, 2];
    let result = mode(&input);
    assert_eq!(result, vec![2]);

    let input = vec![1, 1, 2, 2];
    let result = mode(&input);
    assert_eq!(result, vec![1, 2]);
  }

  #[test]
  #[should_panic]
  fn mode_panic_test() {
    let input = vec![];
    mode(&input);
  }

  #[test]
  fn pig_latin_test() {
    let input = "";
    let result = pig_latin(input);
    assert_eq!(result, "");

    let input = "first";
    let result = pig_latin(input);
    assert_eq!(result, "irstfay");

    let input = "know";
    let result = pig_latin(input);
    assert_eq!(result, "nowkay");

    let input = "apple";
    let result = pig_latin(input);
    assert_eq!(result, "applehay")
  }
}
