use std::fs;

pub(crate) fn run() {
  part1();
  part2();

}

fn part1() {
  let contents = fs::read_to_string("./src/day5/input.txt")
    .expect("Should have been able to read the file");

  // Init stacks
  let mut stacks: Vec<Vec<char>> = vec![];
  for _ in 0..9 {
      stacks.push(vec![]);
  }

  let mut started_moving = false;
  for line in contents.lines() {
    if line.starts_with("move") {
      // Move instruction
      if !started_moving {
        // Reverse the stacks so they are in the correct order to pop
        for n in 0..9 {
          stacks[n].reverse();
        }
        started_moving = true;
      }
      
      let words = line.split(" ").collect::<Vec<&str>>();
      let num_to_move = words[1].parse::<u32>().unwrap();
      let source = words[3].parse::<u32>().unwrap() - 1;
      let dest = words[5].parse::<u32>().unwrap() - 1;

      // Move the top num_to_move disks from source to dest
      for _ in 0..num_to_move {
        let item = stacks[source as usize].pop().unwrap();
        stacks[dest as usize].push(item);
      }
    } else {
      // Starting stack instruction
      for n in 0..9 {
        let val = line.chars().nth((n * 4) + 1).unwrap_or_default();
        if val.is_alphabetic() {
          stacks[n].push(val);
        }
      }
    }

  }

  println!("Part 1");
  for n in 0..9 {
    println!("Stack {}: {:?}", n + 1, stacks[n]);
  }
}

fn part2() {
  let contents = fs::read_to_string("./src/day5/input.txt")
  .expect("Should have been able to read the file");

  // Init stacks
  let mut stacks: Vec<Vec<char>> = vec![];
  for _ in 0..9 {
      stacks.push(vec![]);
  }

  let mut started_moving = false;
  for line in contents.lines() {
    if line.starts_with("move") {
      // Move instruction
      if !started_moving {
        // Reverse the stacks so they are in the correct order to pop
        for n in 0..9 {
          stacks[n].reverse();
        }
        started_moving = true;
      }
      
      let words = line.split(" ").collect::<Vec<&str>>();
      let num_to_move = words[1].parse::<u32>().unwrap();
      let source = words[3].parse::<u32>().unwrap() - 1;
      let dest = words[5].parse::<u32>().unwrap() - 1;

      // Move the top num_to_move disks from source to dest
      let mut moved_disks: Vec<char> = vec![];
      for _ in 0..num_to_move {
        let item = stacks[source as usize].pop().unwrap();
        moved_disks.push(item);
      }
      for item in moved_disks.iter().rev() {
        stacks[dest as usize].push(*item);
      }
    } else {
      // Starting stack instruction
      for n in 0..9 {
        let val = line.chars().nth((n * 4) + 1).unwrap_or_default();
        if val.is_alphabetic() {
          stacks[n].push(val);
        }
      }
    }

  }

  println!("Part 2");
  for n in 0..9 {
    println!("Stack {}: {:?}", n + 1, stacks[n]);
  }
}