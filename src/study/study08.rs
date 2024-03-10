#[allow(dead_code)]
pub fn study001() {
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
#[allow(dead_code)]
pub fn study002() {
    let mut v = vec![1, 2, 3, 4, 5];
    v.push(6);
    let first = &v[0];
    //v.push(6); // panic!
    println!("The first element is: {first}");
} // 8.1. Iterating over the Values in a Vector 부터