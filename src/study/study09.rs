use core::panic;
use std::fs::File;

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
