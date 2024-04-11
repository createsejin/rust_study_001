mod company;
use company::ShirtColor;

fn main() {
  let store = company::Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_pref1 = Some(ShirtColor::Red);
  company::user_helper(&user_pref1, &store);

  let user_pref2 = None;
  company::user_helper(&user_pref2, &store);
}
