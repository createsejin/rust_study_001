struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height 
    }
    fn print_area(&self) {
        println!(
            "The area of the rectangle is {} square pixels.",
            self.area()
        );
    }
}

#[allow(dead_code)]
pub fn study001() {
    println!("Hello, world! It's study001.rs");
    let rect1 = Rectangle {
        width: 30, height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(25);
    rect1.print_area();
    rect4.print_area();
    if rect1.width() {
        println!("The rectangle has a nonzero width\nit is {}", rect1.width);
    }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}