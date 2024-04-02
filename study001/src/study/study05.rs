struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn width(&self) -> bool {
    self.width > 0
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width >= other.width && self.height >= other.height
  }
  fn print_area(&self) {
    println!(
      "The area of the rectangle is {} square pixels.",
      self.area()
    );
  }
}

#[allow(dead_code)]
pub fn study001() {
  println!("Hello, world! It's study001.rs");
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 40,
  };
  let rect3 = Rectangle {
    width: 60,
    height: 45,
  };
  let rect4 = Rectangle::square(25);
  rect1.print_area();
  rect4.print_area();
  if rect1.width() {
    println!("The rectangle has a nonzero width\nit is {}", rect1.width);
  }
  println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
  println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

#[allow(dead_code)]
fn main_000() {
  println!("Hello, world!");
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is {}.", s1, len);
}
#[allow(dead_code)]
fn main_001() {
  let mut s = String::from("hello");
  change(&mut s);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}
fn change(some_string: &mut String) {
  some_string.push_str(", World");
}

#[allow(dead_code)]
fn main_002() {
  let mut s = String::from("hello");

  let r1 = &s;
  let r2 = &s;
  println!("{}, and {}", r1, r2);

  let r3 = &mut s;
  println!("{}", r3);
}

pub fn study002() {
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  println!("{}", r1);
  let r2 = &mut s;
  println!("{}", r2);
}
