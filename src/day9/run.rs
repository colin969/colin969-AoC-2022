use std::{fs, collections::HashSet};

pub(crate) fn run() {
  let tail_visits_1 = compute_tail_visits(2);
  let tail_visits_2 = compute_tail_visits(10);
  println!("Tail Visits 1: {}", tail_visits_1);
  println!("Tail Visits 2: {}", tail_visits_2);
}

fn compute_tail_visits(total_knots: usize) -> i32 {
  let contents = fs::read_to_string("./src/day9/input.txt")
    .expect("Should have been able to read the file");

  let mut rope: Vec<[i32; 2]> = Vec::with_capacity(total_knots);
  for _ in 0..total_knots {
    rope.push([0,0]);
  }
  let mut tail_visits = HashSet::new();

  for line in contents.lines() {
    let (dir, length) = line.split_at(1);
    let translation = match dir {
      "U" => [0, 1],
      "D" => [0, -1],
      "L" => [-1, 0],
      "R" => [1, 0],
      _ => panic!("Invalid direction"),
    };
    for _ in 0..length.trim().parse::<i32>().unwrap() {
      rope[0] = [rope[0][0] + translation[0], rope[0][1] + translation[1]];
      for i in 0..total_knots-1 {
        rope[i+1] = follow_knot(rope[i+1], rope[i]);
      }
      tail_visits.insert(rope[total_knots-1]);
    }
  }

  tail_visits.len() as i32
}

fn follow_knot(mut tail: [i32; 2],  head: [i32; 2]) -> [i32; 2] {
  let distance = [head[0] - tail[0], head[1] - tail[1]];
  if distance[0].abs() > 1 {
    tail[0] += distance[0].signum();
    if distance[1].abs() <= 1 {
      tail[1] = head[1];
    }
  }
  if distance[1].abs() > 1 {
    tail[1] += distance[1].signum();
    if distance[0].abs() <= 1 {
      tail[0] = head[0];
    }
  }
  tail
}