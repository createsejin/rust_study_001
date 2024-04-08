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

  let students_vec = &parts[0..sandwiche_index];
  println!("{:?}", students_vec);
  let sandwiche_vec = &parts[sandwiche_index..];
  println!("{:?}", sandwiche_vec);
}
