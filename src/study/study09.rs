use core::panic;
use rand::Rng;
use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Write};

// Error Handling
pub fn _study001() {
  panic!("crash and burn");
}
pub fn _study002() {
  let v = vec![1, 2, 3];
  v[99];
}
pub fn _study003() {
  let _greeting_file_result = File::open("hello.txt");

  let _greeting_file = match _greeting_file_result {
    Ok(file) => file,
    Err(error) => panic!("Problem opening the file: {:?}", error),
  };
}
pub fn _study004() {
  let _greeting_file_result = File::open("hello.txt");

  let _greeting_file = match _greeting_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      std::io::ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error);
      }
    },
  };
}
pub fn _study005() {
  let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error);
    }
  });
}
// Shortcuts for Panic on Error: unwrap and expect
pub fn _study006() {
  let _greeting_file = File::open("hello.txt").unwrap();
}
pub fn _study007() {
  let _greeting_file =
    File::open("hello.txt").expect("hello.txt should be included in this project");
}
// Propagating Errors
#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
  let username_file_result = File::open("hello.txt");

  let mut username_file = match username_file_result {
    Ok(file) => file,
    Err(e) => return Err(e),
  };

  let mut username = String::new();

  match username_file.read_to_string(&mut username) {
    Ok(_) => Ok(username),
    Err(e) => Err(e),
  }
}
// A Shortcut for Propagating Errors: the ? Operator
#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
  let mut username_file = File::open("hello.txt")?;
  let mut username = String::new();
  username_file.read_to_string(&mut username)?;
  Ok(username)
}
#[allow(dead_code)] // 9-8
fn read_username_from_file3() -> Result<String, io::Error> {
  let mut username = String::new();
  File::open("hello.txt")?.read_to_string(&mut username)?;

  Ok(username)
}
#[allow(dead_code)] // 9-9
fn read_username_from_file4() -> Result<String, io::Error> {
  fs::read_to_string("hello.txt")
}
#[allow(dead_code)]
fn last_char_of_first_line(text: &str) -> Option<char> {
  text.lines().next()?.chars().last()
}
// String, str type, BTW
pub fn _study008() {
  let mut my_str = String::from("hello world");
  let my_im_ref1 = &my_str;
  let my_im_ref2 = &my_str;
  println!("{}", *my_im_ref1);
  println!("{}", *my_im_ref2);
  println!("{}", *my_im_ref1);
  let my_mut_ref1 = &mut my_str;
  *my_mut_ref1 += &String::from(" My name is");
  println!("{}\n", *my_mut_ref1);

  let mut my_str2 = String::from("My String slice2!");
  let my_str2_mut_ref1 = &mut my_str2;
  my_str2_mut_ref1.push_str("push push!");
  println!("{}", my_str2_mut_ref1);
}
fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
pub fn _study009() {
  let mut s = String::from("hello");
  change(&mut s);
  let ref1 = &mut s[0..1];
  println!("ref1 = {}", ref1);
}
// Creating Custom Types for Validation
pub fn _study010() {
  println!("Guess the number!");
  println!("The secret number is between 1 and 100.");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  loop {
    print!("input your guess> ");
    io::stdout().flush().unwrap();
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
    let guess: i32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };
    if guess < 1 || guess > 100 {
      println!("The secret number will be between 1 and 100.");
      continue;
    }
    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
pub struct Guess {
  value: i32,
}
impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}.", value);
    }
    Guess { value }
  }
  pub fn value(&self) -> i32 {
    self.value
  }
}
