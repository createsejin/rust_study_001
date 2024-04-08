use std::{
  collections::VecDeque,
  io::{self, Write},
};

struct Solution {}

impl Solution {
  pub fn count_students(
    &self,
    students: &mut VecDeque<i32>,
    sandwiches: &mut VecDeque<i32>,
  ) -> i32 {
    assert_eq!(students.len(), sandwiches.len());
    println!("array size is equal");

    // self.confirm_pref(students, sandwiches);
    0
  }

  #[allow(dead_code)]
  fn confirm_pref(&self, students: &mut VecDeque<i32>, sandwiches: &mut VecDeque<i32>) {
    let front_student = students.pop_front().unwrap();
    let top_sandwich = sandwiches.pop_front().unwrap();
    if front_student == top_sandwich {
      println!(
        "Front student takes the top sandwich and leaves the line making \
      students = {:?} and sandwiches = {:?}",
        students, sandwiches
      );
    } else {
      println!(
        "Front student leaves the top sandwich and returns to the end \
      of the line making students = {:?}",
        students
      );
    }
  }
}

pub fn test001() {
  let input = "students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]";
  let students_str = "students = [";
  let sandwiche_str = " sandwiches = [";
  let parts: Vec<&str> = input.split(',').collect();

  let mut sandwiche_index: usize = 0;

  for (index, line) in parts.iter().enumerate() {
    let line = *line;
    if line.find("sandwiches") != None {
      sandwiche_index = index;
    }
  }

  let students_vec = remove_extra_chars(&parts[0..sandwiche_index], students_str);
  let students_vec = convert_str_vec_to_i32_vec_deque(students_vec);
  println!("{:?}", students_vec);
  let sandwiches_vec = remove_extra_chars(&parts[sandwiche_index..], sandwiche_str);
  let sandwiches_vec = convert_str_vec_to_i32_vec_deque(sandwiches_vec);
  println!("{:?}", sandwiches_vec);
}

pub fn test002() {
  println!("input students array and sandwiches array like follow:");
  println!("students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]");
  // students = [1,1,1,0,1], sandwiches = [1,0,0,1,1]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let mut students_vec_deque = res.0;
  let mut sandwiches_vec_deque = res.1;
  println!("students_vec_deque = {:?}", students_vec_deque);
  println!("sandwiches_vec_deque = {:?}", sandwiches_vec_deque);
  // check two array size is same
  assert_eq!(students_vec_deque.len(), sandwiches_vec_deque.len());
  println!("array size assertion passed.");

  let front_student = students_vec_deque.pop_front().unwrap();
  let top_sandwich = sandwiches_vec_deque.pop_front().unwrap();
  if front_student == top_sandwich {
    println!(
      "Front student takes the top sandwich and leaves the line making \
      students = {:?} and sandwiches = {:?}",
      students_vec_deque, sandwiches_vec_deque
    );
  } else {
    println!(
      "Front student leaves the top sandwich and returns to the end \
      of the line making students = {:?}",
      students_vec_deque
    );
  }
}

pub fn test003() {
  println!("input students array and sandwiches array like follow:");
  println!("students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]");
  // students = [1,1,1,0,1], sandwiches = [1,0,0,1,1]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let mut students_vec_deque = res.0;
  let mut sandwiches_vec_deque = res.1;
  println!("students_vec_deque = {:?}", students_vec_deque);
  println!("sandwiches_vec_deque = {:?}", sandwiches_vec_deque);
  let solution = Solution {};
  solution.count_students(&mut students_vec_deque, &mut sandwiches_vec_deque);
}

fn parsing_input(input: String) -> (VecDeque<i32>, VecDeque<i32>) {
  let students_str = "students = [";
  let sandwiche_str = " sandwiches = [";
  let parts: Vec<&str> = input.split(',').collect();

  let mut sandwiche_index: usize = 0;

  for (index, line) in parts.iter().enumerate() {
    let line = *line;
    if line.find("sandwiches") != None {
      sandwiche_index = index;
    }
  }
  let students_vec = remove_extra_chars(&parts[0..sandwiche_index], students_str);
  let students_vec = convert_str_vec_to_i32_vec_deque(students_vec);
  let sandwiches_vec = remove_extra_chars(&parts[sandwiche_index..], sandwiche_str);
  let sandwiches_vec = convert_str_vec_to_i32_vec_deque(sandwiches_vec);
  (students_vec, sandwiches_vec)
}

fn get_input(prompt: String) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

fn convert_str_vec_to_i32_vec_deque(input: Vec<String>) -> VecDeque<i32> {
  let mut output: VecDeque<i32> = VecDeque::new();
  for str in input {
    output.push_back(str.as_str().parse().unwrap());
  }
  output
}

fn remove_extra_chars(part_vec: &[&str], extra: &str) -> Vec<String> {
  let last_num = part_vec.last().unwrap().replace(']', "");
  let mut part_vec: Vec<String> = part_vec.iter().map(|x| x.to_string()).collect();
  part_vec[0] = part_vec[0].replace(extra, "");
  let part_vec_size = part_vec.len();
  part_vec[part_vec_size - 1] = last_num.to_string();
  part_vec
}
