use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;
use rand::Rng;

#[allow(dead_code)]
pub fn study001() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
#[allow(dead_code)]
fn func1() -> Result { Ok(()) }
#[allow(dead_code)]
fn func2() -> IoResult<()> { Ok(()) }

#[allow(dead_code)]
pub fn study002() {
    #[allow(unused_variables)]
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
