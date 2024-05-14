use std::ops::Deref;
use std::rc::Rc;

pub mod case001 {
  use super::*;

  #[allow(dead_code)]
  pub fn study001() {
    println!("call study001");
  }

  #[allow(dead_code)]
  pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
  }

  #[allow(dead_code)]
  //@#st1
  pub enum List2<T> {
    Cons(T, Rc<List2<T>>),
    Nil,
  }

  pub struct MyBox<T>(T);
  #[allow(dead_code)]
  impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
      MyBox(x)
    }
  }
  impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
      &self.0
    }
  }

  #[allow(dead_code)]
  pub fn hello(name: &str) {
    println!("Hello, {name}!");
  }

  pub struct CustomSmartPointer {
    pub data: String,
  }

  // Dropping a Value Early with std::mem::drop
  impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
      println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
  }
}

#[cfg(test)]
mod tests_study15 {
  use super::case001::{self, *};
  use case001::List;
  use std::rc::Rc;

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

  #[test]
  fn study007() {
    let _c = CustomSmartPointer {
      data: String::from("my stuff"),
    };
    println!("CustomSmartPointers `{}` created.", _c.data);
    let _d = CustomSmartPointer {
      data: String::from("other stuff"),
    };
    println!("CustomSmartPointers `{}` created.", _d.data);
  }

  #[test]
  fn study008() {
    let c = CustomSmartPointer {
      data: String::from("my stuff"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
  }

  #[test]
  fn study009() {
    use List2::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
  }

  #[test]
  fn study010() {
    use List2::{Cons, Nil};
    //@#st2
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a: {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b: {}", Rc::strong_count(&a));
    {
      let _c = Cons(4, Rc::clone(&a));
      println!("count after creating c: {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope: {}", Rc::strong_count(&a));
  }
}
