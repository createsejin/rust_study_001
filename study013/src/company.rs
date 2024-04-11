use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
  Red,
  Blue,
}

pub struct Inventory {
  pub shirts: Vec<ShirtColor>,
}

impl Inventory {
  //@#giv
  pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    // if some user_preference exist, give that shirt, but without,
    // give most_stocked.
    // if Inventory store doesn't have user_preference shirt,
    // give most_stocked.
    user_preference.unwrap_or_else(|| self.most_stocked())
  }

  fn most_stocked(&self) -> ShirtColor {
    let mut num_red = 0;
    let mut num_blue = 0;

    for color in &self.shirts {
      match color {
        ShirtColor::Red => num_red += 1,
        ShirtColor::Blue => num_blue += 1,
      }
    }
    if num_red > num_blue {
      ShirtColor::Red
    } else {
      ShirtColor::Blue
    }
  }
}
pub fn user_helper(user_pref: &Option<ShirtColor>, store: &Inventory) {
  let giveaway = store.giveaway(*user_pref);
  let user_pref_color: String;
  if let Some(color) = user_pref {
    user_pref_color = format!("{:?}", color);
  } else {
    user_pref_color = String::from("None");
  }
  println!(
    "The user with preference {} gets {:?}",
    user_pref_color, giveaway
  );
}

#[allow(dead_code)]
fn add_one_v1(x: u32) -> u32 {
  x + 1
}
pub fn _study001() {
  let _expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };
  let _add_one_v2 = |x: u32| -> u32 { x + 1 };
  let _add_one_v3 = |x: u32| x + 1;
}

pub fn _study002() {
  let example_closure = |x| x;

  let s = example_closure(String::from("hello"));
  let n = example_closure(5);
}
