use std::fs;

struct Monkey {
  id: u32,
  items: Vec<i128>,
  divisible_by: i128,
  operation: Box<dyn Fn(i128, i128) -> i128 + 'static>,
  op_value: Option<i128>,
  true_monkey: u32,
  false_monkey: u32
}

impl Monkey {
  fn new(id: u32, items: Vec<i128>, divisible_by: i128, operation: Box<dyn Fn(i128, i128) -> i128 + 'static>, op_value: Option<i128>, true_monkey: u32, false_monkey: u32) -> Monkey {
    Monkey {
      id,
      items,
      divisible_by,
      operation,
      op_value,
      true_monkey,
      false_monkey
    }
  }

  pub fn operate(&self, val: i128, divide_by_three: bool) -> i128 {
    let mval = self.op_value.unwrap_or(val);
    let mut worry = (self.operation)(val, mval);
    if divide_by_three {
      worry = worry / 3 as i128;
    }
    worry
  }

  pub fn to_string(&self) -> String {
    format!("Monkey ID: {}, Disivible By: {}, Op Value: {:?}, True Monkey: {}, False Monkey: {}, Items: {:?}", self.id, self.divisible_by, self.op_value, self.true_monkey, self.false_monkey, self.items)
  }
}

pub(crate) fn run() {
  println!("Part 1 20 Rounds:");
  run_monkey_simulation(true, 20);
  println!("Part 2 10000 Rounds:");
  run_monkey_simulation(false, 10000);
}

fn run_monkey_simulation(divide_by_three: bool, rounds: u32) {
  let contents = fs::read_to_string("./src/day11/input.txt")
    .expect("Should have been able to read the file");

  // Load Monkey Data
  let mut monkeys: Vec<Monkey> = vec![];
  let mut monkey_inspections = Vec::with_capacity(8);
  for _ in 0..8 {
    monkey_inspections.push(0);
  }
  let mut lines = contents.lines();
  while let Some(line) = lines.next() {
    // Build each monkey  struct
    // Monkey ID
    let monkey_id = line.chars().nth(7).unwrap().to_digit(10).unwrap();
    // Items
    let items_raw = lines.next().unwrap().split(":").collect::<Vec<&str>>()[1];
    let items: Vec<i128> = items_raw.split(",").map(|x| x.trim().parse::<i128>().unwrap()).collect();
    // Boxed Operation
    let op_raw = lines.next().unwrap().split_at(23).1.trim();
    let op_symbol = op_raw.chars().nth(0).unwrap();
    let op_value = op_raw.split_at(1).1.trim().parse::<i128>().ok();
    let monkey_op: Box<dyn Fn(i128, i128) -> i128 + 'static> = match op_symbol {
      '+' => Box::new(|x, y| x + y),
      '-' => Box::new(|x, y| x - y),
      '*' => Box::new(|x, y| x * y),
      '/' => Box::new(|x, y| x / y),
      _ => panic!("Invalid operation"),
    };
    // Divisibility Test
    let divisible_by = lines.next().unwrap().split_at(21).1.trim().parse::<i128>().unwrap();
    // Throw to true / false monkey
    let true_monkey = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap();
    let false_monkey = lines.next().unwrap().chars().last().unwrap().to_digit(10).unwrap();
    // Struct
    let monkey = Monkey::new(monkey_id, items, divisible_by, monkey_op, op_value, true_monkey, false_monkey);
    println!("{}", monkey.to_string());
    monkeys.push(monkey);
    lines.next().unwrap_or_default();
  }

  // Get final divisor
  let final_divisor = monkeys.iter().fold(1, |prev, cur| prev * cur.divisible_by);

  // Execution
  println!("Execution");
  for _ in 0..rounds {
    for i in 0..monkeys.len() {
      let items_clone =  monkeys[i].items.clone();
      monkey_inspections[i] += items_clone.len();
      for item_idx in 0..items_clone.len() {
        let item = items_clone.get(item_idx).unwrap();
        let result = monkeys[i].operate(*item, divide_by_three) % final_divisor;
        if result % monkeys[i].divisible_by == 0 as i128 {
          // Go to true monkey
          let tm = monkeys[i].true_monkey;
          monkeys[tm as usize].items.push(result);
        } else {
          // Go to false monkey
          let fm = monkeys[i].false_monkey;
          monkeys[fm as usize].items.push(result);
        }
      }
      monkeys[i].items.clear();
    }
  }
  for monkey in monkeys {
    println!("{}", monkey.to_string());
  }
  monkey_inspections.sort();
  monkey_inspections.reverse();
  println!("{:?}", monkey_inspections);
  let business = monkey_inspections[0] as i128 * monkey_inspections[1] as i128;
  println!("Business: {}", business);

}