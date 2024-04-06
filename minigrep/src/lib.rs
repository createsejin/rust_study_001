use std::error::Error;
use std::fs;

pub struct Config {
  query: String,
  file_path: String,
}
impl Config {
  #[allow(dead_code)]
  pub fn new(args: &[String]) -> Self {
    if args.len() != 3 {
      panic!("arguemnts number must be 2!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Self { query, file_path }
  }

  pub fn build(args: &[String]) -> Result<Self, &'static str> {
    if args.len() != 3 {
      return Err("arguemnts number must be 2!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Self { query, file_path })
  }

  pub fn get_query(&self) -> &str {
    &self.query
  }
  pub fn get_file_path(&self) -> &str {
    &self.file_path
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  println!("With text:\n{contents}");
  Ok(())
}
