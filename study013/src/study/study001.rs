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

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  impl Summary for NewsArticle {
    fn summarize(&self) -> String {
      format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
  }

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  pub fn _study003() {
    let tweet = Tweet {
      username: String::from("horse_ebooks"),
      content: String::from("of course, as you probably already know, people"),
      reply: false,
      retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
  }
}

pub mod case004 {
  pub trait Summary {
    fn summarize(&self) -> String {
      String::from("(Read more...)")
    }
  }

  pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
  }
  impl Summary for NewsArticle {}

  pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
  }
  impl Summary for Tweet {
    fn summarize(&self) -> String {
      format!("{}: {}", self.username, self.content)
    }
  }

  pub fn _study004() {
    let article = NewsArticle {
      headline: String::from("Penguins win the Stanley Cup Championship!"),
      location: String::from("Pittsburgh, PA, USA"),
      author: String::from("Iceburgh"),
      content: String::from(
        "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
      ),
    };

    println!("New article available! {}", article.summarize());
  }
}
