use std::fs;

pub(crate) fn run() {
  let contents = fs::read_to_string("./src/day3/input.txt")
    .expect("Should have been able to read the file");

  let packs = contents.lines();
  let scores = packs.to_owned().map(|pack| {
    let (first, second) = pack.split_at(pack.len() / 2);
    let mut score = 0;
    for (_idx, char) in first.chars().enumerate() {
      if second.contains(char) {
        let ascii = char as u32;
        let adjusted = if ascii > 90 {
          ascii - 96
        } else {
          ascii - 38
        };
        score = adjusted;
        break;
      }
    }
    score
  });
  let score: u32 = scores.sum();
  println!("Day 1 Total: {}", score);

  let mut group: Vec<&str> = Vec::new();
  let mut sum_day2 = 0;
  for pack in packs {
    if group.len() == 2 {
      for (_idx, char) in pack.chars().enumerate() {
        if group[0].contains(char) && group[1].contains(char) {
          let ascii = char as u32;
          let adjusted = if ascii > 90 {
            ascii - 96
          } else {
            ascii - 38
          };
          sum_day2 = sum_day2 + adjusted;
          break;
        }
      }
      group.clear();
    } else {
      group.push(pack);
    }
  }
  println!("Day 2 Total: {}", sum_day2);

}