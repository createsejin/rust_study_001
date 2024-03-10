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
    
}