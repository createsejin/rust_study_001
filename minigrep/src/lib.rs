use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}
impl Config {
  #[allow(dead_code)]
  pub fn new(args: &[String]) -> Self {
    if args.len() != 3 {
      panic!("arguemnts number must be 2!");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Self {
      query,
      file_path,
      ignore_case,
    }
  }

  //@#build
  pub fn build<I>(mut args: I) -> Result<Self, &'static str>
  where
    I: Iterator<Item = String>,
  {
    args.next();
    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };
    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file path"),
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Self {
      query,
      file_path,
      ignore_case,
    })
  }
}

//@#run
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for item in results {
    println!("{item}");
  }
  Ok(())
}

//@#sear.sens
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result: Vec<&'a str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }
  result
}

//@#sear.in
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

  #[test] //@#test.sens
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

  #[test] //@#test.in
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
