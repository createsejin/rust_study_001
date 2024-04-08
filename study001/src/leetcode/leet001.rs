struct Solution;

impl Solution {
  pub fn length_of_last_word(s: &str) -> i32 {
    let v: Vec<&str> = s.rsplit(' ').collect();
    let mut words: Vec<&str> = Vec::new();
    for item in &v {
      if *item == "" {
        continue;
      } else {
        words.push(item);
      }
    }
    println!("first word = {}", words[0]);
    words[0].len().try_into().unwrap()
  }
}

fn helper_for_length(s: &str, num: u32) {
  println!("test str = {}", s);
  let result = Solution::length_of_last_word(s);
  println!("result{1:02} = {0}", result, num);
}

pub fn test001() {
  helper_for_length(&"Hello world", 1);
  helper_for_length(&"My name is Sejin, BTW", 2);
  helper_for_length(&"What your name? SeHee?", 3);
  helper_for_length(&"Whatwhat?", 4);
  helper_for_length(&"   fly me   to   the moon  ", 5);
}
