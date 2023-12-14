
/**
 * part1
 */
export function day1_part1(input: String): number {
  let counter = 0;
  [...input].filter(c => c == '(' || c == ')').forEach(c => counter = c === '(' ? counter + 1 : counter - 1);
  console.log(`day 1 - part 1 - ${counter}`);
  return counter;
}

/**
 * part2
 * to enter the basement -> counter == -1
 */
export function day1_part2(input: String): number {
  let counter = 0;
  let position = 0;
  [...input].filter(c => c == '(' || c == ')').forEach((c, i) => {
    counter = c === '(' ? counter + 1 : counter - 1
    if (counter == -1 && position == 0) {
      position = i + 1;
    }
  });
  console.log(`day 1 - part 2 - ${position}`);
  return position;
}
