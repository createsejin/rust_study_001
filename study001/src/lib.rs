pub mod leetcode;
pub mod study;

pub use crate::study::study08;

#[allow(dead_code)]
mod front_of_house;

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
  fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
  }
  fn cook_order() {}

  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }
  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
  #[derive(Debug)]
  pub enum Appetizer {
    Soup,
    Salad,
  }
}
pub fn eat_at_restaurant_002() {
  let order1 = back_of_house::Appetizer::Soup;
  println!("order1 is a {:?}", order1);
  let order2 = back_of_house::Appetizer::Salad;
  println!("order2 is a {:?}", order2);
}
pub fn eat_at_restaurant_001() {
  let mut meal = back_of_house::Breakfast::summer("Rye");
  let mut my_toast = meal.toast;
  //println!("The first order toast is {}", meal.toast);
  println!("The first order toast is {my_toast}");
  meal.toast = String::from("Wheat");
  my_toast = meal.toast;
  println!("I'd like {my_toast} toast please.");
}

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant_000() {
  // Absolute path
  //crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  //front_of_house::hosting::add_to_waitlist();

  hosting::add_to_waitlist();
}
mod customer {
  //use crate::hosting;
  use super::front_of_house::hosting;
  //use crate::front_of_house::hosting;
  #[allow(dead_code)]
  pub fn eat_at_restaurant_003() {
    hosting::add_to_waitlist();
  }
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
//   }
// }

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
// impl Summary for NewsArticle {
//   fn summarize(&self) -> String {
//     format!("{}, by {} ({})", self.headline, self.author, self.location)
//   }
// }
impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!("{}", self.content)
  }
  fn summarize_author(&self) -> String {
    format!("@{}", self.author)
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

// Writing Automated Tests
#[cfg(test)]
mod tests {

  use super::study::study11::*;
  #[test]
  fn exploration() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }

  // #[test]
  // fn another() {
  //   panic!("Make this test fail");
  // }

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 10,
      height: 6,
    };
    let smaller = Rectangle {
      width: 8,
      height: 5,
    };
    assert!(larger.can_hold(&smaller));
  }
}
