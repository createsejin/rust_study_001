use std::fmt::{Debug, Display};

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
impl<X1: Display, Y1: Display> Point3<X1, Y1> {
  fn print(&self) {
    println!("x = {}, y = {}", self.x, self.y);
  }
}
pub fn _study006() {
  let p1 = Point3 { x: 5, y: 10.4 };
  let p2 = Point3 { x: "Hello", y: 'c' };
  let p3 = p1.mixup(p2);
  // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
  p3.print();
}

// Traits: Defining Shared Behavior
use crate::{NewsArticle, Summary, Tweet};

pub fn _study007() {
  let tweet = Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  };
  println!("1 new tweet: {}", tweet.summarize());
  println!();
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
    ),
  };
  println!("New article available! {}", article.summarize());
}
// Traits as Parameters
fn notify1(item: &impl Summary) {
  println!("Breaking new! {}", item.summarize());
}
#[allow(dead_code)]
fn notify2<T: Summary>(item: &T) {
  println!("Breaking new! {}", item.summarize());
}
#[allow(dead_code)]
fn notify3<T: Summary>(item1: &T, item2: &T) {
  println!("Breaking new! {}", item1.summarize());
  println!("Breaking new! {}", item2.summarize());
}
pub fn _study008() {
  let article = NewsArticle {
    headline: String::from("Penguins win the Stanley Cup Championship!"),
    location: String::from("Pittsburgh, PA, USA"),
    author: String::from("Iceburgh"),
    content: String::from(
      "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
    ),
  };
  notify1(&article);
}
// Specifying Multiple Trait Bounds with the + Syntax
#[allow(dead_code)]
fn notify4<T: Summary + Display>(item: &T) {
  println!("Breaking new! {}", item.summarize());
}
// Clearer Trait Bounds with where Clauses
#[allow(dead_code)]
fn some_func<T, U>(t: &T, u: &U) -> i32
where
  T: Display + Clone,
  U: Clone + Debug,
{
  println!("{}", t);
  println!("{:?}", u);
  0
}
#[allow(dead_code)]
fn returns_summarizable() -> impl Summary {
  Tweet {
    username: String::from("horse_ebooks"),
    content: String::from("of course, as you probably already know, people"),
    reply: false,
    retweet: false,
  }
}
// #[allow(dead_code)]
// fn returns_summarizable2(switch: bool) -> impl Summary {
//   if switch {
//     NewsArticle {
//       headline: String::from("Penguins win the Stanley Cup Championship!"),
//       location: String::from("Pittsburgh, PA, USA"),
//       author: String::from("Iceburgh"),
//       content: String::from(
//         "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//       ),
//     }
//   } else {
//     Tweet {
//       username: String::from("horse_ebooks"),
//       content: String::from("of course, as you probably already know, people"),
//       reply: false,
//       retweet: false,
//     }
//   }
// }

#[allow(dead_code)]
struct Pair<T> {
  x: T,
  y: T,
}
#[allow(dead_code)]
impl<T> Pair<T> {
  fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}
#[allow(dead_code)]
impl<T: Display + PartialOrd> Pair<T> {
  fn cmp_display(&self) {
    if self.x > self.y {
      println!("The largest member is x = {}", self.x);
    } else if self.x < self.y {
      println!("The largest member is y = {}", self.y);
    } else {
      println!("x and y is same. x = {}, y = {}", self.x, self.y);
    }
  }
}

// Validating References with Lifetimes
// Preventing Dangling References with Lifetimes

// pub fn _study009() {
//   let r;
//   {
//     let x = 5;
//     r = &x;
//   }
//   println!("r = {}", r);
// }

// Generic Lifetimes in Functions
// fn longest(x: &str, y: &str) -> &str {
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }

// Lifetime Annotation Syntax
#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
pub fn _study009() {
  let string1 = String::from("abcd");
  let string2 = "xyz";

  let result = longest(string1.as_str(), string2);
  println!("The longest string is {}", result);
}
pub fn _study010() {
  let string1 = String::from("long string is long");
  {
    let string2 = String::from("xyz");
    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
  }
}
// pub fn _study011() {
//   let string1 = String::from("long string is long");
//   let result;
//   {
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
//   }
//   println!("The longest string is {}", result);
// }

// Thinking in Terms of Lifetimes
#[allow(dead_code)]
fn longest2<'a>(x: &'a str, _y: &str) -> &'a str {
  x
}
// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//   let result = String::from("really long string");
//   result.as_str()
// }
// Lifetime Annotations in Struct Definitions
#[allow(dead_code)]
struct ImportantExcerpt<'a> {
  part: &'a str,
}
#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> {
  // Lifetime Annotations in Method Definitions
  fn level(&self) -> i32 {
    3
  }
  fn announce_and_return_part(&self, announcement: &str) -> &str {
    println!("Attention please: {}", announcement);
    self.part
  }
}
pub fn _study011() {
  let novel = String::from("Call me Ishmael. Some years ago...");
  let first_sentence = novel.split('.').next().expect("Could not find a '.'");
  let _i = ImportantExcerpt {
    part: first_sentence,
  };
}
// Lifetime Elision
#[allow(dead_code)]
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
#[allow(dead_code)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
  T: Display,
{
  println!("Announcement! {}", ann);
  if x.len() > y.len() {
    x
  } else {
    y
  }
}
