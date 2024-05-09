pub fn study001() {
  println!("call study001");
}

#[allow(dead_code)]
enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

#[cfg(test)]
mod tests_study15 {
  use super::*;

  #[test]
  fn study002() {
    let b = Box::new(5);
    println!("b = {}", b);
  }

  #[test]
  fn study003() {
    use List::{Cons, Nil};
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
  }
}
