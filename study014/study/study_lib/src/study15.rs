use std::ops::Deref;

pub fn study001() {
  println!("call study001");
}

#[allow(dead_code)]
enum List<T> {
  Cons(T, Box<List<T>>),
  Nil,
}

struct MyBox<T>(T);
#[allow(dead_code)]
impl<T> MyBox<T> {
  fn new(x: T) -> MyBox<T> {
    MyBox(x)
  }
}
impl<T> Deref for MyBox<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

pub fn hello(name: &str) {
  println!("Hello, {name}!");
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

  #[test]
  fn study004() {
    let x = 5;
    let mut y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    *y += 2;
    assert_eq!(5, x);
    assert_eq!(7, *y);
  }

  #[test]
  fn study005() {
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, 5);
    assert_eq!(*y, 5);
  }

  #[test]
  fn study006() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // The code we would have to write if Rust didn’t have deref coercion
    hello(&(*m)[..]);
  }
}
