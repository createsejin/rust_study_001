use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  // dbg!(args);

  let query = &args[1];
  let filepath = &args[2];

  println!(r#"Searching for '{}'"#, query);
  println!(r#"In file "{}""#, filepath);
}
