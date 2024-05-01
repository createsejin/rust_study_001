use std::fmt::Debug;
use std::fmt::Display;

pub mod case001 {
  #[allow(dead_code)]
  struct Point<T> {
    x: T,
    y: T,
  }
  #[allow(dead_code)]
  impl<T> Point<T> {
    fn x<'a>(&'a self) -> &'a T {
      &self.x
    }
    fn y<'a>(&'a self) -> &'a T {
      &self.y
    }
  }

  #[allow(dead_code)]
  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }

  pub fn _study001() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p2 = Point { x: 5.2, y: 3.2 };
    println!("distance from origin of p2 = {}", p2.distance_from_origin());
  }
}

pub mod case002 {
  #[allow(dead_code)]
  struct Point<X1, Y1> {
    x: X1,
    y: Y1,
  }
  #[allow(dead_code)]
  impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
      Point {
        x: self.x,
        y: other.y,
      }
    }

    fn x<'a>(&'a self) -> &'a X1 {
      &self.x
    }
    fn y<'a>(&'a self) -> &'a Y1 {
      &self.y
    }
  }

  pub fn _study002() {
    let p = Point { x: 5, y: 10.4 };
    let p2 = Point {
      x: String::from("hello"),
      y: 'c',
    };
    let p3 = p.mixup(p2);

    println!("p3.x = {}", p3.x());
    println!("p3.y = {}", p3.y());
  }
}

pub mod case003 {
  pub trait Summary {
    fn summarize(&self) -> String;
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  pub fn _study003() {
    let tweet = Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
  }
}

pub mod case004 {
  pub trait Summary {
    fn summarize(&self) -> String {
      String::from("(Read more...)")
    }
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  impl Summary for NewsArticle {}

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  pub fn _study004() {
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
}

pub mod case005 {
  pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
      format!("(Read more from {}...)", self.summarize_author())
    }
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
      format!("{}", self.author)
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  impl Summary for Tweet {
    fn summarize_author(&self) -> String {
      format!("@{}", self.username)
    }
  }

  pub fn _study005() {
    let tweet = Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
  }

  #[allow(dead_code)]
  pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
  }

  #[allow(dead_code)]
  pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news1! {}", item1.summarize());
    println!("Breaking news2! {}", item2.summarize());
  }
}

pub mod case006 {

  use super::case005::Summary;
  use super::case005::Tweet;
  use super::*;

  #[allow(dead_code)]
  pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
  }

  #[allow(dead_code)]
  fn some_func<T, U>(_t: &T, _u: &U) -> i32
  where
    T: Display + Clone,
    U: Clone + Debug,
  {
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
}

pub mod case007 {
  use super::*;

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
      if self.x >= self.y {
        println!("The largest member is x = {}", self.x);
      } else {
        println!("The largest member is y = {}", self.y);
      }
    }
  }
}
