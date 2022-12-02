import fs from 'fs';

async function run() {
  const input = fs.readFileSync('./day1/input.txt', { encoding: 'utf8' });
  
  const splits = input.split(/\r?\n/);
  console.log(splits);

  const blocks = [0];
  let block = 0;
  for (const value of splits) {
    if (value === '') {
      block += 1;
      blocks.push(0);
    } else {
      blocks[block] += parseInt(value);
    }
  }
  console.log(blocks.sort().reverse().map((val, idx) => {
    return `Elf ${idx + 1}: Value: ${val}`;
  }));
}

run();