use std::thread;

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

pub fn _study003() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let only_brrows = || println!("From closure: {:?}", list);

  println!("Before calling closure: {:?}", list);
  only_brrows();
  println!("After calling closure: {:?}", list);
}

pub fn _study004() {
  let mut list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  let mut borrows_mutably = || list.push(7);

  // println!("Before calling closure: {:?}", list); //=> Error!
  // cannot immutable borrow at the same time mutable borrowed value
  borrows_mutably();
  println!("After calling closure: {:?}", list);
}

pub fn _study005() {
  let list = vec![1, 2, 3];
  println!("Before defining closure: {:?}", list);

  // thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();
  thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
}
