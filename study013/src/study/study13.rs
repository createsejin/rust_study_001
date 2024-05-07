#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
  width: u32,
  height: u32,
}

pub fn _study007() {
  let mut list = [
    Rectangle {
      width: 10,
      height: 1,
    },
    Rectangle {
      width: 3,
      height: 5,
    },
    Rectangle {
      width: 7,
      height: 12,
    },
  ];

  list.sort_by_key(|r| r.width);
  println!("{list:#?}");
}

pub fn _study008() {
  let mut list = [
    Rectangle {
      width: 10,
      height: 1,
    },
    Rectangle {
      width: 3,
      height: 5,
    },
    Rectangle {
      width: 7,
      height: 12,
    },
  ];

  let mut sort_operations = Vec::new();
  let value = String::from("closure called");

  list.sort_by_key(|r| {
    // sort_operations.push(value); // compile Error! FnMut closure에서는 captured된
    // variable을 closure environment 밖으로 move-out 할 수 없다.
    // 예를 들어서 첫번째 closure call은 가능하다. 하지만 그 이후에는 capture된 value의
    // ownership을 sort_operations이 take한다. 따라서 첫번째 call 이후의 value는 더이상
    // vaild 하지 않다. 따라서 두번째 call은 실패한다.
    sort_operations.push(value.clone());
    // 하지만 이렇게 하면??
    // 이렇게 하면 성공한다. 이유는 value를 clone해서 넣기 때문에 ownership을 take하지
    // 않기 때문이다.
    r.width
  });
  println!("{list:#?}");
  println!("{:#?}", sort_operations);
}

pub fn _study009() {
  let mut list = [
    Rectangle {
      width: 10,
      height: 1,
    },
    Rectangle {
      width: 3,
      height: 5,
    },
    Rectangle {
      width: 7,
      height: 12,
    },
  ];

  let mut num_sort_operations = 0;
  list.sort_by_key(|r| {
    num_sort_operations += 1;
    r.width
  });
  println!("{list:#?}");
  println!("sorted in {num_sort_operations} operations");
}

// Processing a Series of Items with Iterators
pub fn _study010() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  for val in v1_iter {
    println!("Got: {val}");
  }
}

// The Iterator Trait and the next Method
#[test]
pub fn _study011() {
  let v1 = vec![1, 2, 3];
  let mut v1_iter = v1.iter();
  assert_eq!(v1_iter.next(), Some(&1));
  assert_eq!(v1_iter.next(), Some(&2));
  assert_eq!(v1_iter.next(), Some(&3));
  assert_eq!(v1_iter.next(), None);
}

#[test]
fn _study012() {
  let v1 = vec![1, 2, 3];
  let v1_iter = v1.iter();
  let total: i32 = v1_iter.sum(); // take ownership, and v1_iter is no longer valid
                                  // It is a consuming adaptor
  assert_eq!(total, 6);
}

pub fn _study013() {
  let v1 = vec![1, 2, 3];
  // the map is iterator adaptor, don't consuming iterator
  let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  assert_eq!(v2, vec![2, 3, 4]);
  for var in v2.iter() {
    println!("{var}");
  }
}

// Using Closures that Capture Their Environment
#[derive(PartialEq, Debug)]
struct Shoe {
  size: u32,
  style: String,
}

#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
  //                    |-> take ownership
  shoes.into_iter().filter(|s| s.size == shoe_size).collect()
  //    |                  |-> only contains elements if which return true
  //    |-> take ownership of Vec
}

#[test]
fn _study014() {
  let shoes = vec![
    Shoe {
      size: 10,
      style: String::from("sneaker"),
    },
    Shoe {
      size: 13,
      style: String::from("sandal"),
    },
    Shoe {
      size: 10,
      style: String::from("boot"),
    },
  ];

  let in_my_size = shoes_in_size(shoes, 10);

  assert_eq!(
    in_my_size,
    vec![
      Shoe {
        size: 10,
        style: String::from("sneaker"),
      },
      Shoe {
        size: 10,
        style: String::from("boot"),
      },
    ]
  )
}
