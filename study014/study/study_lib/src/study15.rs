pub fn study001() {
  println!("call study001");
}

#[cfg(test)]
mod tests_study15 {
  // use super::*;

  #[test]
  fn study002() {
    let b = Box::new(5);
    println!("b = {}", b);
  }
}
