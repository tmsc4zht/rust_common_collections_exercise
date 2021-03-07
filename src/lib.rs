pub fn mean(input: &[i32]) -> f64 {
  0.0
}

pub fn median(input: &[i32]) -> f64 {
  0.0
}

pub fn mode(input: &[i32]) -> Vec<i32> {
}

pub fn pig_latin(input: &str) -> String {
  String::new()
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
    let input = "first";
    let result = pig_latin(input);
    assert_eq!(result, "irstfay");

    let input = "apple";
    let result = pig_latin(input);
    assert_eq!(result, "applehay")
  }
}
