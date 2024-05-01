mod company;
mod study;
use company::ShirtColor;

fn main() {
  study::study001::case005::_study005();
}

#[allow(dead_code)]
fn main_001() {
  let store = company::Inventory {
    shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
  };

  let user_pref1 = Some(ShirtColor::Red);
  company::user_helper(&user_pref1, &store);

  let user_pref2 = None;
  company::user_helper(&user_pref2, &store);
}
