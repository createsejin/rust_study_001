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

#[cfg(test)]
mod test_study11 {
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
}
