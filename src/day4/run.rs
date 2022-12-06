use std::fs;

pub(crate) fn run() {
  let contents = fs::read_to_string("./src/day4/input.txt")
    .expect("Should have been able to read the file");

  let mut number_contained = 0;
  let mut number_overlapped = 0;
  for line in contents.lines() {
    if line.len() > 1 {
      let (first, second) = line.split_once(",").unwrap();
      // Split by - and convert to integer
      let first_bounds = first.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      let second_bounds = second.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
      /* Part 1 */
      // Check if second bounds is contained in first bounds
      if first_bounds[0] <= second_bounds[0] && first_bounds[1] >= second_bounds[1] {
        number_contained = number_contained + 1;
        number_overlapped = number_overlapped + 1;
      }
      // Check if first bounds is contained in second bounds
      else if second_bounds[0] <= first_bounds[0] && second_bounds[1] >= first_bounds[1] {
        number_contained = number_contained + 1;
        number_overlapped = number_overlapped + 1;
      }
      /* Part 2 */
      // Check if first range contains either second lower bound or second upper bound
      else if first_bounds[0] <= second_bounds[0] && first_bounds[1] >= second_bounds[0] {
        number_overlapped = number_overlapped + 1;
      }
      else if first_bounds[0] <= second_bounds[1] && first_bounds[1] >= second_bounds[1] {
        number_overlapped = number_overlapped + 1;
      } 
    }
  }

  println!("Part 1 Score: {}", number_contained);
  println!("Part 2 Score: {}", number_overlapped);
}