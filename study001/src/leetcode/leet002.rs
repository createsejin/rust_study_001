// struct Solution;
//
// impl Solution {
//   pub fn count_students() {}
// }

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
