"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const day1_1 = require("../src/day1");
describe('day1', () => {
    test('day 1 part 1', () => {
        expect((0, day1_1.day1_part1)('(())')).toBe(0);
        expect((0, day1_1.day1_part1)('()()')).toBe(0);
        expect((0, day1_1.day1_part1)('(((')).toBe(3);
        expect((0, day1_1.day1_part1)('(()(()(')).toBe(3);
        expect((0, day1_1.day1_part1)('))(((((')).toBe(3);
        expect((0, day1_1.day1_part1)('())')).toBe(-1);
        expect((0, day1_1.day1_part1)('))(')).toBe(-1);
        expect((0, day1_1.day1_part1)(')))')).toBe(-3);
        expect((0, day1_1.day1_part1)(')())())')).toBe(-3);
    });
    test('day 1 part 2', () => {
    });
});
//# sourceMappingURL=day1.test.js.map