"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.day2_part2 = exports.day2_part1 = void 0;
/**
 * part1
 */
function day2_part1(input) {
    function area(l, h, w) {
        return (2 * l * w) + (2 * l * h) + (2 * w * h);
    }
    function smalest_area(l, h, w) {
        const areas = [l * w, l * h, w * h];
        return areas.reduce((a, b) => Math.min(a, b));
    }
    const sum = input.split('\n').filter(l => l).map(line => {
        console.log(line);
        const dimensions = line.split('x');
        const l = Number(dimensions[0]);
        const w = Number(dimensions[1]);
        const h = Number(dimensions[2]);
        console.log(area(l, h, w) + smalest_area(l, h, w));
        return area(l, h, w) + smalest_area(l, h, w);
    }).reduce((a, b) => a + b);
    console.log(`day 2 - part 1 - ${sum}`);
    return sum;
}
exports.day2_part1 = day2_part1;
/**
 * part2
 * to enter the basement -> counter == -1
 */
function day2_part2(input) {
    console.log(`day 1 - part 2 - ${0}`);
    return 0;
}
exports.day2_part2 = day2_part2;
//# sourceMappingURL=day2.js.map