pub fn _study001() {
  let number_list = vec![34, 50, 25, 100, 65];

  let mut largest = &number_list[0];

  for number in &number_list {
    if number > largest {
      largest = number;
    }
  }
  println!("The largest number is {}", largest);
}
//##lg1
fn largest_i32(list: &[i32]) -> &i32 {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
fn largest_char(list: &[char]) -> &char {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
pub fn _study002() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest_i32(&number_list);
  println!("The largest number is {}", result);

  let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
  let result = largest_i32(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest_char(&char_list);
  println!("The largest char is {}", result);
}
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];
  for item in list {
    if item > largest {
      largest = item;
    }
  }
  largest
}
pub fn _study003() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {}", result);

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {}", result);
}
#[allow(dead_code)]
struct Point<T> {
  x: T,
  y: T,
}
impl<T> Point<T> {
  fn x(&self) -> &T {
    &self.x
  }
}
impl Point<f32> {
  fn distance_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
  }
}
#[allow(dead_code)]
struct Point2<T, U> {
  x: T,
  y: U,
}
pub fn _study004() {
  let _integer = Point { x: 5, y: 10 };
  let _float = Point { x: 1.0, y: 4.0 };
  let _integer_and_float = Point2 { x: 5, y: 4.0 };
}
pub fn _study005() {
  let p = Point { x: 5, y: 10 };
  println!("p.x = {}", p.x());
  let p = Point { x: 1.2, y: 2.3 };
  let dis = p.distance_from_origin();
  println!("The distance of p from origin = {}", dis);
}
#[allow(dead_code)]
struct Point3<X1, Y1> {
  x: X1,
  y: Y1,
}
impl<X1, Y1> Point3<X1, Y1> {
  fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
    Point3 {
      x: self.x,
      y: other.y,
    }
  }
}
pub fn _study006() {
  let p1 = Point3 { x: 5, y: 10.4 };
  let p2 = Point3 { x: "Hello", y: 'c' };
  let p3 = p1.mixup(p2);
  println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
// Traits: Defining Shared Behavior
use crate::{Summary, Tweet};
pub fn _study007() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());
}
