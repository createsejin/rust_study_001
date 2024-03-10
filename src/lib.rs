#[allow(dead_code)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        #[allow(dead_code)]
        fn seat_at_table() {}
    }

    #[allow(dead_code)]
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

#[allow(dead_code)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant_002() {
    let order1 = back_of_house::Appetizer::Soup;
    println!("order1 is a {:?}", order1);
    let order2 = back_of_house::Appetizer::Salad;
    println!("order2 is a {:?}", order2);
}
pub fn eat_at_restaurant_001() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    let mut my_toast = meal.toast;
    //println!("The first order toast is {}", meal.toast);
    println!("The first order toast is {my_toast}");
    meal.toast = String::from("Wheat");
    my_toast = meal.toast;
    println!("I'd like {my_toast} toast please.");
}

pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant_000() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
mod customer {
    //use crate::hosting;
    use super::front_of_house::hosting;
    //use crate::front_of_house::hosting;
    #[allow(dead_code)]
    pub fn eat_at_restaurant_003() {
        hosting::add_to_waitlist();
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
