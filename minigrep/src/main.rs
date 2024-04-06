use core::panic;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguemnts: {err}");
    process::exit(1);
  });

  println!(r#"Searching for '{}'"#, config.query);
  println!(r#"In file "{}""#, config.file_path);

  // run(config);
  if let Err(e) = run(config) {
    println!("Application error: {e}");
    process::exit(1);
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  println!("With text:\n{contents}");
  Ok(())
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
