use rust_study_001::study;
use std::error::Error;
use std::fs::File;

///home/bae/Projects/rust_study_001/target/debug/rust_study_001
fn main() {
  study::study06::_study009();
}

fn _main_001() -> Result<(), Box<dyn Error>> {
  let _greeting_file = File::open("hello.txt")?;

  Ok(())
}

fn _main_000() {
  study::study09::_study003();
}
