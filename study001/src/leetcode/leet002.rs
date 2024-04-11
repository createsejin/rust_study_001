use std::{
  collections::VecDeque,
  io::{self, Write},
};

struct Solution;

impl Solution {
  pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    assert_eq!(students.len(), sandwiches.len());
    let mut students: VecDeque<i32> = students.into_iter().collect();
    let mut sandwiches: VecDeque<i32> = sandwiches.into_iter().collect();

    loop {
      Solution::confirm_pref(&mut students, &mut sandwiches);
      if Solution::check_all_same_pref(&students, &sandwiches) {
        let remain = students.len().try_into().unwrap();
        println!("The remain {} students cannot eat sandwiches!", remain);
        return remain;
      }
    }
  }

  pub fn confirm_pref(students: &mut VecDeque<i32>, sandwiches: &mut VecDeque<i32>) {
    let front_student = students.pop_front().unwrap();
    let top_sandwich = *sandwiches.front().unwrap();
    if front_student == top_sandwich {
      let _ = sandwiches.pop_front().unwrap();
      println!(
        "Front student takes the top sandwich and leaves the line making \
      students = {:?} and sandwiches = {:?}",
        students, sandwiches
      );
    } else {
      students.push_back(front_student);
      println!(
        "Front student leaves the top sandwich and returns to the end \
      of the line making students = {:?} and sandwiches = {:?}",
        students, sandwiches
      );
    }
  }

  pub fn check_all_same_pref(
    students: &VecDeque<i32>,
    sandwiches: &VecDeque<i32>,
  ) -> bool {
    let front_student = *students.front().unwrap_or(&0);
    let top_sandwich = *sandwiches.front().unwrap_or(&1);
    let num_stu: usize = students.len();
    if num_stu == 0 {
      return true;
    }
    let same_pref = VecDeque::from(vec![front_student; num_stu]);
    if same_pref == *students {
      if top_sandwich != front_student {
        return true;
      }
    }
    false
  }
}
impl AnotherSolution {
  pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let counts: Vec<i32> = Vec::from(vec![2; 0]);
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
  let students_vec = convert_str_vec_to_i32_vec(students_vec);
  println!("{:?}", students_vec);
  let sandwiches_vec = remove_extra_chars(&parts[sandwiche_index..], sandwiche_str);
  let sandwiches_vec = convert_str_vec_to_i32_vec(sandwiches_vec);
  println!("{:?}", sandwiches_vec);
}

pub fn test002() {
  println!("input students array and sandwiches array like follow:");
  println!("students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]");
  // students = [1,1,1,0,1], sandwiches = [1,0,0,1,1]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let students_vec = res.0;
  let sandwiches_vec = res.1;
  println!("students_vec = {:?}", students_vec);
  println!("sandwiches_vec = {:?}", sandwiches_vec);
  // check two array size is same
  assert_eq!(students_vec.len(), sandwiches_vec.len());
  println!("array size assertion passed.");
  let mut students_vec_deque: VecDeque<i32> = students_vec.clone().into_iter().collect();
  let mut sandwiches_vec_deque: VecDeque<i32> =
    sandwiches_vec.clone().into_iter().collect();

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
  // students = [0,0,0,1,0], sandwiches = [0,1,1,0,0]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let students_vec = res.0;
  let sandwiches_vec = res.1;
  println!("students_vec = {:?}", students_vec);
  println!("sandwiches_vec = {:?}", sandwiches_vec);

  let mut students_vec_deque: VecDeque<i32> = students_vec.clone().into_iter().collect();
  let mut sandwiches_vec_deque: VecDeque<i32> =
    sandwiches_vec.clone().into_iter().collect();

  loop {
    let input = get_input("> ".to_string());
    if input == "q".to_string() {
      break;
    }
    Solution::confirm_pref(&mut students_vec_deque, &mut sandwiches_vec_deque);
    if Solution::check_all_same_pref(&students_vec_deque, &sandwiches_vec_deque) {
      println!("The remain students cannot eat sandwiches!");
      break;
    }
  }
  println!("program stop");
}

pub fn test004() {
  println!("input students array and sandwiches array like follow:");
  println!("students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]");
  // students = [1,1,1,0,1], sandwiches = [1,0,0,1,1]
  // students = [0,0,0,1,0], sandwiches = [0,1,1,0,0]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  // students = [1,0,1,1,0,0,1,0,1,1,0,1,0,0,0,1,1,0,0,0,0,1,0,1,0,1,1,1,0,1,0,1,0,1,1,0,0,0,0,1,0,1,0,0,1,0,0,0,1,1,1,0,1,1,1,1,0,1,1,0,0,1,0], sandwiches = [0,1,1,0,1,0,1,0,1,1,0,1,0,0,0,0,1,0,0,1,1,1,1,0,0,1,0,1,0,0,1,0,1,1,1,1,1,0,0,1,1,1,0,1,1,1,0,1,0,1,1,0,0,0,0,1,0,0,1,0,1,1,0]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let students_vec = res.0;
  let sandwiches_vec = res.1;
  println!("students_vec = {:?}", students_vec);
  println!("sandwiches_vec = {:?}", sandwiches_vec);

  let remain = Solution::count_students(students_vec, sandwiches_vec);
  println!("unable to eat students = {}", remain);
}

pub fn test005() {
  println!("input students array and sandwiches array like follow:");
  println!("students = [1,1,1,0,0,1], sandwiches = [1,0,0,0,1,1]");
  // students = [1,1,1,0,1], sandwiches = [1,0,0,1,1]
  // students = [0,0,0,1,0], sandwiches = [0,1,1,0,0]
  // students = [1,1,0,0], sandwiches = [0,1,0,1]
  // students = [1,0,1,1,0,0,1,0,1,1,0,1,0,0,0,1,1,0,0,0,0,1,0,1,0,1,1,1,0,1,0,1,0,1,1,0,0,0,0,1,0,1,0,0,1,0,0,0,1,1,1,0,1,1,1,1,0,1,1,0,0,1,0], sandwiches = [0,1,1,0,1,0,1,0,1,1,0,1,0,0,0,0,1,0,0,1,1,1,1,0,0,1,0,1,0,0,1,0,1,1,1,1,1,0,0,1,1,1,0,1,1,1,0,1,0,1,1,0,0,0,0,1,0,0,1,0,1,1,0]
  let input = get_input(String::from("> "));

  let res = parsing_input(input);
  let students_vec = res.0;
  let sandwiches_vec = res.1;
  println!("students_vec = {:?}", students_vec);
  println!("sandwiches_vec = {:?}", sandwiches_vec);
}

fn parsing_input(input: String) -> (Vec<i32>, Vec<i32>) {
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
  let students_vec = convert_str_vec_to_i32_vec(students_vec);
  let sandwiches_vec = remove_extra_chars(&parts[sandwiche_index..], sandwiche_str);
  let sandwiches_vec = convert_str_vec_to_i32_vec(sandwiches_vec);
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

fn convert_str_vec_to_i32_vec(input: Vec<String>) -> Vec<i32> {
  let mut output: Vec<i32> = Vec::new();
  for str in input {
    output.push(str.as_str().parse().unwrap());
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
