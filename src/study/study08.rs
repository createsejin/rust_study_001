pub use crate::study::study08;

pub fn _study001() {
    #[allow(unused_variables)]
    let mut v = Vec::new();
    v.push(5); v.push(6); v.push(7); v.push(8);

    #[allow(unused_variables)]
    let v2 = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v2[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element.")
    }

    //let dose_not_exist = &v[100];
    let _dose_not_exist = v.get(100);
}

pub fn _study002() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];
    //v.push(6); // panic!
    println!("The first element is: {first}");
} 

pub fn _study003() {
    let v = vec![100, 32, 57];
    for i in &v {
        print!("{i} ");
    } println!();
}

pub fn _study004() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        print!("{i} ");
    } println!();
}

#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn _study005() {
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

// 8.2.
pub fn _study006() {
    let mut _s = String::new();
    
    let data = "initial contents";
    let _s = data.to_string();
    let _s = "initial contents2".to_string();
    let _s = String::from("initial contents3");
    let hello = String::from("السلام عليكم");
    println!("{hello}");
    let hello = String::from("Dobrý den");
    println!("{hello}");
    let hello = String::from("Hello");
    println!("{hello}");
    let hello = String::from("שָׁלוֹם");
    println!("{hello}");
    let hello = String::from("नमस्ते");
    println!("{hello}");
    let hello = String::from("こんにちは");
    println!("{hello}");
    let hello = String::from("안녕하세요");
    println!("{hello}");
    let hello = String::from("你好");
    println!("{hello}");
    let hello = String::from("Olá");
    println!("{hello}");
    let hello = String::from("Здравствуйте");
    println!("{hello}");
    let hello = String::from("Hola");
    println!("{hello}");
}

pub fn _study007() {
    let mut s = String::from("foo");
    s.push_str(" bar");
    let s2 = " bar2";
    s.push_str(s2);
    s.push('!');
    println!("{s}");
    
    let s3 = String::from(" world!");    
    let s4 = s + &s3;
    println!("{s4}");
    //println!("{}", s);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    //let s = s1 + "-" + &s2 + "-" + &s3;
    //println!("{s}");
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    
    //let h = s[0];
}

pub fn _study008() {
    let hello = String::from("Здравствуйте");
    let s = &hello[0..4];
    println!("{s}");
    for c in hello.chars() {
        println!("{}", c);
    }
    let hello_kor = String::from("안녕하시오.");
    for c in hello_kor.chars() {
        println!("{}", c);
    } println!();

    let size_of_kor = hello_kor.len();
    println!("the hello_kor bytes len = {}", size_of_kor);
    let mut i = 0;
    print!("[ ");
    for b in hello_kor.bytes() {
        if i % 3 == 0 { print!("(") }
        if i == size_of_kor - 1 { print!("{b}") }
        else { 
            if i % 3 == 2 { print!("{b}), ")}
            else { print!("{b}, "); }            
        } // 하지만 이것은 완벽한 방법은 아니다.
        i += 1;
    } println!(" ]");
}

// rust에서는 이런 Debug annotation을 이용해서 열거형의 이름을 출력할 수 있다!
// struct도 가능하다. 다만, method 같은건 안되는 모양이다.
#[derive(Debug)] #[allow(dead_code)]
enum UsState {
    Alabame,
    Alaska,
    LA,    
}
pub fn _study009() {
    let state1 = UsState::Alabame;
    println!("state1 = {:?}", state1);
}

// 8.3. 부터
use std::collections::HashMap;

pub fn _study010() {
    let mut scores = HashMap::new();
    let blue_team = String::from("blue");
    let yellow_team = String::from("yellow");
    scores.insert(&blue_team, 10);
    scores.insert(&blue_team, 24);
    scores.insert(&yellow_team, 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{team_name} team score : {score}\n");

    for (key, value) in &scores {
        println!("{key} team score : {value}");
    }
}
pub fn _study011() { // Overwriting a Value
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 23);
    scores.insert(String::from("Blue"), 21);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in &scores {
        println!("{key} team score : {value}");
    }
}
pub fn _study012() {
    
}