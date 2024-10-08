use core::panic;

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

// Writing Automated Tests
#[derive(Debug)]
pub struct Rectangle {
  pub width: u32,
  pub height: u32,
}
impl Rectangle {
  pub fn can_hold(&self, other: &Rectangle) -> bool {
    self.width >= other.width && self.height >= other.height
  }
}

pub fn add_two(a: i32) -> i32 {
  a + 2
}
#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
  a + b
}

pub fn greeting(name: &str) -> String {
  format!("Hello {}!", name)
  // String::from("Hello! {}")
}

// Checking for Panics with should_panic
#[allow(dead_code)]
pub struct Guess {
  value: i32,
}
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!(
        "Guess value must be greater than or equal to 1, got {}.",
        value
      );
    } else if value > 100 {
      panic!(
        "Guess value must be less than or equal to 100, got {}.",
        value
      );
    }
    Guess { value }
  }
}

#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
  println!("I got the value {}", a);
  10
}

#[cfg(test)]
mod test_unit_001 {
  use super::*;

  #[test]
  fn smaller_cannot_hold_larger() {
    let larger = Rectangle {
      width: 10,
      height: 6,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };
    assert!(!smaller.can_hold(&larger));
  }

  #[test]
  fn it_adds_two() {
    assert_eq!(4, add_two(2));
  }

  // Adding Custom Failure Messages
  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was `{}`",
      result
    );
  }

  #[test]
  #[should_panic(expected = "less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200);
  }

  // Using Result<T, E> in Tests
  #[test]
  fn it_works2() -> Result<(), String> {
    if 2 + 2 == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }

  // 11.2. Controlling How Tests Are Run
  // Showing Function Output
  #[test]
  fn this_test_will_pass() {
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
  }
  #[test]
  // cargo test -- --ignored => test ignored only
  // cargo test -- --include-ignored => test all
  #[ignore]
  fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  }

  #[test]
  fn add_two_and_two() {
    assert_eq!(4, add_two(2));
  }
  #[test]
  fn add_three_and_two() {
    assert_eq!(5, add_two(3));
  }
  #[test]
  fn one_hundred() {
    // assert_eq!(102, add_two(100));
    assert_eq!(102, add_two(100));
  }
}
