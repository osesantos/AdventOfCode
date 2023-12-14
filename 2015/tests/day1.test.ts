import { day1_part1, day1_part2 } from '../src/day1';

describe('day 2', () => {
  test('day 1 part 1', () => {
    expect(day1_part1('(())')).toBe(0);
    expect(day1_part1('()()')).toBe(0);
    expect(day1_part1('(((')).toBe(3);
    expect(day1_part1('(()(()(')).toBe(3);
    expect(day1_part1('))(((((')).toBe(3);
    expect(day1_part1('())')).toBe(-1);
    expect(day1_part1('))(')).toBe(-1);
    expect(day1_part1(')))')).toBe(-3);
    expect(day1_part1(')())())')).toBe(-3);
  });

  test('day 1 part 2', () => {
    expect(day1_part2(')')).toBe(1);
    expect(day1_part2('()())')).toBe(5);
  });
});
