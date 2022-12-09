use std::{fs};

pub(crate) fn run() {
  part1();
}

fn part1() {
  let contents = fs::read_to_string("./src/day8/input.txt")
    .expect("Should have been able to read the file");
  
  let rows = contents.lines().collect::<Vec<&str>>();
  let rows_len = rows.len();
  let mut cols: Vec<String> = vec![String::new(); rows.get(0).unwrap().len()];
  for i in 0..rows.get(0).unwrap().len() {
    let mut cur_col = String::new();
    for j in 0..rows_len {
      cur_col.push(rows[j].chars().nth(i).unwrap());
    }
    cols[i] = cur_col;
  }
  let cols_len = rows.len();
  println!("Rows: {}", rows_len);
  println!("Cols: {}", cols_len);

  let mut total_visible = (rows.len() * 2) + (cols.len() * 2) - 4;
  println!("Total outer visible: {}", total_visible);
  let mut checked = 0;
  let mut largest_view = 0;
  for i in 1..(rows_len - 1) {
    for j in 1..(cols_len - 1) {
      checked = checked + 1;
      let cur_row = rows[i];
      let cur_col = cols.get(j).unwrap().as_str();
      let cur_tree = cur_row.chars().nth(j).unwrap();
      let (left, right) = cur_row.split_at(j);
      let (up, down) = cur_col.split_at(i);
      // Check if visible in row or column
      let view_score = score_view(cur_tree, left.chars().rev().collect::<String>().as_str())
        * score_view(cur_tree, &right[1..])
        * score_view(cur_tree, up.chars().rev().collect::<String>().as_str())
        * score_view(cur_tree, &down[1..]);
      if view_score > largest_view {
        largest_view = view_score;
      }
      let mut visible = check_if_largest(cur_tree, left);
      if !visible {
        visible = check_if_largest(cur_tree, &right[1..]);
      }
      if !visible {
        visible = check_if_largest(cur_tree, up);
      }
      if !visible {
        visible = check_if_largest(cur_tree, &down[1..]);
      }
      if visible {
        total_visible += 1;
      }
    }
  }
  println!("Total checked: {}", checked);
  println!("Total visible trees: {}", total_visible);
  println!("Largest view: {}", largest_view)
}

fn check_if_largest(c: char, s: &str) -> bool {
  let mut largest = true;
  for i in s.chars() {
    if i >= c {
      largest = false;
    }
  }
  largest
}

fn score_view(c: char, s: &str) -> i32 {
  let mut score = 0;
  for i in s.chars() {
    if i >= c {
      score += 1;
      break;
    } else {
      score += 1;
    }
  }
  score
}