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

pub mod case002 {
  use std::cell::RefCell;
  use std::rc::Rc;
  //@#ca2

  pub trait Messenger {
    fn send(&self, msg: &str);
  }

  pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
  }

  impl<'a, T> LimitTracker<'a, T>
  where
    T: Messenger,
  {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
      LimitTracker {
        messenger,
        value: 0,
        max,
      }
    }

    pub fn set_value(&mut self, value: usize) {
      self.value = value;
      let percentage_of_max = self.value as f64 / self.max as f64;
      if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
        self
          .messenger
          .send("Urgent warning: You've used up over 90% of your quota!");
      } else if percentage_of_max >= 0.75 {
        self
          .messenger
          .send("Warning: You've used up over 75% of your quota!");
      }
    }
  }

  #[derive(Debug)]
  pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
  }
  //@#st1
}

pub mod case003 {
  //@#ca3
  use std::cell::RefCell;
  use std::rc::Rc;

  #[derive(Debug)]
  pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
  }
}

#[cfg(test)]
mod study15_case001 {
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
    //@#ref1
    use List2::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));
  }

  #[test]
  fn study010() {
    use List2::{Cons, Nil};
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

#[cfg(test)]
mod study15_case002 {
  use super::case002::{LimitTracker, List, Messenger};
  use std::cell::RefCell;
  use std::rc::Rc;
  use List::{Cons, Nil};
  //@#ca2.te

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, msg: &str) {
      self.sent_messages.borrow_mut().push(String::from(msg));
    }

    // violation of borrowing rules and it should be panicked at runtime
    //
    // fn send(&self, msg: &str) {
    //   let mut one_borrow = self.sent_messages.borrow_mut();
    //   let mut two_borrow = self.sent_messages.borrow_mut();
    //
    //   one_borrow.push(String::from(msg));
    //   two_borrow.push(String::from(msg));
    // }
  }

  #[test]
  fn study011() {
    // it_sends_an_over_75_percent_warning_message
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);

    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }

  #[test]
  fn study012() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(
      Rc::clone(&value),
      Rc::new(Cons(Rc::new(RefCell::new(10)), Rc::new(Nil))),
    ));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
  }
}

#[cfg(test)]
mod study15_case003 {
  use super::case003::List;
  use std::cell::RefCell;
  use std::rc::Rc;
  use List::{Cons, Nil};
  //@#ca3.te

  #[test]
  fn study013() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Rc::new(Cons(Rc::new(RefCell::new(3)), Rc::clone(&a)));
    let c = Rc::new(Cons(Rc::new(RefCell::new(4)), Rc::clone(&a)));

    *value.borrow_mut() += 10;

    println!("a = {:#?}", a);
    println!("b = {:#?}", b);
    println!("c = {:#?}", c);
  }
  //@#st2
}
