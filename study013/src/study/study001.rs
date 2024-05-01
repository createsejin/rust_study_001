pub mod case001 {
  #[allow(dead_code)]
  struct Point<T> {
    x: T,
    y: T,
  }
  #[allow(dead_code)]
  impl<T> Point<T> {
    fn x<'a>(&'a self) -> &'a T {
      &self.x
    }
    fn y<'a>(&'a self) -> &'a T {
      &self.y
    }
  }

  #[allow(dead_code)]
  impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
      (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
  }

  pub fn _study001() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p2 = Point { x: 5.2, y: 3.2 };
    println!("distance from origin of p2 = {}", p2.distance_from_origin());
  }
}

pub mod case002 {
  #[allow(dead_code)]
  struct Point<X1, Y1> {
    x: X1,
    y: Y1,
  }
  #[allow(dead_code)]
  impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
      Point {
        x: self.x,
        y: other.y,
      }
    }

    fn x<'a>(&'a self) -> &'a X1 {
      &self.x
    }
    fn y<'a>(&'a self) -> &'a Y1 {
      &self.y
    }
  }

  pub fn _study002() {
    let p = Point { x: 5, y: 10.4 };
    let p2 = Point {
      x: String::from("hello"),
      y: 'c',
    };
    let p3 = p.mixup(p2);

    println!("p3.x = {}", p3.x());
    println!("p3.y = {}", p3.y());
  }
}

pub mod case003 {
  pub trait Summary {
    fn summarize(&self) -> String;
  }
}
