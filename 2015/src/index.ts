import { Command } from 'commander';
import { day1_part1, day1_part2 } from './day1';
import { day2_part1, day2_part2 } from './day2';
import * as fs from 'fs';

function main() {
  const program = new Command();
  program.description('AoC 2015 implementation').option('-d, --day <day>, day to run').parse(process.argv);

  const options = program.opts();

  if (options.day == 'day1') {
    const input = fs.readFileSync('./src/input/day1.txt', 'utf-8');
    day1_part1(input);
    day1_part2(input);
  }

  if (options.day == 'day2') {
    const input = fs.readFileSync('./src/input/day2.txt', 'utf-8');
    day2_part1(input);
    day2_part2(input);
  }
}

main();
