use core::panic;
use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();
  // dbg!(args);

  // let config = Config::new(&args);
  let config = Config::build(&args).unwrap();

  println!(r#"Searching for '{}'"#, config.query);
  println!(r#"In file "{}""#, config.file_path);

  let contents = fs::read_to_string(config.file_path)
    .expect("Should have been able to read the file");

  println!("With text:\n{contents}");
}

struct Config {
  query: String,
  file_path: String,
}
impl Config {
  #[allow(dead_code)]
  fn new(args: &[String]) -> Self {
    if args.len() != 3 {
      panic!("arguemnts number must be 2!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Self { query, file_path }
  }

  fn build(args: &[String]) -> Result<Self, &'static str> {
    if args.len() != 3 {
      return Err("arguemnts number must be 2!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Self { query, file_path })
  }
}

// fn parse_config(args: &[String]) -> Config {
//   let query = args[1].clone();
//   let file_path = args[2].clone();
//
//   Config { query, file_path }
// }
//
