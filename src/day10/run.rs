use std::{fs, collections::VecDeque};

pub(crate) fn run() {
  part1();
}

fn part1() {
  let contents = fs::read_to_string("./src/day10/input.txt")
    .expect("Should have been able to read the file");

  let mut cycle = 0;
  let mut register = 1;
  let mut score = 0;
  let mut exec_queue = VecDeque::with_capacity(3);
  let mut renderer_chars: Vec<char> = vec![];
  for line in contents.lines() {
    cycle += 1;
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
      score += register * cycle;
    }
    renderer_chars.push(char_visible(register, cycle));
    while !exec_queue.is_empty() {
      let val = exec_queue.pop_front().unwrap();
      println!("Cycle: {}, Register: {}, Instruction: pop {}", cycle, register, val);
      cycle += 1;
      register += val;
      if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        score += register * cycle;
      }
      renderer_chars.push(char_visible(register, cycle));
    }
    println!("Cycle: {}, Register: {}, Instruction: {}", cycle, register, line);
    if line.starts_with("addx") { 
      let num_to_add = line.split_at(4).1.trim().parse::<i32>().unwrap();
      exec_queue.push_back(num_to_add);
    }
  }
  println!("Part 1: {}", score);
  println!("Part 2: CRT Render");
  // Render CRT
  let mut pos = 0;
  for char in renderer_chars {
    print!("{}", char);
    pos += 1;
    if pos >= 40 {
      println!("");
      pos = 0;
    }
  }
}

fn char_visible(register: i32, cycle: i32) -> char {
  let pos = (cycle % 40) - 1;
  if pos >= register - 1 && pos <= register + 1 {
    '#'
  } else {
    '.'
  }
}