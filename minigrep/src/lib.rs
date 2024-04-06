use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub file_path: String,
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
    if args.len() < 3 {
      return Err("arguemnts not enough!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Self { query, file_path })
  }
  // pub fn get_query(&self) -> &str {
  //   &self.query
  // }
  // pub fn get_file_path(&self) -> &str {
  //   &self.file_path
  // }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  for item in search(&config.query, &contents) {
    println!("{item}");
  }
  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
doduct do do what?
Duct tape.";
    assert_eq!(
      vec!["safe, fast, productive.", "doduct do do what?"],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
doduct do do what?
Trust me.";
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
