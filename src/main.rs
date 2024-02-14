#[allow(dead_code)]
fn main_000() {
    println!("Hello, world!");
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
} 
#[allow(dead_code)]
fn main_001() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn change(some_string: &mut String) {
    some_string.push_str(", World");
}

mod study;

#[allow(dead_code)]
fn main_002() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, and {}", r1, r2);
    
    let r3 = &mut s;
    println!("{}", r3);
} 

fn main() {
    study::study06::study004();
}





