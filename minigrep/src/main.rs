use minigrep::Config;
use std::env;
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::build(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguemnts: {err}");
    process::exit(1);
  });

  // println!(r#"Searching for '{}'"#, config.query);
  // println!(r#"In file "{}""#, config.file_path);
  // println!();

  // run(config);

  if let Err(e) = minigrep::run(config) {
    println!("Application error: {e}");
    process::exit(1);
  }
}
