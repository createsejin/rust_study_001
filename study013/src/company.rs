use std::thread;
use std::time::Duration;

#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum ShirtColor {
  Red,
  Blue,
}

#[allow(dead_code)]
pub struct Inventory {
  pub shirts: Vec<ShirtColor>,
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

pub fn _study001() {
  let store = Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_pref1 = Some(ShirtColor::Red);
  user_helper(&user_pref1, &store);

  let user_pref2 = None;
  user_helper(&user_pref2, &store);
}

#[allow(dead_code)]
fn add_one_v1(x: u32) -> u32 {
  x + 1
}

pub fn _study002() {
  let _expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };
  let _add_one_v2 = |x: u32| -> u32 { x + 1 };
  let _add_one_v3 = |x: u32| x + 1;
}

pub fn _study003() {
  let example_closure = |x| x;

  let _s = example_closure(String::from("hello"));
  let _n = example_closure(5.to_string());
}

pub fn _study004() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let only_brrows = || println!("From closure: {:?}", list);

  println!("Before calling closure: {:?}", list);
  only_brrows();
  println!("After calling closure: {:?}", list);
}

pub fn _study005() {
  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7);

  // println!("Before calling closure: {:?}", list); //=> Error!
  // cannot immutable borrow at the same time mutable borrowed value
  borrows_mutably();
  println!("After calling closure: {:?}", list);
}

pub fn _study006() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  // thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
  thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
}
