struct Solution;

impl Solution {
  pub fn length_of_last_word(s: &str) -> u32 {
    println!("{}", s);
    0
  }
}

pub fn test001() {
  let test_str01 = String::from("Hello world");
  Solution::length_of_last_word(&test_str01);
}
