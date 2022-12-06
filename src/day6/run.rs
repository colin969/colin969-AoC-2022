use std::fs;

pub struct CircularBuffer<T> {
  buffer: Vec<T>,
  capacity: usize,
  size: usize,
}

impl<T: Default + Clone> CircularBuffer<T> {
  pub fn new(capacity: usize) -> Self {
    let res = Self {
      buffer: vec![T::default(); capacity],
      capacity,
      size: 0,
    };
    res
  }

  pub fn add(&mut self, val: T) {
    // Shift all elements 1 forwards
    if self.size > 1 {
      for n in 1..self.size {
        self.buffer[n - 1] = self.buffer[n].clone();
      }
    }
    // Insert new element
    let size = if self.size == self.capacity { self.size } else { self.size + 1 };
    self.buffer[size - 1] = val;
    self.size = size;
  }
}

pub(crate) fn run() {
  part1();
  part2();
}

fn part1() {
  let contents = fs::read_to_string("./src/day6/input.txt")
    .expect("Should have been able to read the file");

  let mut buf = CircularBuffer::<char>::new(4);
  let mut count = 0;
  for c in contents.chars() {
    count = count + 1;
    buf.add(c);
    // Check if any 2 elements are the same in buf.buffer
    if buf.size == buf.capacity {
      let mut matching = true;
      for i in 0..(buf.size) {
        for n in 0..(buf.size) {
          if buf.buffer[n] == buf.buffer[i] && n != i {
            matching = false;
            break;
          }
        }
      }
      if matching {
        println!("Found match at {}", count);
        break;
      }
    }
  }
}

fn part2() {
  let contents = fs::read_to_string("./src/day6/input.txt")
    .expect("Should have been able to read the file");

  let mut buf = CircularBuffer::<char>::new(14);
  let mut count = 0;
  for c in contents.chars() {
    count = count + 1;
    buf.add(c);
    // Check if any 2 elements are the same in buf.buffer
    if buf.size == buf.capacity {
      let mut matching = true;
      for i in 0..(buf.size) {
        for n in 0..(buf.size) {
          if buf.buffer[n] == buf.buffer[i] && n != i {
            matching = false;
            break;
          }
        }
      }
      if matching {
        println!("Found match at {}", count);
        break;
      }
    }
  }
}