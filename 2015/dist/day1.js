"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.day1_part2 = exports.day1_part1 = void 0;
/**
 * part1
 */
function day1_part1(input) {
    let counter = 0;
    [...input].filter(c => c == '(' || c == ')').forEach(c => counter = c === '(' ? counter + 1 : counter - 1);
    console.log(`day 1 - part 1 - ${counter}`);
    return counter;
}
exports.day1_part1 = day1_part1;
/**
 * part2
 * to enter the basement -> counter == -1
 */
function day1_part2(input) {
    let counter = 0;
    let position = 0;
    [...input].filter(c => c == '(' || c == ')').forEach((c, i) => {
        counter = c === '(' ? counter + 1 : counter - 1;
        if (counter == -1 && position == 0) {
            position = i + 1;
        }
    });
    console.log(`day 1 - part 2 - ${position}`);
    return position;
}
exports.day1_part2 = day1_part2;
//# sourceMappingURL=day1.js.map